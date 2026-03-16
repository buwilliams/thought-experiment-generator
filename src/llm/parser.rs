use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use tracing::warn;

/// Parse a JSON response from an LLM, handling common formatting issues:
/// - Markdown code fences (```json ... ```)
/// - Leading/trailing whitespace
/// - Extracting first JSON object/array from mixed text
/// - Truncated JSON arrays (recovers complete elements)
pub fn parse_llm_json<T: DeserializeOwned>(raw: &str) -> Result<T> {
    let cleaned = strip_markdown_fences(raw).trim().to_string();

    // Try direct parse first
    if let Ok(v) = serde_json::from_str::<T>(&cleaned) {
        return Ok(v);
    }

    // Try extracting the first JSON block
    if let Some(block) = extract_json_block(&cleaned) {
        if let Ok(v) = serde_json::from_str::<T>(block) {
            return Ok(v);
        }
    }

    // Try recovering a truncated JSON array
    if let Some(repaired) = repair_truncated_array(&cleaned) {
        if let Ok(v) = serde_json::from_str::<T>(&repaired) {
            warn!("Recovered truncated JSON array ({} chars repaired)", cleaned.len());
            return Ok(v);
        }
    }

    // Final attempt: be more aggressive about finding JSON
    let trimmed = cleaned.trim_start_matches(|c: char| c != '{' && c != '[');
    if let Some(repaired) = repair_truncated_array(trimmed) {
        if let Ok(v) = serde_json::from_str::<T>(&repaired) {
            return Ok(v);
        }
    }

    serde_json::from_str::<T>(trimmed)
        .with_context(|| format!("Failed to parse LLM JSON. Raw response:\n{raw}"))
}

fn strip_markdown_fences(s: &str) -> &str {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix("```json") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
        // Truncated fence (no closing ```)
        return rest.trim();
    }
    if let Some(rest) = s.strip_prefix("```") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
        return rest.trim();
    }
    s
}

fn extract_json_block(s: &str) -> Option<&str> {
    let open_brace = s.find('{');
    let open_bracket = s.find('[');

    let (start, open, close) = match (open_brace, open_bracket) {
        (Some(b), Some(a)) if a < b => (a, '[', ']'),
        (Some(b), _) => (b, '{', '}'),
        (None, Some(a)) => (a, '[', ']'),
        (None, None) => return None,
    };

    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;

    for (i, c) in s[start..].char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if c == '\\' && in_string {
            escape = true;
            continue;
        }
        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(&s[start..start + i + c.len_utf8()]);
            }
        }
    }
    None
}

/// Attempt to repair a truncated JSON array by finding the last complete
/// top-level element and closing the array.
fn repair_truncated_array(s: &str) -> Option<String> {
    let s = s.trim();
    if !s.starts_with('[') {
        return None;
    }

    // Find positions of all complete top-level objects/values in the array
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;
    let mut last_complete_end = None;

    for (i, c) in s.char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if c == '\\' && in_string {
            escape = true;
            continue;
        }
        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }
        match c {
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth -= 1;
                // depth 1 means we just closed a top-level element inside the array
                if depth == 1 {
                    last_complete_end = Some(i);
                }
                if depth == 0 {
                    // Array is actually complete, no repair needed
                    return None;
                }
            }
            _ => {}
        }
    }

    // Truncate after the last complete element and close the array
    let end = last_complete_end?;
    let mut repaired = s[..=end].to_string();
    // Strip any trailing comma
    let trimmed = repaired.trim_end();
    if trimmed.ends_with(',') {
        repaired = trimmed.trim_end_matches(',').to_string();
    }
    repaired.push(']');
    Some(repaired)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Simple {
        value: i32,
    }

    #[test]
    fn test_direct_json() {
        let result: Simple = parse_llm_json(r#"{"value": 42}"#).unwrap();
        assert_eq!(result, Simple { value: 42 });
    }

    #[test]
    fn test_markdown_fenced() {
        let result: Simple = parse_llm_json("```json\n{\"value\": 42}\n```").unwrap();
        assert_eq!(result, Simple { value: 42 });
    }

    #[test]
    fn test_preamble_text() {
        let result: Simple =
            parse_llm_json("Here is the result:\n{\"value\": 42}").unwrap();
        assert_eq!(result, Simple { value: 42 });
    }

    #[test]
    fn test_array_parse() {
        let result: Vec<Simple> =
            parse_llm_json("[{\"value\": 1}, {\"value\": 2}]").unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_truncated_array() {
        let input = r#"[{"value": 1}, {"value": 2}, {"value": 3"#;
        let result: Vec<Simple> = parse_llm_json(input).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Simple { value: 1 });
        assert_eq!(result[1], Simple { value: 2 });
    }

    #[test]
    fn test_truncated_array_with_trailing_comma() {
        let input = r#"[{"value": 1}, {"value": 2},"#;
        let result: Vec<Simple> = parse_llm_json(input).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_truncated_markdown_fence() {
        let input = "```json\n[{\"value\": 1}, {\"value\": 2}, {\"val";
        let result: Vec<Simple> = parse_llm_json(input).unwrap();
        assert_eq!(result.len(), 2);
    }
}
