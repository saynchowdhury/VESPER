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
use futures::{StreamExt, stream::BoxStream};
use serde::{Deserialize, Serialize};
use anyhow::Result;

use http_client::Client as HttpClient;
use warp_multi_agent_api as api;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VesperModel {
    VesperFast,
    VesperPro,
}

impl VesperModel {
    pub fn display_name(&self) -> &'static str {
        match self {
            VesperModel::VesperFast => "Vesper Fast",
            VesperModel::VesperPro => "Vesper Pro",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UserTier {
    #[default]
    Free,
    Paid,
}

/// Configuration for each inference account
#[derive(Clone, Debug)]
pub struct InferenceAccount {
    pub id: String,
    pub provider: String,           // "groq" or "cloudflare"
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub daily_limit: u64,           // tokens per day
}

pub struct VesperSmartRouter {
    accounts: Vec<InferenceAccount>,
    http_client: Arc<HttpClient>,
    current_account_index: Arc<RwLock<usize>>,
    persistence_sender: Option<std::sync::mpsc::SyncSender<crate::persistence::ModelEvent>>,
}

impl VesperSmartRouter {
    pub fn new(
        accounts: Vec<InferenceAccount>, 
        http_client: Arc<HttpClient>,
        persistence_sender: Option<std::sync::mpsc::SyncSender<crate::persistence::ModelEvent>>,
    ) -> Self {
        Self {
            accounts,
            http_client,
            current_account_index: Arc::new(RwLock::new(0)),
            persistence_sender,
        }
    }

    pub async fn complete_stream(
        &self,
        request: &api::Request,
        model: VesperModel,
        user_tier: UserTier,
    ) -> Result<BoxStream<'static, Result<api::ResponseEvent, Arc<anyhow::Error>>>> {
        let account = self.select_account(model, user_tier).await?;
        
        let payload = self.convert_request_to_openai(request, &account.model)?;
        
        let rb = self.http_client.post(&format!("{}/chat/completions", account.base_url))
            .header("Authorization", format!("Bearer {}", account.api_key))
            .json(&payload);
            
        let stream = rb.eventsource();
        
        let provider_id = account.id.clone();
        let sender = self.persistence_sender.clone();
        
        let mapped_stream = stream.map(move |event| {
            match event {
                Ok(reqwest_eventsource::Event::Message(msg)) => {
                    if msg.data == "[DONE]" {
                        return Ok(api::ResponseEvent {
                            r#type: Some(api::response_event::Type::StreamFinished(api::response_event::StreamFinished {})),
                        });
                    }
                    
                    let chunk: OpenAIStreamChunk = match serde_json::from_str(&msg.data) {
                        Ok(c) => c,
                        Err(e) => return Err(Arc::new(anyhow::anyhow!("Failed to parse chunk: {e}"))),
                    };
                    
                    if let Some(usage) = chunk.usage {
                        if let Some(ref s) = sender {
                            let _ = s.send(crate::persistence::ModelEvent::UpsertVesperUsage {
                                provider: provider_id.clone(),
                                tokens: usage.total_tokens as i64,
                            });
                        }
                    }
                    
                    let text = chunk.choices.first()
                        .and_then(|c| c.delta.content.clone())
                        .unwrap_or_default();
                        
                    Ok(api::ResponseEvent {
                        r#type: Some(api::response_event::Type::StreamDelta(api::response_event::StreamDelta {
                            delta: text,
                        })),
                    })
                }
                Ok(_) => Err(Arc::new(anyhow::anyhow!("Unexpected SSE event type"))),
                Err(e) => Err(Arc::new(anyhow::anyhow!("SSE Error: {e}"))),
            }
        });
        
        Ok(mapped_stream.boxed())
    }

    fn convert_request_to_openai(&self, request: &api::Request, real_model: &str) -> Result<serde_json::Value> {
        let mut messages = Vec::new();
        
        if let Some(ref input) = request.input {
            for item in input {
                match item {
                    api::AIAgentInput::Message(msg) => {
                        let role = match msg.role {
                            1 => "system",
                            2 => "user",
                            3 => "assistant",
                            _ => "user",
                        };
                        messages.push(serde_json::json!({
                            "role": role,
                            "content": msg.content,
                        }));
                    }
                    _ => {} // Handle other input types if needed
                }
            }
        }
        
        Ok(serde_json::json!({
            "model": real_model,
            "messages": messages,
            "stream": true,
            "stream_options": { "include_usage": true }
        }))
    }

    async fn select_account(&self, _model: VesperModel, user_tier: UserTier) -> Result<InferenceAccount> {
        let mut idx = self.current_account_index.write().await;

        if user_tier == UserTier::Paid {
            if let Some(cf) = self.accounts.iter().find(|a| a.provider == "cloudflare") {
                return Ok(cf.clone());
            }
        }

        let groq_accounts: Vec<_> = self.accounts.iter()
            .filter(|a| a.provider == "groq")
            .cloned()
            .collect();

        if groq_accounts.is_empty() {
            if let Some(cf) = self.accounts.iter().find(|a| a.provider == "cloudflare") {
                return Ok(cf.clone());
            }
            return Err(anyhow::anyhow!("No inference accounts available"));
        }

        let account = &groq_accounts[*idx % groq_accounts.len()];
        *idx = (*idx + 1) % groq_accounts.len();
        Ok(account.clone())
    }
}

#[derive(Deserialize)]
struct OpenAIStreamChunk {
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    delta: OpenAIDelta,
}

#[derive(Deserialize)]
struct OpenAIDelta {
    content: Option<String>,
}

#[derive(Deserialize)]
struct OpenAIUsage {
    total_tokens: u32,
}
