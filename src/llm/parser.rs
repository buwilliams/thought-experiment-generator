use anyhow::{Context, Result};
use serde::de::DeserializeOwned;

/// Parse a JSON response from an LLM, handling common formatting issues:
/// - Markdown code fences (```json ... ```)
/// - Leading/trailing whitespace
/// - Extracting first JSON object/array from mixed text
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

    // Final attempt: be more aggressive about finding JSON
    let trimmed = cleaned.trim_start_matches(|c: char| c != '{' && c != '[');
    serde_json::from_str::<T>(trimmed)
        .with_context(|| format!("Failed to parse LLM JSON. Raw response:\n{raw}"))
}

fn strip_markdown_fences(s: &str) -> &str {
    let s = s.trim();
    if let Some(rest) = s.strip_prefix("```json") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
    }
    if let Some(rest) = s.strip_prefix("```") {
        if let Some(inner) = rest.strip_suffix("```") {
            return inner.trim();
        }
    }
    s
}

fn extract_json_block(s: &str) -> Option<&str> {
    // Find the first { or [ and its matching closing bracket
    let open_brace = s.find('{');
    let open_bracket = s.find('[');

    let (start, open, close) = match (open_brace, open_bracket) {
        (Some(b), Some(a)) if a < b => (a, '[', ']'),
        (Some(b), _) => (b, '{', '}'),
        (None, Some(a)) => (a, '[', ']'),
        (None, None) => return None,
    };

    // Find matching close by counting nesting
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
}
