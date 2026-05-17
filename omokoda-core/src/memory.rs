use crate::session::{ContentBlock, ConversationMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompressionLevel {
    None,
    ContentReplace, // Level 1: Truncate large outputs
    Snip,           // Level 2: Remove old non-essential messages
    MicroCompact,   // Level 3: Deduplicate tool uses/results
    Collapse,       // Level 4: Summarize early conversation
    AutoCompact,    // Level 5: Full session summary
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionThresholds {
    pub level_1: usize, // chars as proxy for tokens
    pub level_2: usize,
    pub level_3: usize,
    pub level_4: usize,
    pub level_5: usize,
}

impl Default for CompressionThresholds {
    fn default() -> Self {
        Self {
            level_1: 4000,
            level_2: 8000,
            level_3: 16000,
            level_4: 32000,
            level_5: 64000,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MemoryEngine {
    pub thresholds: CompressionThresholds,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            thresholds: CompressionThresholds::default(),
        }
    }

    pub fn estimate_usage(&self, messages: &[ConversationMessage]) -> usize {
        messages
            .iter()
            .map(|m| {
                m.blocks
                    .iter()
                    .map(|b| match b {
                        ContentBlock::Text { text } => text.len(),
                        ContentBlock::ToolUse { input, .. } => input.len(),
                        ContentBlock::ToolResult { output, .. } => output.len(),
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn compress(
        &self,
        messages: &mut Vec<ConversationMessage>,
        reputation: f64,
    ) -> CompressionLevel {
        let usage = self.estimate_usage(messages);

        if usage > self.thresholds.level_5 {
            self.auto_compact(messages, reputation);
            CompressionLevel::AutoCompact
        } else if usage > self.thresholds.level_4 {
            self.collapse(messages, reputation);
            CompressionLevel::Collapse
        } else if usage > self.thresholds.level_3 {
            self.micro_compact(messages, reputation);
            CompressionLevel::MicroCompact
        } else if usage > self.thresholds.level_2 {
            self.snip(messages, reputation);
            CompressionLevel::Snip
        } else if usage > self.thresholds.level_1 {
            self.content_replace(messages, reputation);
            CompressionLevel::ContentReplace
        } else {
            CompressionLevel::None
        }
    }

    fn should_preserve(&self, message: &ConversationMessage, _reputation: f64) -> bool {
        // Preserve tool uses and results as they are "receipt anchors"
        // Also preserve system messages
        message.role == crate::session::MessageRole::System
            || message.blocks.iter().any(|b| {
                matches!(
                    b,
                    ContentBlock::ToolUse { .. } | ContentBlock::ToolResult { .. }
                )
            })
    }

    fn content_replace(&self, messages: &mut [ConversationMessage], _reputation: f64) {
        for msg in messages {
            for block in &mut msg.blocks {
                if let ContentBlock::ToolResult { output, .. } = block {
                    if output.len() > 1000 {
                        output.truncate(1000);
                        output.push_str("\n... [TRUNCATED] ...");
                    }
                }
            }
        }
    }

    fn snip(&self, messages: &mut Vec<ConversationMessage>, reputation: f64) {
        let mut i = 0;
        let mut snipped = 0;
        let limit = (messages.len() / 10).max(1);
        while i < messages.len() && snipped < limit {
            if !self.should_preserve(&messages[i], reputation) {
                messages.remove(i);
                snipped += 1;
            } else {
                i += 1;
            }
        }
    }

    fn micro_compact(&self, messages: &mut [ConversationMessage], reputation: f64) {
        self.content_replace(messages, reputation);
    }

    fn collapse(&self, messages: &mut Vec<ConversationMessage>, _reputation: f64) {
        if messages.len() > 20 {
            let to_collapse = 10;
            let summary_text = format!("[COLLAPSED {} EARLY MESSAGES]", to_collapse);
            messages.drain(0..to_collapse);
            messages.insert(
                0,
                ConversationMessage {
                    role: crate::session::MessageRole::System,
                    blocks: vec![ContentBlock::Text { text: summary_text }],
                    is_private: false,
                    timestamp: 0,
                },
            );
        }
    }

    fn auto_compact(&self, messages: &mut Vec<ConversationMessage>, reputation: f64) {
        let usage = self.estimate_usage(messages);
        let target = self.thresholds.level_3;
        if usage > target {
            self.collapse(messages, reputation);
            self.snip(messages, reputation);
            self.content_replace(messages, reputation);
        }
    }
}
