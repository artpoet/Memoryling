# ADR-0002: SQLite v1 local lineage store and fixture-only first-memory pilot

- Status: Accepted
- Date: 2026-08-10
- Related: [ADR-0001](0001-local-first-derived-memory.md)

## Context

ADR-0001 requires a local-first graph that preserves source lineage and supports downstream forgetting. The project needed to prove that path through the desktop UI, native boundary, persistence, derivation, explanation, and deletion without placing real agent memory in the repository or reading a user's tool-home.

The actual supported format and source-selection UX for user-owned Codex durable memory have not been validated. Treating private files or the current machine's memory layout as a public connector specification would widen the trust boundary prematurely.

## Decision

Memoryling will implement the first vertical slice with these limits:

1. **Synthetic fixed source.** The adapter reads exactly one fictional Codex-shaped JSON resource bundled with the app. Its source ID, locator, format version, record kind, and size are constrained. It is fixture infrastructure, not a production Codex connector.
2. **Rust-owned preview.** The WebView selects a declared source ID, not a filesystem path. Parsed preview data remains in Rust process memory and receives a pending token. Approval must present that token, the matching source ID, and selected record IDs.
3. **Explicit persistence gate.** Previewing or canceling does not persist source content. Approval stores only selected normalized records and their source contract, hashes, timestamps, adapter version, and lineage.
4. **SQLite schema v1.** The desktop uses bundled `rusqlite` and stores `memoryling.sqlite3` under Tauri's app-local data directory. Migration 0001 creates source imports, memory events, derived signals, world effects, and their lineage joins. `PRAGMA user_version = 1` identifies the schema; unknown versions fail closed.
5. **Deterministic derivation.** Schema v1 accepts one `completion` event kind. Stable IDs and content hashes use SHA-256. Each supported event produces one versioned completion signal and one active completion-star world effect.
6. **Transactional forgetting.** Forgetting clears derived state, deletes the local source and cascading events, and deterministically re-derives effects from supported records that remain in one transaction. It never writes to or deletes the source fixture.
7. **Local-only runtime boundary.** This path adds no telemetry, network client, cloud sync, remote model call, arbitrary filesystem access, or browser fallback. Real-memory access remains visibly off until a production connector and consent flow are verified.

SQLite foreign keys and `secure_delete` are enabled for each connection. They support application-level integrity and deletion but do not constitute a cryptographic or physically irrecoverable secure-erasure guarantee.

## Consequences

### Positive

- The project can verify preview → approval → persistence → derivation → explanation → forgetting without using private data.
- The local schema, native command boundary, and UI are exercised together instead of ending at a connector skeleton.
- Stable lineage and deterministic derivation make restart and deletion behavior testable.
- A future real connector can reuse the local pipeline while remaining a separately reviewable trust-boundary change.

### Costs and limits

- The current adapter says nothing authoritative about a real Codex durable-memory format or tool-home layout.
- The current runtime exposes only one fixture source and one `completion` derivation path.
- Approved normalized text is sensitive once real sources exist, so the SQLite database must never be printed, committed, attached to issues, or exposed in screenshots.
- Future schema versions require explicit migrations and compatibility tests.
- Application-level deletion cannot control external backups, snapshots, or storage-media recovery behavior.

## Rejected alternatives

- **Read the current machine's Codex tool-home immediately:** rejected because it would use private implementation details as a specification and bypass an explicit real-source review.
- **Let the WebView submit arbitrary filesystem paths:** rejected because it broadens access beyond the single approved fixture and weakens the native trust boundary.
- **Use browser local storage for memory records:** rejected because it would not prove native persistence, migrations, transactional lineage deletion, or restart behavior.
- **Store only the final creature mark:** rejected because it would break explanation and complete downstream forgetting.

## Privacy impact

Repository and automated-test data remain fictional. The implemented path cannot discover or read user-owned Codex memory. After approval, normalized fixture text and lineage are stored only in app-local SQLite; forgetting removes that local imported copy and supported downstream records while leaving the read-only bundled fixture unchanged.

Any production connector for real Codex memory must retain explicit source selection, read-only access, preview and consent, fail-closed parsing, local lineage, and the visible real-memory-off boundary until verified. It may require a follow-up ADR if it changes the trust model or data flow.
