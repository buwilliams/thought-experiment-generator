use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::node::Node;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BranchStatus {
    Active,
    Terminated,
    CrossPollinated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub parent_branch_ids: Vec<Uuid>,
    pub topic: String,
    pub nodes: Vec<Node>,
    pub current_depth: u32,
    pub depth_limit: u32,
    pub status: BranchStatus,
    pub trajectory_score: Option<f64>,
}

impl Branch {
    pub fn new(topic: String, depth_limit: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent_branch_ids: Vec::new(),
            topic,
            nodes: Vec::new(),
            current_depth: 0,
            depth_limit,
            status: BranchStatus::Active,
            trajectory_score: None,
        }
    }

    pub fn latest_tension(&self) -> Option<&super::tension::UnresolvedTension> {
        self.nodes.last().and_then(|n| n.unresolved_tension.as_ref())
    }

    pub fn score_history(&self) -> Vec<f64> {
        self.nodes.iter().map(|n| n.deutsch_score.overall_score).collect()
    }
}
