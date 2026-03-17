use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Critique {
    pub reach: f64,
    pub reach_why: String,
    pub novelty: f64,
    pub novelty_why: String,
    pub falsifiable: f64,
    pub falsifiable_why: String,
    pub explanation: String,
}

impl Critique {
    pub fn total_score(&self) -> f64 {
        self.reach + self.novelty + self.falsifiable
    }
}
