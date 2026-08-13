# Privacy Principles

Memoryling's emotional value depends on access to sensitive context. Privacy is therefore a product behavior, not a legal footer.

## Commitments

### 1. Local-first by default

Approved source content and derived state stay on the user's device by default. There is no telemetry or cloud sync. The ordinary pet, fixture pipeline, and experimental Codex work-record pilot remain local-only. Source v0.4.0 adds one separate exception: Daily Memory Scout may send a visible, coarse work-context summary to OpenAI only after purpose-specific opt-in and only while the feature remains enabled.

### 2. Explicit, narrow consent

Memoryling must explain which source it wants to read, which executable／account boundary is involved, and what categories of records will be imported. Consent to one source does not authorize another. The source v0.4.0 pilot permits one active approved source total and binds approval to a canonical scope hash; choosing another thread requires forgetting the current Memoryling copy and starting a fresh selection and consent flow.

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

## Optional Daily Memory Scout boundary

Daily Memory Scout is off by default and is not required for the ordinary pet. Enabling it requires a user-supplied OpenAI API key, a visible preview of the exact coarse context categories, a daytime setting, and explicit consent to automatic daily transmission. The separate consent contract fixes the provider, model, source IDs, compiler version, purpose, data categories, 3,000-character cap, and automatic-send behavior. A supporting-source change invalidates that consent.

The native first-launch flow may optionally save that key in Windows Credential Manager, but it does not test the key, enable Daily Scout, transmit context, or perform Web Search. First launch records only a content-free completion bit alongside shell preferences. Local-only remains the recommended default, and the later context-preview／consent gate is still mandatory before automatic daily transmission.

The only outbound content is a deterministic allowlist of coarse work domains, public tool／model names, generic goals, non-sensitive constraints, an evidence date range, and preferred insight categories. The compiler reads only an approved `codex-app-server-thread` record. It never copies prompts, final-answer text, paths, repository URLs, thread IDs, customer or person names, tool output, credentials, or arbitrary phrases; the bundled synthetic fixture is ineligible.

The API key is written to Windows Credential Manager through Rust and is never returned to frontend IPC, stored in SQLite／JSON／localStorage, or logged. All API calls are Rust-only and use the fixed official OpenAI endpoint, pinned `gpt-5.6-terra`, `store: false`, and only the Web Search tool. The WebView cannot choose the endpoint, headers, model, tools, or instructions. `store: false` avoids application-state storage but does not promise zero provider retention: ordinary OpenAI API abuse-monitoring data may still be retained for up to 30 days, and the user's API account bears the cost.

At most one attempt is reserved per local calendar date in SQLite before any provider call. A failure is recorded and is not automatically retried that day. The app does not install a background service or replay missed dates. Returned text is bounded and displayed through React escaping; links must be HTTPS citations extracted from Web Search annotations, never URLs invented in model prose. Web pages are untrusted data and cannot trigger local reads, commands, other tools, account login, or external writes.

Turning the feature off stops future attempts but may retain local insight history. Users can separately clear local insights, delete the key, or reset all three. Forgetting a supporting source deletes its dependent insights and disables the consent in the same local transaction. These controls cannot delete provider-side abuse-monitoring data already retained under OpenAI policy.

## Current local-source evidence

The implemented first-memory pipeline has two deliberately narrow paths:

- The fixture source is one fictional Codex-shaped JSON resource bundled with the app. It cannot scan arbitrary paths or read a user's Codex tool-home.
- Source v0.4.0 includes `codex-app-server-thread` v1 as an experimental work／thread-history pilot, not a durable-memory connector. OpenAI publishes no stable durable-memory export API or compatibility-guaranteed memory-file schema, so Memoryling does not parse `~/.codex/memories/` or any Codex-owned database, session, or rollout file.
- Rust resolves only the fixed standard Codex Desktop executable, requires exactly `codex-cli 0.134.0`, and permits only local stdio `thread/list` and `thread/read`. That connector has no WebSocket, model call, telemetry, cloud sync, background watcher, startup scan, or network request; Daily Memory Scout is the separately consented network boundary above.
- Listing is user-triggered and content-minimized. Raw thread IDs, titles, paths, prompts, responses, previews, and tool output remain Rust-only in a short-lived catalog. One explicit selection authorizes one read of the last completed turn's final answer; selected text never enters frontend IPC or the preview.
- Preview state is held in Rust process memory and bound to short-lived catalog／preview handles. Previewing or canceling does not persist selected source content; the desktop may still initialize an empty local schema.
- Explicit approval checks the canonical consent-scope hash, then stores the one selected normalized record, timestamps, hashes, adapter metadata, scope revision, and lineage in `memoryling.sqlite3` under Tauri's app-local data directory. SQLite schema v3 still allows one approved source total and adds Daily Scout settings, attempt-ledger, insight-lineage, and citation tables.
- Derivation is deterministic and local: the supported user-confirmed completion record creates one completion signal and one completion-star effect. Text content, record volume, tokens, elapsed time, and tool activity do not alter its weight.
- Forgetting runs in a local transaction: it removes the imported source, consent scope, cascading normalized event, and downstream state, then recomputes from supported records that remain. The bundled fixture and original Codex thread are read-only and are not changed or deleted.
- CLI version verification and App Server work share one 10-second deadline. Output is size-bounded, stderr is not surfaced, and timeout／failure cleanup is bounded so a child process cannot create an indefinite UI wait.
- SQLite foreign keys and `secure_delete` are enabled, but this is an application-level deletion control, not a cryptographic secure-erasure guarantee.

Synthetic contract coverage and a content-free live `thread/list` smoke exist for the exact pinned CLI. That smoke did not select a thread or call `thread/read`. No user-owned durable memory has been imported, and no private thread has been read for UAT. The visible durable-memory access state must therefore remain off; separately authorized exact-source private UAT would validate only the pinned experimental work-record pilot, not a production connector.

Automatic ongoing creature derivation, source-scope disable／re-enable, additional sources, and broader growth mappings remain future design. The v0.4.0 import scope does not weaken the current preview, explicit approval, completion confirmation, or forgetting controls; Daily Scout uses a distinct purpose-specific consent.

## Implemented pet-surface display boundary

The source v0.4.0 floating-pet shell enforces this privacy contract; extended live accessibility and screen-sharing acceptance remain open. The previously verified v0.2.0 installer remains the unchanged packaged baseline:

- The resident pet surface receives a whitelisted render-safe state, not the full memory or lineage DTO. It excludes normalized memory text, source paths or locators, private explanation content, and arbitrary record payloads.
- Exact per-window app-command permissions plus Rust caller-label checks deny all fixture／Codex content commands and all ten Daily Scout settings, credential, network, external-link, full-insight, and deletion commands from the pet surface. Production-ACL and independent caller-defense invoke tests cover the full sensitive-command manifest.
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

Source v0.4.0 satisfies these design conditions for the bounded Daily Scout path through ADR-0006, the visible context／consent UI, local history and key controls, and the unchanged ordinary-pet path. Any broader provider, data category, purpose, tool, endpoint, or automatic-action boundary must fail closed pending a new review.
