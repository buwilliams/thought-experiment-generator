use crate::types::AtomConfig;

#[derive(Debug, Clone)]
pub enum LlmProvider {
    Anthropic,
    /// Claude Max subscription — uses Bearer token auth against api.anthropic.com
    AnthropicToken,
    OpenAi,
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: LlmProvider,
    pub api_key: String,
    pub model: String,
    pub max_concurrent: usize,
    pub low_temperature: f64,
    pub high_temperature: f64,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub depth_limit: u32,
    pub num_branches: u32,
    pub survivor_threshold: f64,
    pub novel_threshold: f64,
    pub bg_universal_ratio: f64,
    pub draws_per_depth: u32,
    pub temperature: f64,
    pub atom: AtomConfig,
    pub llm: LlmConfig,
}

impl Config {
    pub fn new(
        provider_str: &str,
        model: &str,
        depth: u32,
        branches: u32,
        threshold: f64,
        novel_threshold: f64,
        ratio: f64,
        draws: u32,
        temperature: f64,
        objects: u32,
        relationships: u32,
        properties: u32,
        max_concurrent: usize,
    ) -> anyhow::Result<Self> {
        let provider = match provider_str {
            "anthropic" => LlmProvider::Anthropic,
            "anthropic-token" => LlmProvider::AnthropicToken,
            "openai" => LlmProvider::OpenAi,
            other => anyhow::bail!(
                "Unknown provider: {other}. Use 'anthropic', 'anthropic-token', or 'openai'."
            ),
        };

        let api_key = match &provider {
            LlmProvider::Anthropic => std::env::var("ANTHROPIC_API_KEY")
                .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))?,
            LlmProvider::AnthropicToken => std::env::var("ANTHROPIC_TOKEN")
                .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
                .map_err(|_| anyhow::anyhow!(
                    "ANTHROPIC_TOKEN not set. Set it to your Claude Max session token."
                ))?,
            LlmProvider::OpenAi => std::env::var("OPENAI_API_KEY")
                .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY not set"))?,
        };

        Ok(Self {
            depth_limit: depth,
            num_branches: branches,
            survivor_threshold: threshold,
            novel_threshold,
            bg_universal_ratio: ratio,
            draws_per_depth: draws,
            temperature,
            atom: AtomConfig {
                objects,
                relationships,
                properties,
            },
            llm: LlmConfig {
                provider,
                api_key,
                model: model.to_string(),
                max_concurrent,
                low_temperature: 0.1,
                high_temperature: temperature,
            },
        })
    }
}
