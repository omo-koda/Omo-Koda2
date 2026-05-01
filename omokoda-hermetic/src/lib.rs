use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HermeticState {
    pub risk: f32,
    pub polarity: f32,
    pub rhythm: f32,
    pub gender: f32,
}

pub fn risk_score(prompt: &str) -> f32 {
    let risky_terms = ["harm", "exploit", "steal", "malware", "bypass"];
    let hits = risky_terms.iter().filter(|term| prompt.to_lowercase().contains(**term)).count() as f32;
    (hits / risky_terms.len() as f32).clamp(0.0, 1.0)
}

pub fn polarity_score(tool: &str, args: &str) -> f32 {
    let signal = format!("{} {}", tool.to_lowercase(), args.to_lowercase());
    if signal.contains("delete") || signal.contains("wipe") || signal.contains("drop") {
        0.2
    } else if signal.contains("create") || signal.contains("write") || signal.contains("birth") {
        0.8
    } else {
        0.5
    }
}

pub fn rhythm_check(cooldown: i64, elapsed: i64) -> bool {
    elapsed >= cooldown
}

pub fn gender_balance(think: u32, act: u32) -> f32 {
    let total = think + act;
    if total == 0 {
        return 0.5;
    }
    let balance = think.min(act) as f32 / total as f32;
    balance.clamp(0.0, 1.0)
}

pub fn apply_vibration_decay(agent: &mut crate::Agent) {
    agent.consistency_score = (agent.consistency_score * 0.99).clamp(0.0, 1.0);
}
