# Receipt Spec

## Goal
Make actions auditable without exposing internal mechanics.

## Receipt rules
- Every public `act` emits a receipt.
- Receipts record: agent id, action name, normalized params, timestamp, and result summary.
- Receipts are append-only.
- Receipts may be anchored to external proof systems later, but that is hidden behind the runtime.

## Public guarantees
- Users can inspect receipt metadata.
- Receipts can be linked to actions and timelines.
- Public receipts should be stable and deterministic for the same normalized action input.

## Hidden guarantees
- Receipt hashing, signing, and anchoring are internal implementation details.
- No receipt API should expose secret keys or raw internal policy state.

## Receipt minting
- Receipt minting MUST use `execute_transaction_block`.
- `dry_run` is PROHIBITED for receipt generation.
- Using `dry_run` produces no on-chain state and violates the audit guarantee.

## Frozen receipt schema
```text
ActReceipt {
  agent_id:  vector<u8>
  action:    vector<u8>
  payload:   vector<u8>
  timestamp: u64  (tx_context epoch)
  receipt_id: address  (object::uid_to_address of temp UID)
}
```
