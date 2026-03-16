use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QuadSource {
    Background,
    Universal,
    Novel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quad {
    pub id: Uuid,
    pub object_a: String,
    pub relationship: String,
    pub object_b: String,
    pub property: String,
    pub source: QuadSource,
    /// For novel quads: the node ID that generated them.
    pub provenance: Option<Uuid>,
}

impl Quad {
    pub fn new_background(object_a: String, relationship: String, object_b: String, property: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            object_a,
            relationship,
            object_b,
            property,
            source: QuadSource::Background,
            provenance: None,
        }
    }

    pub fn new_universal_rich(object_a: String, relationship: String, object_b: String, property: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            object_a,
            relationship,
            object_b,
            property,
            source: QuadSource::Universal,
            provenance: None,
        }
    }

    pub fn new_novel(object_a: String, relationship: String, object_b: String, property: String, provenance: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            object_a,
            relationship,
            object_b,
            property,
            source: QuadSource::Novel,
            provenance: Some(provenance),
        }
    }

    /// Key for deduplication.
    pub fn dedup_key(&self) -> (&str, &str, &str) {
        (&self.object_a, &self.relationship, &self.object_b)
    }
}
