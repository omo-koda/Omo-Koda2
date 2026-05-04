use crate::parser::Statement;
use crate::receipt::{Receipt, ReceiptStore};

const ACT_TIER_0_BASE: f64 = 0.040;

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub receipt: Option<Receipt>,
    pub private_mode: bool,
}

#[derive(Debug, Default)]
pub struct Steward {
    agent_id: Option<String>,
    reputation: f64,
    receipts: ReceiptStore,
}

impl Steward {
    pub fn new() -> Self {
        Self {
            agent_id: None,
            reputation: 0.0,
            receipts: ReceiptStore::new(),
        }
    }

    pub fn dispatch(&mut self, stmt: Statement) -> Result<ExecutionResult, String> {
        match stmt {
            Statement::Birth { name, .. } => {
                self.agent_id = Some(name);
                Ok(ExecutionResult {
                    receipt: None,
                    private_mode: false,
                })
            }
            Statement::Think { private, .. } => {
                self.ensure_born()?;
                Ok(ExecutionResult {
                    receipt: None,
                    private_mode: private,
                })
            }
            Statement::Act { tool, params, .. } => {
                let agent_id = self.ensure_born()?.to_string();
                if !tool_allowed(self.tier(), &tool) {
                    return Err(format!("tool requires higher tier: {tool}"));
                }

                let receipt = Receipt::new(&agent_id, &tool, &params);
                self.receipts.record(receipt.clone());
                self.reputation = (self.reputation + rep_gain(ACT_TIER_0_BASE, self.reputation)).min(100.0);

                Ok(ExecutionResult {
                    receipt: Some(receipt),
                    private_mode: false,
                })
            }
            Statement::SlashCmd { .. } => Err("slash commands are not executable by the Steward".into()),
        }
    }

    pub fn reputation(&self) -> f64 {
        self.reputation
    }

    pub fn tier(&self) -> u8 {
        tier_for(self.reputation)
    }

    pub fn apply_daily_decay(&mut self, days: u64) {
        if days == 0 {
            return;
        }

        let early_days = days.min(7) as f64;
        let late_days = days.saturating_sub(7) as f64;
        let penalty = (early_days * 0.008) + (late_days * 0.015);
        self.reputation = (self.reputation - penalty).max(0.0);
    }

    pub fn set_reputation_for_test(&mut self, reputation: f64) {
        self.reputation = reputation.clamp(0.0, 100.0);
    }

    fn ensure_born(&self) -> Result<&str, String> {
        self.agent_id
            .as_deref()
            .ok_or_else(|| "agent must be born first".to_string())
    }
}

fn difficulty(rep: f64) -> f64 {
    1.0 / (1.0 + (rep / 25.0))
}

fn rep_gain(base: f64, rep: f64) -> f64 {
    base * difficulty(rep)
}

fn tier_for(reputation: f64) -> u8 {
    if reputation >= 100.0 {
        5
    } else if reputation >= 80.0 {
        4
    } else if reputation >= 60.0 {
        3
    } else if reputation >= 40.0 {
        2
    } else if reputation >= 20.0 {
        1
    } else {
        0
    }
}

fn tool_allowed(tier: u8, tool: &str) -> bool {
    let allowed = match tier {
        0 => &["web_search", "note_taking"][..],
        1 => &["web_search", "note_taking", "image_gen_basic"][..],
        2 => &["web_search", "note_taking", "image_gen_basic", "code_runner"][..],
        3 => &[
            "web_search",
            "note_taking",
            "image_gen_basic",
            "code_runner",
            "data_analysis",
            "api_connect",
        ][..],
        4 => &[
            "web_search",
            "note_taking",
            "image_gen_basic",
            "code_runner",
            "data_analysis",
            "api_connect",
            "agent_orchestration",
        ][..],
        _ => &[
            "web_search",
            "note_taking",
            "image_gen_basic",
            "code_runner",
            "data_analysis",
            "api_connect",
            "agent_orchestration",
            "self_modification",
            "multi_agent_fabric",
        ][..],
    };

    allowed.contains(&tool)
}
