use crate::session::ConversationMessage;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderClass {
    Local,
    BrowserLocal,
    RegisteredLocal,
    External,
    Hive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetadata {
    pub name: String,
    pub class: ProviderClass,
    pub endpoint: String,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn metadata(&self) -> &ProviderMetadata;
    async fn generate(&self, prompt: &str, history: &[ConversationMessage]) -> Result<String, String>;
}

#[derive(Debug)]
pub struct ProviderRegistry {
    pub ollama: OllamaProvider,
    pub mock: Option<MockProvider>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            ollama: OllamaProvider::new("http://localhost:11434".to_string()),
            mock: None,
        }
    }

    pub fn with_mock(response: String) -> Self {
        Self {
            ollama: OllamaProvider::new("http://localhost:11434".to_string()),
            mock: Some(MockProvider::new(response)),
        }
    }

    pub fn is_allowed_in_private(&self, provider: &ProviderMetadata) -> bool {
        match provider.class {
            ProviderClass::Local => {
                provider.endpoint.contains("localhost") || provider.endpoint.contains("127.0.0.1")
            }
            ProviderClass::BrowserLocal => true,
            ProviderClass::RegisteredLocal => true,
            ProviderClass::External => false,
            ProviderClass::Hive => false,
        }
    }

    pub async fn route_think(
        &self,
        prompt: &str,
        history: &[ConversationMessage],
        private_mode: bool,
    ) -> Result<String, String> {
        // 0. Try Mock if present (for tests)
        if let Some(ref mock) = self.mock {
            return mock.generate(prompt, history).await;
        }

        // Fallback chain: Ollama -> WebLLM (BrowserLocal stub) -> HARD FAIL
        
        // 1. Try Ollama
        if self.is_allowed_in_private(self.ollama.metadata()) || !private_mode {
            match tokio::time::timeout(Duration::from_secs(30), self.ollama.generate(prompt, history)).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(e)) => {
                    if private_mode {
                        return Err(format!("Ollama error in /private (HARD FAIL): {}", e));
                    }
                }
                Err(_) => {
                    if private_mode {
                        return Err("Ollama timeout in /private (HARD FAIL)".to_string());
                    }
                }
            }
        }

        // 2. Try WebLLM (Stub for now)
        // If we were in a real browser environment, we'd call WebLLM here.
        // For now, if Ollama failed and we are in private, we HARD FAIL.
        
        if private_mode {
            Err("No local provider available in /private mode (HARD FAIL)".to_string())
        } else {
            // In public mode, we could fallback to external, but for now just fail if local is gone
            Err("Reasoning failed: no provider responded".to_string())
        }
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct OllamaProvider {
    metadata: ProviderMetadata,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(endpoint: String) -> Self {
        Self {
            metadata: ProviderMetadata {
                name: "Ollama".to_string(),
                class: ProviderClass::Local,
                endpoint,
            },
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn generate(&self, prompt: &str, _history: &[ConversationMessage]) -> Result<String, String> {
        let url = format!("{}/api/generate", self.metadata.endpoint);
        let body = serde_json::json!({
            "model": "llama3", // Default model
            "prompt": prompt,
            "stream": false
        });

        let resp = self.client.post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Ollama status error: {}", resp.status()));
        }

        let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(json["response"].as_str().unwrap_or("").to_string())
    }
}

#[derive(Debug)]
pub struct MockProvider {
    pub metadata: ProviderMetadata,
    pub response: String,
}

impl MockProvider {
    pub fn new(response: String) -> Self {
        Self {
            metadata: ProviderMetadata {
                name: "Mock".to_string(),
                class: ProviderClass::Local,
                endpoint: "mock://".to_string(),
            },
            response,
        }
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    fn metadata(&self) -> &ProviderMetadata {
        &self.metadata
    }

    async fn generate(&self, _prompt: &str, _history: &[ConversationMessage]) -> Result<String, String> {
        Ok(self.response.clone())
    }
}
