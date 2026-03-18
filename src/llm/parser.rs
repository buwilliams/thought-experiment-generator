use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use tracing::warn;

/// Parse a JSON response from an LLM, handling common formatting issues:
/// - Markdown code fences (```json ... ```)
/// - Leading/trailing whitespace
/// - Extracting first JSON object/array from mixed text
/// - Truncated JSON (recovers complete elements at any nesting depth)
/// - Invalid JSON escape sequences (e.g. LaTeX \( \) \[ \mathbf)
pub fn parse_llm_json<T: DeserializeOwned>(raw: &str) -> Result<T> {
    let cleaned = strip_markdown_fences(raw).trim().to_string();

    // Try direct parse first
    if let Ok(v) = serde_json::from_str::<T>(&cleaned) {
        return Ok(v);
    }

    // Sanitize invalid escape sequences (e.g. LaTeX \( \) \[ \]) and retry
    let sanitized = sanitize_json_escapes(&cleaned);
    if sanitized != cleaned {
        if let Ok(v) = serde_json::from_str::<T>(&sanitized) {
            return Ok(v);
        }
    }

    // Try extracting the first JSON block
    if let Some(block) = extract_json_block(&sanitized) {
        if let Ok(v) = serde_json::from_str::<T>(block) {
            return Ok(v);
        }
    }

    // Try recovering truncated JSON (handles bare arrays and wrapped objects)
    if let Some(repaired) = repair_truncated_json(&sanitized) {
        if let Ok(v) = serde_json::from_str::<T>(&repaired) {
            warn!("Recovered truncated JSON ({} chars)", sanitized.len());
            return Ok(v);
        }
    }

    // Final attempt: strip leading non-JSON content
    let trimmed = sanitized.trim_start_matches(|c: char| c != '{' && c != '[');
    if let Some(repaired) = repair_truncated_json(trimmed) {
        if let Ok(v) = serde_json::from_str::<T>(&repaired) {
            return Ok(v);
        }
    }

    serde_json::from_str::<T>(trimmed)
        .with_context(|| format!("Failed to parse LLM JSON. Raw response:\n{raw}"))
}

/// Replace invalid JSON escape sequences (e.g. `\(`, `\[`, `\mathbf`) with
/// their literal counterparts. Only touches backslashes inside JSON strings.
/// Valid JSON escapes (`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX`)
/// are left untouched.
fn sanitize_json_escapes(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 16);
    let mut in_string = false;
    let mut just_escaped = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if just_escaped {
            result.push(c);
            just_escaped = false;
            continue;
        }

        if c == '\\' && in_string {
            match chars.peek() {
                Some(&next)
                    if matches!(next, '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | 'u') =>
                {
                    // Valid JSON escape — pass through, skip the next char via just_escaped
                    result.push('\\');
                    just_escaped = true;
                }
                _ => {
                    // Invalid escape (e.g. LaTeX \( \) \[ \mathbf) — double the backslash
                    result.push('\\');
                    result.push('\\');
                }
            }
        } else if c == '"' {
            result.push(c);
            in_string = !in_string;
        } else {
            result.push(c);
        }
    }

    result
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

/// Attempt to repair truncated JSON by finding the last complete element at
/// the deepest recoverable level and closing all open containers.
///
/// Handles both bare arrays `["a", "b"` and wrapped objects `{"key": ["a", "b"`.
/// Distinguishes object keys from values so a truncated `{"key": 3` is not
/// mistaken for a complete key-value pair.
fn repair_truncated_json(s: &str) -> Option<String> {
    let s = s.trim();
    if s.is_empty() || (!s.starts_with('{') && !s.starts_with('[')) {
        return None;
    }

    #[derive(Clone)]
    enum Kind {
        Array,
        /// `awaiting_value`: true after seeing `:`, meaning the next completed
        /// value (string, object, or array) closes the current KV pair.
        Object { awaiting_value: bool },
    }

    struct Entry {
        kind: Kind,
        /// Byte position immediately after the last complete child element.
        last_safe: Option<usize>,
    }

    let mut stack: Vec<Entry> = Vec::new();
    let mut in_string = false;
    let mut escaped = false;

    for (i, c) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' && in_string {
            escaped = true;
            continue;
        }
        if c == '"' {
            if !in_string {
                in_string = true;
            } else {
                in_string = false;
                let end = i + 1;
                if let Some(top) = stack.last_mut() {
                    match &mut top.kind {
                        Kind::Array => {
                            top.last_safe = Some(end);
                        }
                        Kind::Object { awaiting_value } => {
                            if *awaiting_value {
                                // String was the value of a KV pair — safe
                                top.last_safe = Some(end);
                                *awaiting_value = false;
                            }
                            // else: string was a key — not yet safe
                        }
                    }
                }
            }
            continue;
        }
        if in_string {
            continue;
        }

        match c {
            '{' => stack.push(Entry { kind: Kind::Object { awaiting_value: false }, last_safe: None }),
            '[' => stack.push(Entry { kind: Kind::Array, last_safe: None }),
            ':' => {
                if let Some(top) = stack.last_mut() {
                    if let Kind::Object { awaiting_value } = &mut top.kind {
                        *awaiting_value = true;
                    }
                }
            }
            '}' | ']' => {
                stack.pop();
                if stack.is_empty() {
                    // Parsed completely — no repair needed
                    return None;
                }
                let end = i + c.len_utf8();
                if let Some(parent) = stack.last_mut() {
                    match &mut parent.kind {
                        Kind::Array => {
                            parent.last_safe = Some(end);
                        }
                        Kind::Object { awaiting_value } => {
                            if *awaiting_value {
                                parent.last_safe = Some(end);
                                *awaiting_value = false;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        return None;
    }

    // Find the deepest stack level with a recorded safe position.
    // Only close containers at or above that level; deeper ones are discarded.
    let repair_level = stack.iter().enumerate().rev()
        .find(|(_, e)| e.last_safe.is_some())
        .map(|(idx, _)| idx)?;

    let repair_pos = stack[repair_level].last_safe.unwrap();

    let mut repaired = s[..repair_pos].trim_end().to_string();
    if repaired.ends_with(',') {
        repaired.pop();
    }

    // Close containers from repair_level up to the outermost
    for entry in stack[..=repair_level].iter().rev() {
        let closer = match entry.kind {
            Kind::Array => ']',
            Kind::Object { .. } => '}',
        };
        repaired.push(closer);
    }

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

    #[derive(Debug, Deserialize, PartialEq)]
    struct Questions {
        questions: Vec<String>,
    }

    #[test]
    fn test_truncated_wrapped_array() {
        // The real failure case: {"questions": [...]} with the array truncated
        let input = r#"{"questions": ["first question?", "second question?", "third que"#;
        let result: Questions = parse_llm_json(input).unwrap();
        assert_eq!(result.questions.len(), 2);
        assert_eq!(result.questions[0], "first question?");
        assert_eq!(result.questions[1], "second question?");
    }

    #[test]
    fn test_truncated_wrapped_array_missing_close() {
        // Array complete but outer } missing
        let input = r#"{"questions": ["q1", "q2", "q3"]"#;
        let result: Questions = parse_llm_json(input).unwrap();
        assert_eq!(result.questions.len(), 3);
    }

    #[test]
    fn test_latex_escapes_in_questions() {
        // GPT-5.4 uses LaTeX notation like \(R\) inside JSON strings
        let input = r#"{"questions": ["Does \(R\) hold?", "Is \mathbf{X} valid?"]}"#;
        let result: Questions = parse_llm_json(input).unwrap();
        assert_eq!(result.questions.len(), 2);
        assert!(result.questions[0].contains(r"\(R\)"));
    }

    #[test]
    fn test_valid_escapes_preserved() {
        // Valid JSON escapes must not be mangled
        let input = r#"{"questions": ["Line 1\nLine 2", "Tab\there", "Quote \"here\""]}"#;
        let result: Questions = parse_llm_json(input).unwrap();
        assert_eq!(result.questions[0], "Line 1\nLine 2");
        assert_eq!(result.questions[1], "Tab\there");
        assert_eq!(result.questions[2], "Quote \"here\"");
    }
}
