#[cfg(test)]
mod receipt_tests {
    use omokoda_core::receipt::{Receipt, ReceiptStore};

    #[test]
    fn receipt_has_required_fields() {
        let r = Receipt::new("agent-001", "web_search", "bitcoin origin");
        assert!(!r.agent_id.is_empty());
        assert!(!r.action.is_empty());
        assert!(!r.payload.is_empty());
        assert!(!r.receipt_id.is_empty());
        assert!(r.timestamp > 0);
    }

    #[test]
    fn same_input_different_receipts() {
        // Each receipt gets a unique ID even with same inputs
        let a = Receipt::new("agent-001", "web_search", "query");
        let b = Receipt::new("agent-001", "web_search", "query");
        assert_ne!(a.receipt_id, b.receipt_id);
    }

    #[test]
    fn receipt_id_is_hex_string() {
        let r = Receipt::new("agent-001", "web_search", "query");
        assert!(r.receipt_id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn receipt_store_is_empty_on_init() {
        let store = ReceiptStore::new();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn receipt_store_records_receipt() {
        let mut store = ReceiptStore::new();
        let r = Receipt::new("agent-001", "web_search", "query");
        store.record(r);
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn receipt_store_retrieves_by_id() {
        let mut store = ReceiptStore::new();
        let r = Receipt::new("agent-001", "web_search", "query");
        let id = r.receipt_id.clone();
        store.record(r);
        let found = store.get(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().receipt_id, id);
    }

    #[test]
    fn receipt_payload_is_blake3_hash() {
        // Payload must be a 64-char hex string (BLAKE3 output)
        let r = Receipt::new("agent-001", "web_search", "query");
        assert_eq!(r.payload.len(), 64);
        assert!(r.payload.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn dry_run_flag_is_always_false() {
        // Receipts must never be dry_run — enforce this at the type level
        let r = Receipt::new("agent-001", "act", "params");
        assert!(!r.dry_run);
    }
}
