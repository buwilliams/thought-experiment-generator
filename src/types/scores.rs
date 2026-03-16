use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeutschScore {
    pub hard_to_vary: f64,
    pub reach: f64,
    pub minimal_assumptions: f64,
    pub tension_resolution: f64,
    pub overall_score: f64,
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceResult {
    pub internally_consistent: bool,
    pub consistent_reason: String,
    pub topic_relevant: bool,
    pub relevant_reason: String,
    pub passes: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryScore {
    pub trajectory_score: f64,
    pub depth_of_insight: String,
    pub best_node: Uuid,
    pub justification: String,
}
