use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActQuality {
    Failed,      // -0.5x gain (slashing)
    Basic,       // 1.0x gain
    Useful,      // 1.25x gain
    HighValue,   // 1.5x gain
    Exceptional, // 2.0x gain
}

impl ActQuality {
    pub fn multiplier(&self) -> f64 {
        match self {
            ActQuality::Failed => -0.5,
            ActQuality::Basic => 1.0,
            ActQuality::Useful => 1.25,
            ActQuality::HighValue => 1.5,
            ActQuality::Exceptional => 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HookDecision {
    Allow,
    Deny(String),
    Warn(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookContext {
    pub tool_name: String,
    pub input: String,
    pub output: Option<String>,
    pub reputation: f64,
    pub tier: u8,
}

pub trait Hook: std::fmt::Debug + Send + Sync {
    fn run(&self, ctx: &HookContext) -> HookDecision;
}

#[derive(Debug)]
pub struct ShellHook {
    pub command: String,
}

impl Hook for ShellHook {
    fn run(&self, ctx: &HookContext) -> HookDecision {
        let child = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .env("OMOKODA_TOOL", &ctx.tool_name)
            .env("OMOKODA_REP", ctx.reputation.to_string())
            .env("OMOKODA_TIER", ctx.tier.to_string())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string());

        let mut child = match child {
            Ok(c) => c,
            Err(e) => return HookDecision::Warn(format!("Failed to spawn hook: {}", e)),
        };

        if let Some(mut stdin) = child.stdin.take() {
            let json = serde_json::to_string(ctx).unwrap_or_default();
            let _ = stdin.write_all(json.as_bytes());
        }

        let status = match child.wait() {
            Ok(s) => s,
            Err(e) => return HookDecision::Warn(format!("Hook wait failed: {}", e)),
        };

        match status.code() {
            Some(0) => HookDecision::Allow,
            Some(2) => HookDecision::Deny("Hook denied execution".to_string()),
            Some(code) => HookDecision::Warn(format!("Hook returned non-zero code: {}", code)),
            None => HookDecision::Warn("Hook terminated by signal".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct ReputationGate {
    pub min_reputation: f64,
}

impl Hook for ReputationGate {
    fn run(&self, ctx: &HookContext) -> HookDecision {
        if ctx.reputation < self.min_reputation {
            HookDecision::Deny(format!(
                "Reputation too low. Required: {}, Current: {}",
                self.min_reputation, ctx.reputation
            ))
        } else {
            HookDecision::Allow
        }
    }
}

pub struct HookRunner {
    pub pre_act: Vec<Box<dyn Hook>>,
    pub post_act: Vec<Box<dyn Hook>>,
}

impl Default for HookRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for HookRunner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HookRunner")
            .field("pre_act_count", &self.pre_act.len())
            .field("post_act_count", &self.post_act.len())
            .finish()
    }
}

impl HookRunner {
    pub fn new() -> Self {
        Self {
            pre_act: Vec::new(),
            post_act: Vec::new(),
        }
    }

    pub fn run_pre(&self, ctx: &HookContext) -> HookDecision {
        for hook in &self.pre_act {
            match hook.run(ctx) {
                HookDecision::Allow => continue,
                other => return other,
            }
        }
        HookDecision::Allow
    }

    pub fn run_post(&self, ctx: &HookContext) -> HookDecision {
        for hook in &self.post_act {
            match hook.run(ctx) {
                HookDecision::Allow => continue,
                other => return other,
            }
        }
        HookDecision::Allow
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JusticeEngine {
    #[serde(skip, default = "HookRunner::new")]
    pub hook_runner: HookRunner,
}

impl JusticeEngine {
    pub fn new() -> Self {
        Self {
            hook_runner: HookRunner::new(),
        }
    }

    pub fn evaluate_act(&self, tool_output: &str, is_error: bool) -> ActQuality {
        if is_error {
            return ActQuality::Failed;
        }

        // Simple heuristic for now: length of output as a proxy for utility
        let len = tool_output.len();
        if len > 500 {
            ActQuality::HighValue
        } else if len > 100 {
            ActQuality::Useful
        } else {
            ActQuality::Basic
        }
    }

    pub fn evaluate_action(
        &self,
        current_reputation: f64,
        _tool: &str,
        _params: &str,
        output: &str,
        is_success: bool,
    ) -> (f64, ActQuality) {
        use crate::reputation::{reputation_gain, ACT_TIER_0, ACT_TIER_1, ACT_TIER_2, ACT_TIER_4};
        let quality = self.evaluate_act(output, !is_success);

        let base = match quality {
            ActQuality::Failed => ACT_TIER_0,
            ActQuality::Basic => ACT_TIER_0,
            ActQuality::Useful => ACT_TIER_1,
            ActQuality::HighValue => ACT_TIER_2,
            ActQuality::Exceptional => ACT_TIER_4,
        };

        let gain = reputation_gain(base, current_reputation, quality.multiplier());
        (current_reputation + gain, quality)
    }

    pub fn evaluate_think(&self, current_reputation: f64, high_value: bool) -> (f64, f64) {
        use crate::reputation::{reputation_gain, THINK_HIGH, THINK_NORMAL};
        let base = if high_value { THINK_HIGH } else { THINK_NORMAL };
        let gain = reputation_gain(base, current_reputation, 1.0);
        (current_reputation + gain, gain)
    }

    pub fn check_ethics_violation(&self, reputation: f64) -> f64 {
        reputation * 0.75
    }

    pub fn check_budget_overrun(&self, reputation: f64) -> f64 {
        reputation * 0.90
    }
}
