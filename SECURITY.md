# Security Policy

Memoryling is pre-release software and must not yet be trusted with production-sensitive memory sources. Source v0.3.0 contains a version-bound experimental Codex work／thread-history pilot; it is not a production connector or a Codex durable-memory interface.

## Reporting a vulnerability

Please do not open a public issue for a suspected security or privacy vulnerability. Use GitHub's private vulnerability reporting feature under the repository's Security tab. Include reproduction steps, impact, affected versions, and any suggested mitigation.

Do not attach real agent memories, credentials, tokens, private prompts, or personal databases. Create the smallest synthetic reproduction possible.

## Current security boundary

- The verified v0.2.0 installer remains an unchanged fixture-only no-redo artifact. The v0.3.0 connector work is source-only: there is no accepted v0.3.0 installer and no private-thread UAT.
- OpenAI currently publishes no stable Codex durable-memory export API or compatibility-guaranteed file schema. Memoryling does not scan or parse `~/.codex/memories/`, Codex databases, session files, rollout files, or arbitrary user-supplied paths.
- Rust alone resolves the fixed standard local Codex Desktop executable and requires exactly `codex-cli 0.134.0`. The experimental adapter may issue only local App Server stdio `thread/list` and `thread/read`; it opens no WebSocket, model, telemetry, cloud, or runtime network boundary and performs no source write or mutation.
- Listing is explicitly user-triggered and content-minimized. Raw identifiers, titles, paths, prompts, responses, tool output, and selected final-answer text remain in Rust; the frontend receives only opaque handles and a redacted count／time／hash preview after one explicit selection.
- Approval is bound to a canonical consent-scope hash. SQLite schema v2 permits one approved source and keeps selected normalized text app-local; external lineage is content-free. Forgetting removes or recomputes Memoryling's local copy and downstream graph only, never the original Codex thread.
- CLI-version verification and the App Server request share one 10-second deadline. Output is capped, stderr is suppressed from product surfaces, and child-process cleanup is bounded after timeout or failure.
- The pet WebView receives a whitelisted render DTO and is denied all eight sensitive memory commands by both exact capabilities and independent Rust caller-label guards.
- No telemetry, cloud upload, remote model call, background source scan, or automatic synchronization is implemented.
- Any future production connector must be source-specific, read-only by default, based on a supported interface, and separately privacy-reviewed.
- Derived records must retain source lineage and support deletion or recomputation.
- Secrets and local memory databases are ignored by Git and must never enter fixtures.

Synthetic fixtures and a content-free live `thread/list` smoke cover the pinned compatibility boundary. That smoke did not select a thread or call `thread/read`. Access to any private thread requires separate exact-source authorization and must record only content-free pass／fail evidence.

Only the latest released version will receive security fixes once packaged releases begin.
