# Ọmọ Kọ́dà Soul Interface Spec
**Version: 1.0.0 (FROZEN for Week 2)**

This document defines the boundary between the Rust sovereign runtime and the Sui on-chain identity.

## 1. The Soul Object (soul.move)

The `SoulState` is the root of an agent's existence. It is created exactly once at birth and is immutable.

### 1.1 Fields

```rust
struct SoulState has key, store {
    id: UID,
    agent_id: vector<u8>,           // 16-char agent-xxxxxxxxxxxxxxxx
    birth_timestamp: u64,           // identity-critical (current_unix_timestamp)
    hermetic_seed: vector<u8>,      // 32 bytes derived from Odu Primary Index
    odu_seed_commitment: vector<u8>, // BLAKE3(K_root || odu_index)
    dna_fingerprint: vector<u8>,    // 86-char string
}
```

### 1.2 Invariants
- `birth_timestamp` MUST match the value used for K_0 derivation.
- `agent_id` MUST be unique across the hive.
- `hermetic_seed` is used to initialize the 7 principles.

---

## 2. The Agent Object (agent.move)

The `AgentState` is the mutable expression of the soul.

### 2.1 Fields

```rust
struct AgentState has key {
    id: UID,
    soul: ID,                       // Reference to SoulState
    reputation: u64,                // Stored as rep * 1000
    tier: u8,                       // 0 to 5
    birth_timestamp: u64,           // Must match SoulState
    dna_fingerprint: vector<u8>,    // Must match SoulState
    act_counter: u64,               // Total number of acts performed
}
```

---

## 3. BIPỌ̀N39 Mnemonic Interface

The mnemonic is the recovery path for the agent's soul.

- **Wordlist Size**: 256 tokens.
- **Encoding**: 32 bytes entropy -> 33 words (1 word per 7.75 bits + checksum).
- **Derivation Path**: `m/44'/784'/0'/0/0` (Sui Standard).
- **KDF**: `argon2id(memory=65536, iterations=3, parallelism=1, output=32)`.

---

## 4. SEAL Vault Boundary

- `K_root` NEVER leaves the hardware enclave.
- All on-chain commitments are salted with `BLAKE3(agent_id || birth_timestamp || chain_id)`.
- Key rotation happens every 100 acts or 24 hours.

---

## 5. Entry Functions

### 5.1 `birth_agent`
```move
public entry fun birth_agent(
    name: vector<u8>,
    metadata: vector<MetadataPair>,
    ctx: &mut TxContext
)
```
- Creates `SoulState` and `AgentState`.
- Emits `BirthEvent`.

### 5.2 `record_act`
```move
public entry fun record_act(
    agent: &mut AgentState,
    action: vector<u8>,
    payload_hash: vector<u8>,
    signature: vector<u8>,
    ctx: &mut TxContext
)
```
- Verifies signature against agent's public key.
- Updates `reputation` and `tier`.
- Increments `act_counter`.
- Emits `ActReceipt`.
- **Structurally prohibited from `dry_run` execution.**
