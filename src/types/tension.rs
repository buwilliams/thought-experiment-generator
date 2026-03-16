use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TensionType {
    Paradox,
    Gap,
    Contradiction,
    OpenQuestion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedTension {
    pub tension: String,
    pub objects_involved: Vec<String>,
    pub relationships_involved: Vec<String>,
    pub tension_type: TensionType,
}
