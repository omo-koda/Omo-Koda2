# Memory Spec

## Goal
Define a simple, layered memory model for agents.

## Memory layers
- **Working memory**: active context for the current interaction.
- **Short-term memory**: recent history and intermediate notes.
- **Long-term memory**: durable knowledge, identity, and receipts.
- **Public memory**: publishable knowledge and garden entries.
- **Private memory**: sealed thoughts, internal plans, and hidden state.

## Rules
- `birth` initializes memory state.
- `think` updates working/short-term memory and may commit to private or public storage.
- `act` may append receipts and durable state changes.
- Memory access is gated by privacy mode and tool permissions.

## Hidden memory internals
- Memory keys, rotation logic, and hidden derivation are runtime concerns.
- The public language must never expose raw memory keys or derivation steps.
- Memory structure should stay stable even if storage backends change.

## Persistence
- Durable state should survive sessions.
- Public memory must remain queryable.
- Private memory must remain inaccessible outside the authorized runtime.

## ARGON2ID PARAMETERS (v1.0 — never change without versioned migration)
- memory: 65536 (64MB)
- iterations: 3
- parallelism: 1
- output_len: 32 bytes
- salt: BLAKE3(agent_id || birth_timestamp || chain_id)

Changing these parameters invalidates all existing mnemonics on prior params.
All agents on the same network MUST use identical parameters.

## Notes
These parameters are protocol constants. Treat them as frozen contract data, not implementation detail.