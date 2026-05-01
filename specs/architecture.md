# Agent OS Architecture Spec

## Goal
Define a spec-first Agent OS with a tiny public surface and a rich hidden runtime.

## Layers
1. **Surface language**: `birth`, `think`, `act`
2. **Interpreter**: parses and routes surface commands
3. **Policy + Hermetic engine**: privacy, permissions, hidden rules
4. **Memory + receipts**: persistence and auditability
5. **Stdlib / hidden modules**: identity, tools, swarm, reputation, economy
6. **Frontends**: terminal, web, mobile, dashboard

## Principles
- Surface simplicity first.
- Hidden complexity allowed only behind internal modules.
- Every new capability should be expressed as a runtime module, not a new primitive.
- The public language must remain stable even as internals evolve.

## Build order
1. Freeze language, privacy, memory, and receipt specs.
2. Align runtime data structures to the spec.
3. Split hidden internals into clean modules.
4. Add tests for parsing, privacy, memory routing, and receipts.
5. Add frontends only after the runtime contract is stable.
