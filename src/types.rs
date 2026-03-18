use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Layer {
    Mind,
    Candidates,
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layer::Mind => write!(f, "mind"),
            Layer::Candidates => write!(f, "candidates"),
        }
    }
}

impl std::str::FromStr for Layer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mind" => Ok(Layer::Mind),
            "candidates" => Ok(Layer::Candidates),
            other => anyhow::bail!("Unknown layer: {other}. Use 'mind' or 'candidates'."),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub run: u32,
    pub score: f64,
    pub problem_id: String,
}

/// A conjecture held in the mind or candidates layer — stable, trusted, scored over runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjectureMeta {
    pub id: String,
    pub layer: Layer,
    pub score: f64,
    pub rank: u32,
    pub run_count: u32,
    pub problem_coverage: Vec<String>,
    pub created_at: String,
    pub promoted_from: Option<String>,
    pub history: Vec<HistoryEntry>,
}

#[derive(Debug, Clone)]
pub struct Conjecture {
    pub meta: ConjectureMeta,
    pub title: String,
    pub summary: String,
    pub full_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProblemSource {
    User,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemMeta {
    pub id: String,
    pub source: ProblemSource,
    pub score: f64,
    pub rank: u32,
    pub run_count: u32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub meta: ProblemMeta,
    pub title: String,
    pub summary: String,
    pub full_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub answer: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateProblem {
    pub text: String,
    pub score: f64,
}

/// A generated output — ephemeral conjecture produced each run, scored but not yet trusted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedMeta {
    pub problem_id: String,
    pub conjecture_id: String,
    pub run: u32,
    pub logical_consistency: f64,
    pub hard_to_vary: f64,
    pub total: f64,
    pub candidate_problems: Vec<CandidateProblem>,
}

#[derive(Debug, Clone)]
pub struct Generated {
    pub meta: GeneratedMeta,
    pub text: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateInfo {
    pub run: u32,
    pub created_at: String,
    pub last_run_at: String,
}

// --- Problem Sets ---

pub const PROBLEMSET_MAX_SIZE: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSetMeta {
    pub id: String,
    pub problems: Vec<Problem>,
    pub run_count: u32,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct ProblemSet {
    pub meta: ProblemSetMeta,
    pub content: String,
}

// --- LLM response types ---

#[derive(Debug, Deserialize)]
pub struct ConsistencyResponse {
    pub score: f64,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct QuestionsResponse {
    pub questions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnswerEntry {
    pub question: String,
    pub answer: bool,
}

#[derive(Debug, Deserialize)]
pub struct AnswersResponse {
    pub answers: Vec<AnswerEntry>,
}

#[derive(Debug, Deserialize)]
pub struct CandidatesResponse {
    pub candidates: Vec<CandidateProblem>,
}

/// Response when promoting a generated output into a reusable candidate conjecture.
#[derive(Debug, Deserialize)]
pub struct PromoteResponse {
    pub summary: String,
    pub full_text: String,
}

/// Response when summarizing a conjecture for storage or display.
#[derive(Debug, Deserialize)]
pub struct SummaryResponse {
    pub summary: String,
}

#[derive(Debug, Deserialize)]
pub struct DeduplicateResponse {
    pub remove: Vec<String>,
}
