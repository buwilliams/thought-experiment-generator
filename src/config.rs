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
    /// Total thought experiments to generate.
    pub num_experiments: u32,
    /// Background sentences to include per thought experiment.
    pub num_background: u32,
    /// Generated sentences to include per thought experiment.
    pub num_generated: u32,
    /// Random words per line in words.txt.
    pub num_words: u32,
    /// Total sentences in the background and generated pools.
    pub pool_size: usize,
    /// LLM temperature for generation (0.0–1.0).
    pub temperature: f64,
    pub llm: LlmConfig,
}

impl Config {
    pub fn new(
        provider_str: &str,
        model: &str,
        num_experiments: u32,
        num_background: u32,
        num_generated: u32,
        num_words: u32,
        pool_size: usize,
        temperature: f64,
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
            LlmProvider::AnthropicToken => std::env::var("CLAUDE_CODE_OAUTH_TOKEN")
                .map_err(|_| anyhow::anyhow!(
                    "CLAUDE_CODE_OAUTH_TOKEN not set. Set it to your Claude Max session token."
                ))?,
            LlmProvider::OpenAi => std::env::var("OPENAI_API_KEY")
                .map_err(|_| anyhow::anyhow!("OPENAI_API_KEY not set"))?,
        };

        Ok(Self {
            num_experiments,
            num_background,
            num_generated,
            num_words,
            pool_size,
            temperature,
            llm: LlmConfig {
                provider,
                api_key,
                model: model.to_string(),
                max_concurrent,
            },
        })
    }
}
