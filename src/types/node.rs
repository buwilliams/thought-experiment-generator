use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::scores::DeutschScore;
use super::tension::UnresolvedTension;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    pub depth: u32,
    pub branch_id: Uuid,
    pub thought_experiment: String,
    pub quads_used: Vec<Uuid>,
    pub deutsch_score: DeutschScore,
    pub unresolved_tension: Option<UnresolvedTension>,
    pub score_history: Vec<f64>,
    pub accumulated_path: Vec<Uuid>,
}
