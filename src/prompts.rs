pub struct Prompt {
    pub system: &'static str,
    pub user: String,
}

pub fn background_generation(topic: &str, count: usize) -> Prompt {
    Prompt {
        system: "Generate sentences as a plain newline-separated list. No numbering, no formatting, no preamble.",
        user: format!(
            "Given the following topic context, write {count} sentences as a newline-separated flat list with no formatting. Prefer anomalies, unresolved tensions, edge cases, and open questions over established textbook facts. Draw directly from the specifics of the context provided.\n\nTopic context:\n{topic}"
        ),
    }
}

pub fn words_to_sentences(lines: &str) -> Prompt {
    Prompt {
        system: "Convert each line of words into one sentence. Return exactly one sentence per input line, in the same order. No numbering, no formatting, no preamble.",
        user: format!(
            "Turn each line into a sentence that uses all the words on that line. The sentence does not need to make conventional sense — preserve the strangeness and tension of the word combination. Do not normalize it into a familiar claim. Lines:\n{lines}"
        ),
    }
}

pub fn te_generation(sentences: &str) -> Prompt {
    Prompt {
        system: "You are a reasoner, not a storyteller. Your job is to take the provided sentences as axioms — assume all of them are simultaneously true — and reason out what that implies about the world. Write a thought experiment in 500 words or fewer that follows strictly from those premises. Do not invent novelty. Do not be creative for its own sake. Just follow the logic of the collision. No preamble.",
        user: format!("Premises:\n{sentences}"),
    }
}

pub fn criticism(te_text: &str) -> Prompt {
    Prompt {
        system: "Evaluate thought experiments using the principles of fallibilism. Return only valid JSON with no preamble or markdown.",
        user: format!(
            r#"{te_text}

Using the principles of fallibilism decide:
- Reach: Does the implication of this thought experiment propose something no prior work has proposed — does it break into genuinely new explanatory territory, score 0.0 - 1.0?
- Reach: Why (in 20 words or less)?
- Novelty: Does it reframe or connect existing knowledge in a non-obvious way, score 0.0 - 1.0?
- Novelty: Why (in 20 words or less)?
- Falsifiable: Is the thought experiment falsifiable, score 0.0 - 1.0?
- Falsifiable: Why (in 20 words or less)?
- In less than 20 words, what does it explain or mean?

Return JSON:
{{
    "reach": 0.0,
    "reach_why": "",
    "novelty": 0.0,
    "novelty_why": "",
    "falsifiable": 0.0,
    "falsifiable_why": "",
    "explanation": ""
}}"#
        ),
    }
}

pub fn summarize(te_text: &str, critique_json: &str) -> Prompt {
    Prompt {
        system: "Summarize the following thought experiment and its critique in 20 words or fewer. Return only the summary with no preamble.",
        user: format!("{te_text}\n\n{critique_json}"),
    }
}
