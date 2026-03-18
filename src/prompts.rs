use crate::types::Tool;

pub struct Prompt {
    pub system: String,
    pub user: String,
}

pub fn format_mind_system(mind_tools: &[Tool]) -> String {
    if mind_tools.is_empty() {
        return String::from("You are a careful, rigorous reasoner.");
    }
    let mut s = String::from("You reason using the following principles and frameworks:\n\n");
    for (i, tool) in mind_tools.iter().enumerate() {
        s.push_str(&format!("{}. {}\n{}\n\n", i + 1, tool.title, tool.summary));
    }
    s.trim_end().to_string()
}

pub fn conjecture_generation(mind_system: &str, tool_summary: &str, problem_summary: &str) -> Prompt {
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "You are reasoning from a specific perspective. Your perspective is:\n{tool_summary}\n\n\
            Apply this perspective to the following problem and generate a conjecture — a structured \
            claim about what is true, what follows, or what is illuminated when this perspective meets \
            this problem. Follow the logic of the collision. Do not invent novelty for its own sake. \
            500 words or fewer.\n\nProblem: {problem_summary}"
        ),
    }
}

pub fn logical_consistency_check(mind_system: &str, conjecture: &str) -> Prompt {
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "Evaluate whether the following conjecture is internally self-consistent — does it \
            contradict itself, rely on incompatible premises, or make claims that cannot simultaneously \
            be true?\n\nReturn JSON: {{\"score\": 0.0, \"reason\": \"...\"}}\n\nConjecture: {conjecture}"
        ),
    }
}

pub fn generate_questions(mind_system: &str, conjecture: &str, problem_summary: &str) -> Prompt {
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "Generate 10 yes/no questions that probe whether the following conjecture is \"hard to \
            vary\" — meaning its parts are load-bearing and cannot be arbitrarily modified without \
            destroying the explanation. Questions must be specific to this conjecture and this problem, \
            not generic.\n\nReturn JSON: {{\"questions\": [\"...\", ...]}}\n\n\
            Conjecture: {conjecture}\n\nProblem: {problem_summary}"
        ),
    }
}

pub fn answer_questions(mind_system: &str, conjecture: &str, questions: &[String]) -> Prompt {
    let formatted = questions
        .iter()
        .enumerate()
        .map(|(i, q)| format!("{}. {}", i + 1, q))
        .collect::<Vec<_>>()
        .join("\n");
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "Answer each of the following yes/no questions about this conjecture.\n\n\
            Return JSON: {{\"answers\": [{{\"question\": \"...\", \"answer\": true}}]}}\n\n\
            Conjecture: {conjecture}\n\nQuestions:\n{formatted}"
        ),
    }
}

pub fn extract_candidate_problems(mind_system: &str, conjecture: &str) -> Prompt {
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "Identify unresolved tensions, unexplored implications, or open questions raised by this \
            conjecture that are worth exploring as new problems. For each candidate, score 0.0–1.0 \
            whether it is worth pursuing.\n\n\
            Return JSON: {{\"candidates\": [{{\"text\": \"...\", \"score\": 0.0}}]}}\n\n\
            Conjecture: {conjecture}"
        ),
    }
}

pub fn summarize_conjecture(conjecture: &str, score: f64) -> Prompt {
    Prompt {
        system: String::from(
            "Summarize in 20 words or fewer. Return only the summary, no preamble.",
        ),
        user: format!("{conjecture}\n\nScore: {score:.2}"),
    }
}

pub fn summarize_for_tool(mind_system: &str, conjecture: &str, score: f64) -> Prompt {
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "Convert the following conjecture into a reusable perspective tool.\n\n\
            Return JSON: {{\"summary\": \"...\", \"full_text\": \"...\"}}\n\n\
            The summary must be 1-2 sentences suitable for use in LLM prompts.\n\
            The full_text must be a readable, standalone description of the perspective this \
            conjecture embodies — what lens it provides, what kinds of problems it is useful for, \
            and what it illuminates. 100-200 words.\n\n\
            Conjecture: {conjecture}\nScore: {score:.2}"
        ),
    }
}

pub fn summarize_tool(mind_system: &str, title: &str, full_text: &str) -> Prompt {
    Prompt {
        system: mind_system.to_string(),
        user: format!(
            "Summarize the following tool into 1-2 sentences suitable for use as context in LLM \
            prompts. The summary should capture the core lens or principle the tool provides.\n\n\
            Return JSON: {{\"summary\": \"...\"}}\n\nTitle: {title}\nFull text: {full_text}"
        ),
    }
}
