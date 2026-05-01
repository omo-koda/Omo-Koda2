# Claw – 3-Primitive Agent OS

Vision: a spec-first Agent OS with a public surface limited to `birth`, `think`, and `act`. Everything else lives in hidden stdlibs, policy, memory, receipts, and runtime modules.

Structure:
- claw-core – Rust runtime (surface parsing, dispatch, hidden state)
- claw-hermetic – Hermetic policy engine (risk, polarity, rhythm, balance)
- specs/ – frozen language, privacy, memory, receipts, stdlib, architecture specs
- claw-frontend – Web CLI + UI
- claw-on-chain – Sui identity, permissions, receipts
- claw-sdk – No-code agent creation SDK
- claw-tools – Built-in tools and adapters
- claw-garden – Public wiki/marketplace
- claw-docs – User docs + philosophy

Public surface:
- `birth`
- `think`
- `act`

Hidden internals:
- identity
- policy
- memory
- receipts
- tools
- swarm
- reputation
- economy

Roadmap phases implemented below.