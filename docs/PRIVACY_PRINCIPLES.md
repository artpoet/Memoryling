# Privacy Principles

Memoryling's emotional value depends on access to sensitive context. Privacy is therefore a product behavior, not a legal footer.

## Commitments

### 1. Local-first by default

Approved source content and derived state stay on the user's device by default. No telemetry, cloud sync, remote model request, or network transport is currently implemented. The experimental Codex work-record pilot launches only a fixed local executable and communicates over local stdio; it does not make thread history equivalent to durable memory.

### 2. Explicit, narrow consent

Memoryling must explain which source it wants to read, which executable／account boundary is involved, and what categories of records will be imported. Consent to one source does not authorize another. The source v0.3.0 pilot permits one active approved source total and binds approval to a canonical scope hash; choosing another thread requires forgetting the current Memoryling copy and starting a fresh selection and consent flow.

The implemented schema-v1 consent scope covers one exact source, its source-specific adapter version, the allowlisted `user-confirmed-completion` category, the `local-creature-derivation` purpose, and consent／mapping versions. It authorizes only the selected import; automatic future-record intake, scope disable／re-enable, and content-derived growth remain future work. Another source requires another scope; a new category, purpose, or materially changed mapping requires a new consent revision. This never authorizes scanning another location or silently expanding use.

### 3. Read-only connectors

Source adapters may read only an explicitly selected, approved source and must not alter, delete, reorganize, or “repair” the source agent's files or threads. The Codex pilot is restricted to local stdio `thread/list` and `thread/read`; it cannot start, resume, rename, archive, delete, or otherwise mutate a thread.

### 4. Import preview

Before the experimental work-record scope is activated, users see the source kind, adapter and exact CLI-version boundary, data category, purpose, bounded count／time metadata, excluded categories, character count, and a content hash. They do not see thread title, path, raw identifier, prompt, response, tool output, or selected final-answer text. They may cancel before persistence and must explicitly confirm that the selected thread represents completed work. The current implementation does not automatically ingest later records or expand the scope.

The fixture path still shows fictional fixture text. The Codex path uses a content-free redacted preview: `thread/list` creates only a short-lived opaque catalog, and `thread/read` is permitted only after one explicit selection. No private thread has been read for UAT; that remains a separately authorized exact-source gate.

### 5. Source lineage

Every derived signal and world effect keeps machine-readable references to its source records and derivation version. For the Codex pilot, externally visible lineage is content-free: only opaque hashes, source kind, adapter／mapping versions, scope hash／revision, timestamps, and exclusion reasons may cross the backend boundary. “Why did this happen?” remains a required control without revealing private source text.

### 6. Complete forgetting

Deleting an imported source or record invalidates its dependent signals. Effects must be deleted or recomputed from the remaining sources.

“Complete” means complete within Memoryling's imported records and supported downstream graph. It does not authorize changing the source agent's files or threads and does not promise physically irrecoverable erasure from storage media, backups, or operating-system snapshots.

### 7. Bounded reminders

Reminder candidates do not become notifications automatically. Policy must enforce quiet hours, daily budgets, urgency thresholds, snooze state, and a global off switch.

### 8. No secrets in development artifacts

Real memories, tokens, credentials, private prompts, local databases, and identifiable logs must not appear in this repository, issues, CI, screenshots, or fixtures.

## Current local-source evidence

The implemented first-memory pipeline has two deliberately narrow paths:

- The fixture source is one fictional Codex-shaped JSON resource bundled with the app. It cannot scan arbitrary paths or read a user's Codex tool-home.
- Source v0.3.0 adds `codex-app-server-thread` v1 as an experimental work／thread-history pilot, not a durable-memory connector. OpenAI publishes no stable durable-memory export API or compatibility-guaranteed memory-file schema, so Memoryling does not parse `~/.codex/memories/` or any Codex-owned database, session, or rollout file.
- Rust resolves only the fixed standard Codex Desktop executable, requires exactly `codex-cli 0.134.0`, and permits only local stdio `thread/list` and `thread/read`. There is no WebSocket, model call, telemetry, cloud sync, background watcher, startup scan, or runtime network request.
- Listing is user-triggered and content-minimized. Raw thread IDs, titles, paths, prompts, responses, previews, and tool output remain Rust-only in a short-lived catalog. One explicit selection authorizes one read of the last completed turn's final answer; selected text never enters frontend IPC or the preview.
- Preview state is held in Rust process memory and bound to short-lived catalog／preview handles. Previewing or canceling does not persist selected source content; the desktop may still initialize an empty local schema.
- Explicit approval checks the canonical consent-scope hash, then stores the one selected normalized record, timestamps, hashes, adapter metadata, scope revision, and lineage in `memoryling.sqlite3` under Tauri's app-local data directory. SQLite schema v2 allows one approved source total.
- Derivation is deterministic and local: the supported user-confirmed completion record creates one completion signal and one completion-star effect. Text content, record volume, tokens, elapsed time, and tool activity do not alter its weight.
- Forgetting runs in a local transaction: it removes the imported source, consent scope, cascading normalized event, and downstream state, then recomputes from supported records that remain. The bundled fixture and original Codex thread are read-only and are not changed or deleted.
- CLI version verification and App Server work share one 10-second deadline. Output is size-bounded, stderr is not surfaced, and timeout／failure cleanup is bounded so a child process cannot create an indefinite UI wait.
- SQLite foreign keys and `secure_delete` are enabled, but this is an application-level deletion control, not a cryptographic secure-erasure guarantee.

Synthetic contract coverage and a content-free live `thread/list` smoke exist for the exact pinned CLI. That smoke did not select a thread or call `thread/read`. No user-owned durable memory has been imported, and no private thread has been read for UAT. The visible durable-memory access state must therefore remain off; separately authorized exact-source private UAT would validate only the pinned experimental work-record pilot, not a production connector.

Automatic ongoing derivation, scope disable／re-enable, additional sources, and broader growth mappings remain future design. The v0.3.0 one-import scope does not weaken the current preview, explicit approval, completion confirmation, or forgetting controls.

## Implemented pet-surface display boundary

The source v0.3.0 floating-pet shell enforces this privacy contract; extended live accessibility and screen-sharing acceptance remain open. The previously verified v0.2.0 installer remains the unchanged packaged baseline:

- The resident pet surface receives a whitelisted render-safe state, not the full memory or lineage DTO. It excludes normalized memory text, source paths or locators, private explanation content, and arbitrary record payloads.
- Exact per-window app-command permissions plus Rust caller-label checks deny fixture list／preview, Codex list／preview, cancel, full-state, approve, and forget commands from the pet surface. Production-ACL and independent caller-defense invoke tests cover all eight.
- Pet reactions, native menu items, tray labels, window titles, onboarding, and operating-system surfaces must remain neutral; they cannot reveal names, projects, traits, or source summaries.
- Cross-window events carry only opaque revisions or non-sensitive shell state. Each surface refetches a typed state limited to its purpose.
- Closing the detail window, hiding the pet, quitting the app, and forgetting a source are distinct operations and must never be described as equivalent deletion.
- The visible durable-memory-off status remains on the pet surface until a supported real-memory connector is verified. The experimental work-record pilot is labeled separately. Native labels are authoritative; browser mode keeps connector access off and does not imitate persistence or resident-window behavior.
- A screenshot／screen-sharing privacy mode and neutral growth-summary review are required before public testing with real memory-derived state.

## Before any network feature

Any feature that transmits memory-derived content must receive:

- a dedicated architecture decision record;
- a visible data-flow explanation;
- purpose-specific opt-in;
- a reviewable payload preview where practical;
- retention and deletion behavior;
- an offline or local-only path.

Until those conditions are met, the product must fail closed.
