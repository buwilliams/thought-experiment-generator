use rand::prelude::*;
use uuid::Uuid;

use super::quad::Quad;

#[derive(Debug, Clone)]
pub struct PoolRatio {
    pub background: f64,
    pub universal: f64,
    pub novel: f64,
}

impl PoolRatio {
    pub fn initial(bg_ratio: f64) -> Self {
        Self {
            background: bg_ratio,
            universal: 1.0 - bg_ratio,
            novel: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtomConfig {
    pub objects: u32,
    pub relationships: u32,
    pub properties: u32,
}

impl Default for AtomConfig {
    fn default() -> Self {
        Self {
            objects: 4,
            relationships: 3,
            properties: 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawResult {
    pub objects: Vec<String>,
    pub relationships: Vec<String>,
    pub properties: Vec<String>,
    pub quads_used: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct DrawPool {
    pub background: Vec<Quad>,
    pub universal: Vec<Quad>,
    pub novel: Vec<Quad>,
    pub ratio: PoolRatio,
}

impl DrawPool {
    pub fn new(background: Vec<Quad>, universal: Vec<Quad>, bg_ratio: f64) -> Self {
        Self {
            background,
            universal,
            novel: Vec::new(),
            ratio: PoolRatio::initial(bg_ratio),
        }
    }

    /// Recalculate ratio based on novel pool size.
    /// Shifts from 50/50/0 toward 30/30/40 as novel pool grows.
    pub fn update_ratio(&mut self) {
        let total = self.background.len() + self.universal.len() + self.novel.len();
        if total == 0 {
            return;
        }
        let novel_frac = self.novel.len() as f64 / total as f64;
        // Novel ratio caps at 0.4, scales linearly with pool fraction
        let novel_ratio = (novel_frac * 2.0).min(0.4);
        let remaining = 1.0 - novel_ratio;
        self.ratio = PoolRatio {
            background: remaining / 2.0,
            universal: remaining / 2.0,
            novel: novel_ratio,
        };
    }

    /// Add novel quads with provenance tracking.
    pub fn add_novel(&mut self, quads: Vec<Quad>) {
        self.novel.extend(quads);
    }

    /// Sample a draw of objects, relationships, and properties from the three pools.
    /// Half the slots draw from background/novel, half from universal.
    pub fn draw(&self, config: &AtomConfig, rng: &mut impl Rng) -> DrawResult {
        let mut objects = Vec::new();
        let mut relationships = Vec::new();
        let mut properties = Vec::new();
        let mut quads_used = Vec::new();

        // Objects: half from background/novel, half from universal
        let bg_novel_objects = config.objects / 2;
        let universal_objects = config.objects - bg_novel_objects;

        for _ in 0..bg_novel_objects {
            if let Some(q) = self.sample_bg_or_novel(rng) {
                objects.push(q.object_a.clone());
                quads_used.push(q.id);
            }
        }
        for _ in 0..universal_objects {
            if let Some(q) = self.sample_universal(rng) {
                objects.push(q.object_a.clone());
                quads_used.push(q.id);
            }
        }

        // Relationships: split similarly
        let bg_novel_rels = (config.relationships + 1) / 2;
        let universal_rels = config.relationships - bg_novel_rels;

        for _ in 0..bg_novel_rels {
            if let Some(q) = self.sample_bg_or_novel(rng) {
                relationships.push(q.relationship.clone());
                quads_used.push(q.id);
            }
        }
        for _ in 0..universal_rels {
            if let Some(q) = self.sample_universal(rng) {
                relationships.push(q.relationship.clone());
                quads_used.push(q.id);
            }
        }

        // Properties: half and half
        let bg_novel_props = config.properties / 2;
        let universal_props = config.properties - bg_novel_props;

        for _ in 0..bg_novel_props {
            if let Some(q) = self.sample_bg_or_novel(rng) {
                properties.push(q.property.clone());
                quads_used.push(q.id);
            }
        }
        for _ in 0..universal_props {
            if let Some(q) = self.sample_universal(rng) {
                properties.push(q.property.clone());
                quads_used.push(q.id);
            }
        }

        DrawResult {
            objects,
            relationships,
            properties,
            quads_used,
        }
    }

    /// Sample from background or novel pools weighted by ratio.
    fn sample_bg_or_novel(&self, rng: &mut impl Rng) -> Option<&Quad> {
        let bg_weight = self.ratio.background;
        let novel_weight = self.ratio.novel;
        let total = bg_weight + novel_weight;
        if total == 0.0 {
            return None;
        }

        let use_novel = !self.novel.is_empty() && rng.r#gen::<f64>() < (novel_weight / total);

        if use_novel {
            self.novel.choose(rng)
        } else if !self.background.is_empty() {
            self.background.choose(rng)
        } else {
            self.novel.choose(rng)
        }
    }

    fn sample_universal(&self, rng: &mut impl Rng) -> Option<&Quad> {
        self.universal.choose(rng)
    }
}
