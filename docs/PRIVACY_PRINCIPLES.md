# Privacy Principles

Memoryling's emotional value depends on access to sensitive context. Privacy is therefore a product behavior, not a legal footer.

## Commitments

### 1. Local-first by default

Approved source content and derived state stay on the user's device by default. There is no telemetry or cloud sync. The ordinary pet, fixture pipeline, Codex Agent-memory connector, and supplementary work-record pilot remain local-only. Daily Memory Scout is the separate exception: it may send a visible coarse summary only from its separately approved work-record source after purpose-specific opt-in. Agent-memory documents are categorically ineligible.

### 2. Explicit, narrow consent

Memoryling must explain which source it wants to read and what categories and purposes are authorized. Consent to one source does not authorize another. Source v0.5.0 still permits one active approved source total. The primary Agent-memory consent binds the exact root hash, adapter version, two allowlisted document categories, local derivation, and automatic read-only sync; choosing another source requires forgetting the current Memoryling copy and starting a fresh preview and consent flow.

Schema-v2 Agent-memory consent authorizes future changes only inside the same exact source and two-file allowlist. Fixture and work-record consent remain schema v1 and import-bound. Another source, changed root, new category or purpose, or materially changed mapping requires a new consent revision. No scope authorizes broader Codex-home scanning, external AI use, or silent purpose expansion.

### 3. Read-only connectors

Source adapters may read only an explicitly selected, approved source and must not alter, delete, reorganize, or “repair” the source agent's files or threads. The primary Codex Agent-memory adapter checks only the two top-level files approved by ADR-0007. The supplementary Codex work-record pilot is restricted to local stdio `thread/list` and `thread/read`; it cannot start, resume, rename, archive, delete, or otherwise mutate a thread.

### 4. Import preview

Before Agent-memory scope activation, users see the source kind, adapter, two-file allowlist, purposes, bounded count／time metadata, character counts, hashes, and automatic-sync behavior. They never see raw Agent-memory text or local paths. The supplementary work-record path keeps its exact CLI-version and one-record completion confirmation. Both paths may be canceled before persistence.

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

The source v0.5.0 pipeline has three narrow paths:

- **Primary Agent memory:** `codex-local-memory-store` v1 checks only top-level `memory_summary.md` and `MEMORY.md` under the configured Codex `memories` root. It does not enumerate rollout summaries, sessions, databases, evidence, prompts, logs, or arbitrary files.
- **Synthetic fixture:** one repository-visible fictional JSON resource for safe end-to-end testing.
- **Supplementary work record:** `codex-app-server-thread` v1 lists neutral candidates and reads one explicitly selected completed thread through exact `codex-cli 0.134.0` local stdio.

The Agent-memory root and files must be non-symlink regular local paths. Each file is capped at 2 MiB and combined input at 4 MiB; invalid UTF-8, empty files, unsafe paths, and changed source roots fail closed. Preview state stays in Rust and exposes only logical IDs, timestamps, character counts, and hashes.

After exact schema-v2 consent, source text is stored only in `memoryling.sqlite3` under Tauri app-local data. Startup, a 15-minute in-process interval, and `Sync now` check only that approved source. A successful change transactionally replaces the local events and recomputes one aggregate memory-continuity signal and halo. A missing source withdraws events and effects until recovery; unsafe input preserves the last valid state and reports attention needed.

Raw Agent-memory and selected work-record text never enters frontend IPC, logs, pet DTOs, native labels, repository fixtures, or Daily Scout. Daily Scout's compiler explicitly accepts only its separately approved `codex-app-server-thread` source. Forgetting removes Memoryling's local source, scope, sync state, events, lineage, and effects, never the Codex originals.

SQLite schema v4 enables foreign keys and `secure_delete`; deletion remains an application-level control, not a cryptographic secure-erasure promise. Automated tests use temporary synthetic memory files. No private Agent memory or thread has been read for UAT in this source milestone.

## Implemented pet-surface display boundary

The source v0.5.0 floating-pet shell enforces this privacy contract; extended live accessibility and screen-sharing acceptance remain open. The previously verified v0.2.0 installer remains the unchanged packaged baseline:

- The resident pet surface receives a whitelisted render-safe state, not the full memory or lineage DTO. It excludes normalized memory text, source paths or locators, private explanation content, and arbitrary record payloads.
- Exact per-window app-command permissions plus Rust caller-label checks deny all fixture／Codex content commands and all ten Daily Scout settings, credential, network, external-link, full-insight, and deletion commands from the pet surface. Production-ACL and independent caller-defense invoke tests cover the full sensitive-command manifest.
- Pet reactions, native menu items, tray labels, window titles, onboarding, and operating-system surfaces must remain neutral; they cannot reveal names, projects, traits, or source summaries.
- Cross-window events carry only opaque revisions or non-sensitive shell state. Each surface refetches a typed state limited to its purpose.
- Closing the detail window, hiding the pet, quitting the app, and forgetting a source are distinct operations and must never be described as equivalent deletion.
- The pet shows `codex-local` only after Agent-memory consent and while approved local events are available. A missing source withdraws the halo and returns visible access to off. The supplementary work-record pilot stays separately labeled; browser mode always keeps connector access off.
- A screenshot／screen-sharing privacy mode and neutral growth-summary review are required before public testing with real memory-derived state.

## Before any network feature

Any feature that transmits memory-derived content must receive:

- a dedicated architecture decision record;
- a visible data-flow explanation;
- purpose-specific opt-in;
- a reviewable payload preview where practical;
- retention and deletion behavior;
- an offline or local-only path.

Source v0.5.0 preserves these design conditions for the bounded Daily Scout path through ADR-0006 and explicitly excludes Agent-memory documents. Any broader provider, data category, purpose, tool, endpoint, or automatic-action boundary must fail closed pending a new review.
