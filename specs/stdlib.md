# Stdlib and Hidden Modules Spec

## Goal
Move everything beyond the three primitives into hidden libraries.

## Hidden modules
- **identity**: agent birth, fingerprints, ownership, transfer
- **policy**: Hermetic checks, permissions, sandbox enforcement
- **memory**: layer routing, persistence, key rotation, recall
- **receipts**: audit trails, hashing, signatures, anchors
- **tools**: approved capabilities and tool registry
- **reputation**: scoring, tiering, unlock rules
- **swarm**: delegation, witness, sub-agent coordination
- **economy**: budgets, credits, fees, rewards

## Rules
- These modules are callable only by the runtime, not directly by users.
- Public syntax should never mention them by name.
- Each module should have a single responsibility and a narrow interface.
- Shared data structures should be hidden behind the runtime boundary.

## Design intent
The user experiences a tiny language.
The runtime experiences a full operating system.
