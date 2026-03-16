use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use tokio::sync::Semaphore;
use tracing::{debug, warn};

use crate::config::{LlmConfig, LlmProvider};
use crate::llm::parser::parse_llm_json;

pub struct LlmClient {
    http: reqwest::Client,
    config: LlmConfig,
    semaphore: Arc<Semaphore>,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            config,
            semaphore,
        }
    }

    /// Send a prompt and parse a JSON response of type T.
    pub async fn call<T: DeserializeOwned>(&self, prompt: &str, temperature: f64) -> Result<T> {
        let raw = self.call_raw(prompt, temperature).await?;
        parse_llm_json(&raw).with_context(|| format!("Failed to parse LLM JSON response"))
    }

    /// Raw string response.
    pub async fn call_raw(&self, prompt: &str, temperature: f64) -> Result<String> {
        let _permit = self.semaphore.acquire().await?;
        self.call_with_retry(prompt, temperature, 3).await
    }

    async fn call_with_retry(
        &self,
        prompt: &str,
        temperature: f64,
        max_retries: u32,
    ) -> Result<String> {
        let mut last_err = None;
        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = Duration::from_millis(500 * 2u64.pow(attempt - 1));
                debug!("Retrying LLM call after {delay:?} (attempt {attempt})");
                tokio::time::sleep(delay).await;
            }

            match self.do_call(prompt, temperature).await {
                Ok(text) => return Ok(text),
                Err(e) => {
                    warn!("LLM call failed (attempt {}): {e}", attempt + 1);
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.unwrap())
    }

    async fn do_call(&self, prompt: &str, temperature: f64) -> Result<String> {
        match &self.config.provider {
            LlmProvider::Anthropic => self.call_anthropic(prompt, temperature).await,
            LlmProvider::AnthropicToken => self.call_anthropic_token(prompt, temperature).await,
            LlmProvider::OpenAi => self.call_openai(prompt, temperature).await,
        }
    }

    async fn call_anthropic(&self, prompt: &str, temperature: f64) -> Result<String> {
        let body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": 4096,
            "temperature": temperature,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let resp = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if status.is_server_error() || status == 429 {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error {status}: {text}");
        }

        let json: serde_json::Value = resp.json().await?;

        if let Some(err) = json.get("error") {
            anyhow::bail!("Anthropic API error: {err}");
        }

        let text = json["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|block| block["text"].as_str())
            .ok_or_else(|| anyhow::anyhow!("Unexpected Anthropic response format: {json}"))?;

        Ok(text.to_string())
    }

    /// Call Anthropic API using a Bearer session token (Claude Max subscription).
    async fn call_anthropic_token(&self, prompt: &str, temperature: f64) -> Result<String> {
        let body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": 4096,
            "temperature": temperature,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let resp = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if status.is_server_error() || status == 429 {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error {status}: {text}");
        }

        let json: serde_json::Value = resp.json().await?;

        if let Some(err) = json.get("error") {
            anyhow::bail!("Anthropic API error: {err}");
        }

        let text = json["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|block| block["text"].as_str())
            .ok_or_else(|| anyhow::anyhow!("Unexpected Anthropic response format: {json}"))?;

        Ok(text.to_string())
    }

    async fn call_openai(&self, prompt: &str, temperature: f64) -> Result<String> {
        let body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": 4096,
            "temperature": temperature,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        });

        let resp = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if status.is_server_error() || status == 429 {
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI API error {status}: {text}");
        }

        let json: serde_json::Value = resp.json().await?;

        let text = json["choices"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|choice| choice["message"]["content"].as_str())
            .ok_or_else(|| anyhow::anyhow!("Unexpected OpenAI response format: {json}"))?;

        Ok(text.to_string())
    }
}
