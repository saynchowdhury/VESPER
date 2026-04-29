// Vesper white-label change — safe, no UI impact
// crates/ai/src/providers/vesper_router.rs
//
// Smart Inference Router for Vesper
// - Free tier: Rotates across your 3-4 Groq API keys (generous limits)
// - Auto-fallback: When Groq quota exhausted → Cloudflare Workers AI (your infra, zero marginal cost)
// - Paid users: Always use Cloudflare Workers AI
// - Friendly model names: "Vesper Fast", "Vesper Pro"
// - All calls go through existing http_client crate (OpenAI-compatible format)

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::http_client::HttpClient;
use crate::providers::{Provider, ProviderError, ProviderResponse};

/// Configuration for each inference account
#[derive(Clone, Debug)]
pub struct InferenceAccount {
    pub id: String,
    pub provider: String,           // "groq" or "cloudflare"
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub daily_limit: u64,           // tokens per day
    pub tokens_used_today: Arc<RwLock<u64>>,
}

pub struct VesperSmartRouter {
    accounts: Vec<InferenceAccount>,
    http_client: Arc<HttpClient>,
    current_account_index: Arc<RwLock<usize>>,
}

impl VesperSmartRouter {
    pub fn new(accounts: Vec<InferenceAccount>, http_client: Arc<HttpClient>) -> Self {
        Self {
            accounts,
            http_client,
            current_account_index: Arc::new(RwLock::new(0)),
        }
    }

    /// Main entry point — decides which backend to use
    pub async fn complete(
        &self,
        messages: Vec<serde_json::Value>,
        model_name: &str,           // "Vesper Fast", "Vesper Pro", or OpenRouter models
        user_tier: &str,            // "free" or "paid"
        user_openrouter_key: Option<String>, // BYOK Feature
    ) -> Result<ProviderResponse, ProviderError> {
        let (real_model, base_url, api_key, used_account) = if let Some(key) = user_openrouter_key {
            // BYOK OpenRouter Mode
            let mapped_model = match model_name {
                "Vesper Fast" => "meta-llama/llama-3-70b-instruct",
                "Vesper Pro"  => "meta-llama/llama-3.1-405b-instruct",
                _ => model_name, // Let user specify raw OpenRouter models
            };
            (mapped_model.to_string(), "https://openrouter.ai/api/v1".to_string(), key, None)
        } else {
            // Managed Vesper Mode
            let account = self.select_account(model_name, user_tier).await?;
            let mapped_model = match model_name {
                "Vesper Fast" => "llama-3.1-70b-versatile",
                "Vesper Pro"  => "llama-3.1-405b-instruct",
                _ => "llama-3.1-70b-versatile",
            };
            (mapped_model.to_string(), account.base_url.clone(), account.api_key.clone(), Some(account))
        };

        let payload = serde_json::json!({
            "model": real_model,
            "messages": messages,
            "temperature": 0.7,
            "max_tokens": 2048,
        });

        let url = format!("{}/chat/completions", base_url);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        // OpenRouter specific headers (best practice)
        if base_url.contains("openrouter.ai") {
            headers.insert("HTTP-Referer".to_string(), "https://github.com/saynchowdhury/VESPER".to_string());
            headers.insert("X-Title".to_string(), "Vesper Terminal".to_string());
        }

        // Use existing HttpClient (assumed to have post_json method)
        let response = self.http_client
            .post_json(&url, &payload, Some(headers))
            .await
            .map_err(|e| ProviderError::RequestFailed(e.to_string()))?;

        // Track usage (simplified — in real impl update SQLite)
        if let Some(acc) = used_account {
            let mut used = acc.tokens_used_today.write().await;
            *used += response.usage.total_tokens as u64;   // approximate
        }

        Ok(response)
    }

    async fn select_account(&self, model_name: &str, user_tier: &str) -> Result<InferenceAccount, ProviderError> {
        let mut idx = self.current_account_index.write().await;

        if user_tier == "paid" {
            // Paid users → always use Cloudflare (your infra)
            if let Some(cf) = self.accounts.iter().find(|a| a.provider == "cloudflare") {
                return Ok(cf.clone());
            }
        }

        // Free tier or fallback → rotate Groq accounts
        let groq_accounts: Vec<_> = self.accounts.iter()
            .filter(|a| a.provider == "groq")
            .cloned()
            .collect();

        if groq_accounts.is_empty() {
            return Err(ProviderError::NoAvailableAccount);
        }

        // Simple round-robin + check quota
        for _ in 0..groq_accounts.len() {
            let account = &groq_accounts[*idx % groq_accounts.len()];
            *idx = (*idx + 1) % groq_accounts.len();

            let used = *account.tokens_used_today.read().await;
            if used < account.daily_limit {
                return Ok(account.clone());
            }
        }

        // All Groq accounts exhausted → fallback to Cloudflare
        if let Some(cf) = self.accounts.iter().find(|a| a.provider == "cloudflare") {
            Ok(cf.clone())
        } else {
            Err(ProviderError::AllAccountsExhausted)
        }
    }
}

// Error types (extend existing ProviderError if needed)
#[derive(Debug)]
pub enum ProviderError {
    RequestFailed(String),
    NoAvailableAccount,
    AllAccountsExhausted,
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            ProviderError::NoAvailableAccount => write!(f, "No inference account available"),
            ProviderError::AllAccountsExhausted => write!(f, "All inference accounts exhausted"),
        }
    }
}

impl std::error::Error for ProviderError {}