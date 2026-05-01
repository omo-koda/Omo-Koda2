# Privacy and Visibility Spec

## Goal
Make privacy a runtime rule, not a UI convention.

## Visibility modes
- **Public**: visible to the garden / shared surface and eligible for receipts.
- **Private**: stored only in hidden runtime memory and encrypted storage.
- **Sandbox**: local-only execution, no public publication.

## Rules
- `think` defaults to private unless explicitly published.
- `act` defaults to public unless sandboxed.
- Sandbox mode forces all actions to remain local.
- Hidden keys, memory seeds, and policy state must never be exposed through the public API.
- Ownership transfer must rotate access so prior owners lose private-memory access.

## Boundary guarantees
- Privacy enforcement happens inside the interpreter/runtime.
- The frontend may request private/public behavior, but cannot override policy.
- No fallback path may silently widen visibility.

## BLOCKED under /private (hard list, not configurable)
- OpenClaw
- Anthropic (Claude)
- OpenAI (GPT-*)
- Google (Gemini)
- OpenRouter
- Any Hive node
- Any external endpoint not explicitly registered at birth

## ALLOWED under /private
- WebLLM (browser-local)
- Ollama (localhost only)
- Custom local endpoint (user-registered at birth, stored in vault)

## Timeout behavior
- On `/private` + timeout: HARD FAIL with explicit message.
- Never silently escalate to an external provider. Ever.