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
    consistency_check: ParsedTemplate,
    generate_questions: ParsedTemplate,
    answer_questions: ParsedTemplate,
    candidate_problems: ParsedTemplate,
    summarize_output: ParsedTemplate,
    promote_conjecture: ParsedTemplate,
    deduplicate: ParsedTemplate,
    conjecture_summary: ParsedTemplate,
    ask: ParsedTemplate,
    ask_consolidate: ParsedTemplate,
    novelty_check: ParsedTemplate,
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
            consistency_check: load_and_parse(PROMPTS_DIR, "consistency_check.md")?.into_system_user()?,
            generate_questions: load_and_parse(PROMPTS_DIR, "generate_questions.md")?.into_system_user()?,
            answer_questions: load_and_parse(PROMPTS_DIR, "answer_questions.md")?.into_system_user()?,
            candidate_problems: load_and_parse(PROMPTS_DIR, "candidate_problems.md")?.into_system_user()?,
            summarize_output: load_and_parse(PROMPTS_DIR, "summarize_output.md")?.into_system_user()?,
            promote_conjecture: load_and_parse(PROMPTS_DIR, "promote_conjecture.md")?.into_system_user()?,
            deduplicate: load_and_parse(PROMPTS_DIR, "deduplicate.md")?.into_system_user()?,
            conjecture_summary: load_and_parse(PROMPTS_DIR, "conjecture_summary.md")?.into_system_user()?,
            ask: load_and_parse(PROMPTS_DIR, "ask.md")?.into_system_user()?,
            ask_consolidate: load_and_parse(PROMPTS_DIR, "ask_consolidate.md")?.into_system_user()?,
            novelty_check: load_and_parse(PROMPTS_DIR, "novelty_check.md")?.into_system_user()?,
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

    pub fn logical_consistency_check(&self, mind_system: &str, generated: &str) -> Prompt {
        self.consistency_check.apply(&[
            ("mind_system", mind_system),
            ("generated", generated),
        ])
    }

    pub fn generate_questions(&self, mind_system: &str, generated: &str, problem_summary: &str) -> Prompt {
        self.generate_questions.apply(&[
            ("mind_system", mind_system),
            ("generated", generated),
            ("problem_summary", problem_summary),
        ])
    }

    pub fn answer_questions(&self, mind_system: &str, generated: &str, questions: &[String]) -> Prompt {
        let formatted = questions
            .iter()
            .enumerate()
            .map(|(i, q)| format!("{}. {}", i + 1, q))
            .collect::<Vec<_>>()
            .join("\n");
        self.answer_questions.apply(&[
            ("mind_system", mind_system),
            ("generated", generated),
            ("formatted_questions", &formatted),
        ])
    }

    pub fn extract_candidate_problems(&self, mind_system: &str, generated: &str) -> Prompt {
        self.candidate_problems.apply(&[
            ("mind_system", mind_system),
            ("generated", generated),
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
