pub struct Prompt {
    pub system: &'static str,
    pub user: String,
}

pub fn background_generation(topic: &str, count: usize) -> Prompt {
    Prompt {
        system: "Generate sentences as a plain newline-separated list. No numbering, no formatting, no preamble.",
        user: format!(
            "Write {count} different sentences of fundamental learning or explanations about \"{topic}\" as a newline-separated flat list with no formatting."
        ),
    }
}

pub fn words_to_sentences(lines: &str) -> Prompt {
    Prompt {
        system: "Convert each line of words into one sentence as though it were fundamental knowledge. Return exactly one sentence per input line, in the same order. No numbering, no formatting, no preamble.",
        user: format!(
            "Replace each line as a sentence, as though it were fundamental knowledge, using the words on that line. Lines:\n{lines}"
        ),
    }
}

pub fn te_generation(sentences: &str) -> Prompt {
    Prompt {
        system: "Rewrite the provided sentences as a single thought experiment in 500 words or fewer. Thought experiments must be wild, imaginative, unintuitive, oppose commonly accepted beliefs, or combine ideas in interesting ways. Use only the concepts from the sentences. No preamble.",
        user: format!("Sentences:\n{sentences}"),
    }
}

pub fn criticism(te_text: &str) -> Prompt {
    Prompt {
        system: "Evaluate thought experiments using the principles of fallibilism. Return only valid JSON with no preamble or markdown.",
        user: format!(
            r#"{te_text}

Using the principles of fallibilism decide:
- Reach: Does the thought experiment have reach, score 0.0 - 1.0?
- Reach: Why (in 20 words or less)?
- Novelty: Does the thought experiment contribute new understanding or novelty, score 0.0 - 1.0?
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
