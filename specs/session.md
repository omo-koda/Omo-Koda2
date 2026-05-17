# Session Specification (v1.0 — FROZEN)

## Purpose
Defines the structure and persistence of conversation sessions, ensuring privacy and continuity.

## Session Object
A session consists of public and private components:

```json
{
  "version": 1,
  "session_id": "string (BLAKE3)",
  "agent_id": "string",
  "public_messages": [
    {
      "role": "User|Assistant|System",
      "blocks": [
        { "type": "Text", "content": "..." },
        { "type": "ToolUse", "id": "...", "name": "...", "input": "..." },
        { "type": "ToolResult", "tool_use_id": "...", "output": "...", "is_error": "boolean" }
      ],
      "usage": { "input_tokens": 0, "output_tokens": 0 }
    }
  ],
  "encrypted_private": {
    "version": 1,
    "private_ciphertext": "array/base64-compatible bytes (encrypted private blocks)",
    "nonce": "12 bytes",
    "salt": "16 bytes",
    "key_version": 1,
    "kdf": {
      "algorithm": "argon2id-v0x13",
      "memory_kb": 65536,
      "iterations": 3,
      "parallelism": 1,
      "output_len": 32
    }
  },
  "merkle_root": "string (BLAKE3)"
}
```

## Persistence Rules
- **Public Messages**: Stored as versioned JSON. Eligible for receipt anchoring.
- **Private Messages**: Must be encrypted with **ChaCha20Poly1305** using a key derived via **Argon2id** from the agent's memory seed (`K_root`).
- **Storage Location**: Runtime auto-save stores each agent under `~/.omokoda/sessions/<agent_id>/agent.json` by default. Custom test/runtime paths must preserve the same `<agent_id>/agent.json` shape unless explicitly using a direct persistence path.
- **Permissions**: Session directories are `0700` and files are `0600` on Unix platforms.
- **Versioning**: The outer session and inner encrypted envelope are separately versioned. Unsupported versions fail closed until a migration is implemented.
- **Asymmetric Persistence**:
  - User messages: Blocking save (required for resume).
  - Assistant messages: Fire-and-forget (optional, regeneratable).

## Encryption Parameters
- **KDF**: Argon2id
- **Memory**: 65536 KB
- **Iterations**: 3
- **Parallelism**: 1
- **Cipher**: ChaCha20Poly1305

## Lifecycle
- **Birth**: Creates the initial session with the agent's Odu seed.
- **Seal**: Serializes private messages/Odu private data, encrypts them as `encrypted_private.private_ciphertext`, drops plaintext private memory, and zeroizes passphrase-derived keys.
- **Resume**: Loads public history and encrypted private envelope. Private messages remain unavailable until explicit unlock.
- **Unlock**: Derives the unlock key with Argon2id, authenticates/decrypts the private envelope with ChaCha20Poly1305, and fails closed on wrong key/passphrase/seed with no partial private data returned.
- **Rotate Key**: Decrypts with the old key, reseals with a fresh nonce and incremented key version, and invalidates the old key.
- **Archive**: Finalizes the Merkle root and seals the session.
- **Destroy**: Zeroizes local keys and archives the encrypted blob.

## Leakage Invariants
- Private prompt/assistant text must never appear in saved JSON or exported session JSON.
- Sealed private memory must not remain reachable through `AgentState::private_data()`.
- `/private` content must remain encrypted even when the public session is exported.
- Seal/unseal emits internal audit events for future Justice logging.
