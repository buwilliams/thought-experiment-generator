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
}

#[derive(Debug, Clone)]
pub struct Config {
    pub llm: LlmConfig,
    pub consistency_threshold: f64,
    pub problem_admission_threshold: f64,
    pub min_run_count: u32,
    pub temperature: f64,
}

impl Config {
    pub fn new(
        provider_str: &str,
        model: &str,
        max_concurrent: usize,
        consistency_threshold: f64,
        problem_admission_threshold: f64,
        min_run_count: u32,
        temperature: f64,
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
            LlmProvider::AnthropicToken => std::env::var("CLAUDE_CODE_OAUTH_TOKEN")
                .map_err(|_| anyhow::anyhow!(
                    "CLAUDE_CODE_OAUTH_TOKEN not set. Set it to your Claude Max session token."
                ))?,
            LlmProvider::OpenAi => std::env::var("OPENAI_API_KEY")
                .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY not set"))?,
        };

        Ok(Self {
            llm: LlmConfig {
                provider,
                api_key,
                model: model.to_string(),
                max_concurrent,
            },
            consistency_threshold,
            problem_admission_threshold,
            min_run_count,
            temperature,
        })
    }
}
