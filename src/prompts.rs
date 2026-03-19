use anyhow::{Context, Result};

use crate::types::Conjecture;

const PROMPTS_DIR: &str = "data/prompts";

pub struct Prompt {
    pub system: String,
    pub user: String,
}

pub struct PromptTemplates {
    mind_system_header: String,
    mind_system_item: String,
    mind_system_empty: String,
    generate_output: ParsedTemplate,
    comparative_evaluate: ParsedTemplate,
    summarize_output: ParsedTemplate,
    promote_conjecture: ParsedTemplate,
    deduplicate: ParsedTemplate,
    conjecture_summary: ParsedTemplate,
    ask: ParsedTemplate,
    ask_consolidate: ParsedTemplate,
    novelty_check: ParsedTemplate,
    review_assess: ParsedTemplate,
}

struct ParsedTemplate {
    system: String,
    user: String,
}

impl PromptTemplates {
    pub fn load() -> Result<Self> {
        let mind_sys = load_and_parse(PROMPTS_DIR, "mind_system.md")?;
        Ok(Self {
            mind_system_header: mind_sys.get_section("Header"),
            mind_system_item: mind_sys.get_section("Item"),
            mind_system_empty: mind_sys.get_section("Empty"),
            generate_output: load_and_parse(PROMPTS_DIR, "generate_output.md")?.into_system_user()?,
            comparative_evaluate: load_and_parse(PROMPTS_DIR, "comparative_evaluate.md")?.into_system_user()?,
            summarize_output: load_and_parse(PROMPTS_DIR, "summarize_output.md")?.into_system_user()?,
            promote_conjecture: load_and_parse(PROMPTS_DIR, "promote_conjecture.md")?.into_system_user()?,
            deduplicate: load_and_parse(PROMPTS_DIR, "deduplicate.md")?.into_system_user()?,
            conjecture_summary: load_and_parse(PROMPTS_DIR, "conjecture_summary.md")?.into_system_user()?,
            ask: load_and_parse(PROMPTS_DIR, "ask.md")?.into_system_user()?,
            ask_consolidate: load_and_parse(PROMPTS_DIR, "ask_consolidate.md")?.into_system_user()?,
            novelty_check: load_and_parse(PROMPTS_DIR, "novelty_check.md")?.into_system_user()?,
            review_assess: load_and_parse(PROMPTS_DIR, "review_assess.md")?.into_system_user()?,
        })
    }

    pub fn format_mind_system(&self, mind: &[Conjecture]) -> String {
        if mind.is_empty() {
            return self.mind_system_empty.clone();
        }
        let mut s = self.mind_system_header.clone();
        s.push_str("\n\n");
        for (i, c) in mind.iter().enumerate() {
            let item = apply(&self.mind_system_item, &[
                ("index", &(i + 1).to_string()),
                ("title", &c.title),
                ("summary", &c.summary),
            ]);
            s.push_str(&item);
            s.push('\n');
        }
        s.trim_end().to_string()
    }

    pub fn generate_output(&self, mind_system: &str, conjecture_summary: &str, problemset_context: &str, problem_summary: &str) -> Prompt {
        self.generate_output.apply(&[
            ("mind_system", mind_system),
            ("conjecture_summary", conjecture_summary),
            ("problemset_context", problemset_context),
            ("problem_summary", problem_summary),
        ])
    }

    /// Evaluate all outputs for one lens comparatively in a single call.
    /// outputs: Vec<(problem_id, problem_summary, output_text)>
    pub fn comparative_evaluate(
        &self,
        mind_system: &str,
        lens_summary: &str,
        outputs: &[(String, String, String)],
    ) -> Prompt {
        let formatted = outputs
            .iter()
            .map(|(id, summary, text)| {
                format!("--- Problem: {} ---\nSummary: {}\n\n{}", id, summary, text)
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        self.comparative_evaluate.apply(&[
            ("mind_system", mind_system),
            ("lens_summary", lens_summary),
            ("formatted_outputs", &formatted),
        ])
    }

    pub fn summarize_generated(&self, generated: &str, score: f64) -> Prompt {
        self.summarize_output.apply(&[
            ("score", &format!("{score:.2}")),
            ("generated", generated),
        ])
    }

    pub fn promote_generated(&self, mind_system: &str, generated: &str, score: f64) -> Prompt {
        self.promote_conjecture.apply(&[
            ("mind_system", mind_system),
            ("generated", generated),
            ("score", &format!("{score:.2}")),
        ])
    }

    pub fn deduplicate_problems(&self, mind_system: &str, problems: &[(String, String)]) -> Prompt {
        let formatted = problems
            .iter()
            .map(|(id, summary)| format!("- {id}: {summary}"))
            .collect::<Vec<_>>()
            .join("\n");
        self.deduplicate.apply(&[
            ("mind_system", mind_system),
            ("formatted_problems", &formatted),
        ])
    }

    pub fn ask(&self, mind_system: &str, conjecture_summary: &str, question: &str) -> Prompt {
        self.ask.apply(&[
            ("mind_system", mind_system),
            ("conjecture_summary", conjecture_summary),
            ("question", question),
        ])
    }

    pub fn review_assess(
        &self,
        mind_system: &str,
        mind_full: &str,
        top_outputs: &str,
        trajectory: &str,
    ) -> Prompt {
        self.review_assess.apply(&[
            ("mind_system", mind_system),
            ("mind_full", mind_full),
            ("top_outputs", top_outputs),
            ("trajectory", trajectory),
        ])
    }

    pub fn novelty_check(&self, mind_system: &str, title: &str, conjecture: &str) -> Prompt {
        self.novelty_check.apply(&[
            ("mind_system", mind_system),
            ("title", title),
            ("conjecture", conjecture),
        ])
    }

    pub fn ask_consolidate(&self, mind_system: &str, question: &str, perspectives: &str) -> Prompt {
        self.ask_consolidate.apply(&[
            ("mind_system", mind_system),
            ("question", question),
            ("perspectives", perspectives),
        ])
    }

    pub fn conjecture_summary(&self, mind_system: &str, title: &str, full_text: &str) -> Prompt {
        self.conjecture_summary.apply(&[
            ("mind_system", mind_system),
            ("title", title),
            ("full_text", full_text),
        ])
    }
}

// --- Internals ---

struct RawTemplate {
    content: String,
}

impl RawTemplate {
    fn get_section(&self, name: &str) -> String {
        let header = format!("## {}", name);
        let start = match self.content.find(&header) {
            Some(i) => i + header.len(),
            None => return String::new(),
        };
        let after = &self.content[start..];
        let end = after.find("\n## ").unwrap_or(after.len());
        after[..end].trim().to_string()
    }

    fn into_system_user(self) -> Result<ParsedTemplate> {
        let system = self.get_section("System");
        let user = self.get_section("User");
        if user.is_empty() {
            anyhow::bail!("Template missing ## User section");
        }
        Ok(ParsedTemplate { system, user })
    }
}

impl ParsedTemplate {
    fn apply(&self, vars: &[(&str, &str)]) -> Prompt {
        Prompt {
            system: apply(&self.system, vars),
            user: apply(&self.user, vars),
        }
    }
}

fn load_and_parse(dir: &str, filename: &str) -> Result<RawTemplate> {
    let path = std::path::Path::new(dir).join(filename);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Could not load prompt template: {}", path.display()))?;
    Ok(RawTemplate { content })
}

fn apply(template: &str, vars: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, val) in vars {
        result = result.replace(&format!("{{{{{}}}}}", key), val);
    }
    result
}
