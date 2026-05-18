use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TokenEvent {
    Token(String),
    ToolUseStart { id: String, name: String },
    ToolUseInput { id: String, input: String },
    ToolUseEnd { id: String },
    Done,
}

pub trait StreamingProvider {
    fn stream(&self, prompt: &str) -> Box<dyn Iterator<Item = TokenEvent>>;
}

pub fn parse_sse(chunk: &str) -> Vec<TokenEvent> {
    let mut events = Vec::new();
    for line in chunk.lines() {
        if let Some(data) = line.strip_prefix("data: ") {
            let data = data.trim();
            if data == "[DONE]" {
                events.push(TokenEvent::Done);
                continue;
            }

            if let Ok(event) = serde_json::from_str::<TokenEvent>(data) {
                events.push(event);
            } else if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                // Handle various provider formats
                if let Some(text) = json["choices"][0]["delta"]["content"].as_str() {
                    events.push(TokenEvent::Token(text.to_string()));
                } else if let Some(text) = json["completion"].as_str() {
                    events.push(TokenEvent::Token(text.to_string()));
                } else if let Some(text) = json["message"]["content"].as_str() {
                    events.push(TokenEvent::Token(text.to_string()));
                } else if let Some(text) = json["response"].as_str() {
                    events.push(TokenEvent::Token(text.to_string()));
                }
            }
        }
    }
    events
}
