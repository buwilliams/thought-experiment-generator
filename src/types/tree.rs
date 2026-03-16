use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::branch::Branch;
use super::draw_pool::DrawPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Causal,
    Counterfactual,
    Mechanistic,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedTopic {
    pub core_subject: String,
    pub core_tension: String,
    pub question_type: QuestionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPollinationRecord {
    pub branch_a: Uuid,
    pub branch_b: Uuid,
    pub new_branch: Uuid,
    pub depth_at_merge: u32,
}

#[derive(Debug, Clone)]
pub struct Tree {
    pub id: Uuid,
    pub topic: String,
    pub topic_normalized: NormalizedTopic,
    pub draw_pool: DrawPool,
    pub branches: Vec<Branch>,
    pub cross_pollinations: Vec<CrossPollinationRecord>,
    pub top_trajectories: Vec<Uuid>,
}

impl Tree {
    pub fn new(topic: String, topic_normalized: NormalizedTopic, draw_pool: DrawPool) -> Self {
        Self {
            id: Uuid::new_v4(),
            topic,
            topic_normalized,
            draw_pool,
            branches: Vec::new(),
            cross_pollinations: Vec::new(),
            top_trajectories: Vec::new(),
        }
    }
}
