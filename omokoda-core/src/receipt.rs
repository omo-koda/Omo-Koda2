use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Receipt {
    pub agent_id: String,
    pub action: String,
    pub payload: String,
    pub receipt_id: String,
    pub timestamp: u64,
    pub dry_run: bool,
}

impl Receipt {
    pub fn new(agent_id: &str, action: &str, params: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_secs();
        let timestamp_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos();

        let payload = blake3_hash_hex(&[action.as_bytes(), params.as_bytes()]);
        let receipt_id = blake3_hash_hex(&[
            agent_id.as_bytes(),
            action.as_bytes(),
            params.as_bytes(),
            timestamp_nanos.to_string().as_bytes(),
        ]);

        Self {
            agent_id: agent_id.to_string(),
            action: action.to_string(),
            payload,
            receipt_id,
            timestamp,
            dry_run: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct ReceiptStore {
    receipts: HashMap<String, Receipt>,
}

impl ReceiptStore {
    pub fn new() -> Self {
        Self {
            receipts: HashMap::new(),
        }
    }

    pub fn record(&mut self, receipt: Receipt) {
        self.receipts.insert(receipt.receipt_id.clone(), receipt);
    }

    pub fn get(&self, receipt_id: &str) -> Option<&Receipt> {
        self.receipts.get(receipt_id)
    }

    pub fn count(&self) -> usize {
        self.receipts.len()
    }
}

fn blake3_hash_hex(parts: &[&[u8]]) -> String {
    let mut hasher = blake3::Hasher::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().to_hex().to_string()
}
