# Architecture

## Status

This document separates the intended product architecture from the subset implemented as of 2026-08-13. The source tree is now v0.5.0: it adds the one-time approved, two-file Codex Agent-memory source as the primary local path while retaining the v0.2.0 fixture pipeline, the supplementary version-bound work-record pilot, and the separately consented BYOK Daily Memory Scout. The Agent-memory adapter is versioned and fail-closed because its generated-file schema is not a stable third-party contract. The verified v0.2.0 installer remains the packaged no-redo baseline; no v0.5.0 installer, private-source UAT, or paid live API smoke is claimed.

## System shape

    Explicitly approved local sources
        ├─ Codex Agent-memory root (primary; two-file allowlist)
        ├─ bundled fictional fixture
        └─ experimental Codex work／thread history (supplementary)
        → read-only source adapters
        → import preview and consent gate
        → normalized local memory events
        → derivation engine
        → lineage-aware local store
        ├─→ creature state, stories, conversations, reminders
        └─→ optional coarse-context compiler → OpenAI Web Search → cited daily insight
        → bilingual Tauri UI

The primary Agent-memory path aggregates all currently available allowlisted documents into one deterministic continuity signal and one memory halo. The fixture and work-record paths still produce one user-confirmed completion event and one completion-star effect. The fixture is test infrastructure; the work-record path is a source-only compatibility pilot around an experimental local host, not Agent-memory access or production-readiness evidence.

## Layers

| Layer | Responsibility | Current state |
|---|---|---|
| Desktop shell | Native window, lifecycle, notifications | 0.2.0 pre-creates transparent `pet` and hidden `main` windows; Rust owns native menu, tray, single-instance recovery, show／hide／focus, close／minimize／restore compensation, explicit Quit, and pet position recovery |
| Experience UI | Creature, habitat, stories, controls, explanations | Bilingual pet and detail surfaces plus primary Agent-memory consent／sync, supplementary fixture／work-record flows, redacted preview, lineage, forgetting, Daily Scout controls, onboarding, reduced-motion handling, and honest connected／missing／off states |
| Source adapters | Read an explicitly approved local source without mutating it | Primary `codex-local-memory-store` v1 reads exactly two generated files after source consent; fixture v1 reads one fixed JSON resource; supplementary `codex-app-server-thread` v1 uses bounded local App Server stdio behind an exact CLI pin |
| Import gate | Preview scope, explain access, obtain consent | Implemented for all three paths; Agent-memory preview approves the complete allowlisted source once, while fixture records and one work record retain their narrower flows; private content stays in Rust |
| Normalizer | Convert source records into a versioned local event schema | Event schema v1 accepts `agent-memory-document` and `completion` under adapter-specific validation |
| Derivation engine | Produce traits, tensions, story hooks, reminder candidates | Deterministic aggregate Agent-memory continuity → memory halo plus completion → completion star |
| Local store | Persist normalized events, consent scopes, derived effects, lineage, and settings | SQLite schema v4 stores one approved source, source-sync status, derived lineage, and Daily Scout state under Tauri app-local data; the OpenAI key stays outside SQLite in Windows Credential Manager |
| Daily Scout | Compile minimized approved-work context and optionally fetch one cited insight | Source v0.4.0 boundary retained: Rust-only fixed OpenAI Responses API／Web Search, pinned model, `store: false`, one attempt per local date, and an app-running scheduler; off by default and Agent-memory-ineligible |
| Conversation layer | Ground dialogue in approved local context | Not implemented; provider decision open |
| Reminder policy | Enforce quiet hours, budget, urgency, and snooze state | UI concept only |

## Optional Daily Memory Scout in source v0.4.0

Proposed [ADR-0006](adr/0006-optional-byok-daily-memory-scout.md) defines this separate online path. It is available only after the user saves a BYOK OpenAI key, reviews the outbound summary, accepts the purpose-specific consent, and chooses a time from 08:00 through 21:59. The ordinary pet works unchanged with no key and no network request.

1. The local compiler accepts only the one approved `codex-app-server-thread` source. It scans at most 12 normalized completion events for a fixed keyword allowlist and emits only coarse work domains, public tool／model labels, generic goals, non-sensitive constraints, evidence dates, and fixed insight categories. It never emits raw normalized text, prompt／answer content, path, repository URL, record or thread ID, person／client name, credential, or arbitrary extracted phrase. Synthetic fixture data is ineligible.
2. The detail UI displays that exact coarse context before enablement. Consent binds provider `openai`, model `gpt-5.6-terra`, source IDs, categories, purpose, compiler version, automatic daily send, and a 3,000-character maximum. A context or source-scope change disables the feature until renewed consent.
3. The API key crosses frontend IPC once as a password-field command, is immediately cleared from React state, and is stored through `keyring-core` in Windows Credential Manager. It is never returned to the WebView or placed in SQLite, JSON settings, localStorage, logs, fixtures, or repository state.
4. Rust owns a fixed `https://api.openai.com/v1/responses` client. The request pins `gpt-5.6-terra`, `store: false`, low reasoning／verbosity, one required `web_search` tool, output limits, and instructions treating webpages as untrusted. The WebView cannot select the endpoint, authorization header, model, tools, or prompt.
5. A SQLite immediate transaction reserves one attempt for the current local date before the HTTP call. The monotonic date guard prevents duplicate paid searches across restarts, retries, or clock rollback. Failure consumes the date and is shown honestly; missed days are not replayed. A lightweight in-process scheduler checks after startup and every 15 minutes only while Memoryling runs.
6. A result is accepted only if the response contains a completed Web Search call, bounded message text, and at least one HTTPS `url_citation` annotation. Up to three annotation-derived links are persisted. Model-written plaintext URLs do not become links. Rust opens only the two fixed OpenAI setup URLs or an exact URL already persisted in the citation table; no general opener capability is exposed to the WebView. The floating pet receives only `off`／`waiting`／`ready`; full message and citations stay on `main`.
7. Turning off stops future attempts without forcing history deletion. Clear-history, delete-key, and full reset are separate controls. Forgetting the supporting source deletes dependent insights and disables Daily Scout in the same transaction; the attempt ledger remains so deletion cannot buy a second search that day.

The current proof is synthetic and content-free: OpenAI response parsing, citation rejection, context minimization, schema migration, credential abstraction, success／failure once-per-date behavior, ACL separation, and bilingual UI flows have automated coverage. A real API request would be paid and may involve ordinary OpenAI abuse-monitoring retention, so it remains an explicit acceptance gate rather than an inferred pass.

## Primary Codex Agent-memory connector introduced in source v0.5.0

Accepted [ADR-0007](adr/0007-codex-agent-memory-auto-sync.md) makes the current local Codex Agent-memory store the primary source. The user chooses the source once and approves schema-v2 consent for the exact root, adapter v1, two data categories, local derivation, and automatic read-only sync. Individual memory documents are not separately selected.

1. Rust resolves `CODEX_HOME\memories` or `%USERPROFILE%\.codex\memories`; consent stores only a hash of that exact root. UI and SQLite receive the generic `codex-home://memories` locator.
2. The adapter checks only top-level `memory_summary.md` and `MEMORY.md`. It does not enumerate rollout summaries, sessions, evidence, databases, prompts, logs, or other Codex-home content. Root and file symlinks, non-regular files, invalid UTF-8, empty files, files over 2 MiB, and combined input over 4 MiB fail closed.
3. Preview contains logical document IDs, modified timestamps, character counts, and hashes only. Raw text never enters frontend IPC, pet DTOs, native labels, logs, fixtures, or screenshots.
4. Approval requires the exact scope hash and complete allowlisted document set. Migration 0004 advances SQLite to schema v4, expands event／signal／effect allowlists, and adds `source_sync_state`.
5. Startup, a 15-minute in-process interval, and `Sync now` re-read only an already approved source. A successful change transactionally replaces local events and recomputes one aggregate `agent-memory-continuity` signal and `memory-halo` effect with complete lineage.
6. If the same approved source disappears, its local events and effects are withdrawn while consent remains for recovery. Invalid, oversized, unreadable, or location-changed input keeps the last valid state and reports `needs-attention`. Forgetting deletes the Memoryling-local source, scope, sync state, lineage, and effects but never changes Codex.
7. Daily Scout's compiler remains restricted to `codex-app-server-thread`; Agent-memory documents can never enter its outbound context.

Official documentation describes the current local memory location and generated artifacts but does not promise a stable third-party schema. Adapter-v1 filenames are therefore a fail-closed product contract, not an OpenAI compatibility guarantee. Automated evidence uses synthetic temporary files only; private-memory UAT requires separate exact authorization.

## Supplementary Codex work-record pilot introduced in source v0.3.0

Official OpenAI documentation exposes the documented `thread/list` and `thread/read` method names without requiring the opt-in `experimentalApi` capability, but it still labels the overall App Server command／transport experimental and unsupported for production. Memoryling therefore treats this integration as a version-bound work／thread-history pilot, never as durable-memory access. The supporting evidence and decision boundary are recorded in the [Codex source-format evaluation](research/2026-08-12_codex-source-format-evaluation.md) and proposed [ADR-0005](adr/0005-codex-thread-history-source-pilot.md).

The implementation boundary is deliberately narrow:

1. Rust alone resolves and launches the fixed standard Codex Desktop executable under `%LOCALAPPDATA%\Programs\OpenAI\Codex\bin\codex.exe`. No WebView or caller can provide an executable, path, argument, transport, or method name. The command must report exactly `codex-cli 0.134.0`; every other version fails closed.
2. Listing starts only after a user action. `thread/list` becomes a short-lived, content-minimized catalog with generic labels, source kind, and bounded time metadata. Raw thread IDs, titles, paths, previews, prompts, responses, and tool output remain Rust-only and are neither rendered nor persisted by the catalog.
3. The user selects exactly one candidate. Only then may Memoryling issue one `thread/read` with turns, reject an active or malformed thread, and take the final `agentMessage` with phase `final_answer` from the last completed turn. Completion is still explicitly confirmed by the user; text is not used to infer success, personality, topic, or weight.
4. The preview is redacted and content-free. Frontend IPC receives counts, time bounds, exclusions, and a content hash／character count—not the selected final-answer text. Approval is bound to the canonical consent-scope JSON and its 64-hex hash before the normalized completion record can enter app-local SQLite.
5. Migration 0002 advances the local store to `PRAGMA user_version = 2` and adds `source_consent_scopes`. The current schema allows one active approved source total. A new thread requires forgetting the current Memoryling source and completing a fresh list → select → preview → consent flow.
6. External lineage keeps only redacted or opaque hashes, adapter／mapping versions, scope hash, consent revision, timestamps, and exclusion reasons. Approved normalized text stays only in the local Memoryling database and never crosses frontend IPC, logs, pet DTOs, native labels, notifications, or repository fixtures.
7. Forgetting deletes or recomputes Memoryling's local source, event, signal, effect, explanation, render state, cache, and lineage. It never edits, archives, deletes, resumes, or otherwise mutates the original Codex thread.
8. The `main` capability and independent Rust caller-label guard protect all nine sensitive memory commands, including Agent-memory sync. `pet` cannot invoke any of them.
9. CLI-version verification and the App Server request share one 10-second operation deadline. Stdout size and line length are capped, stderr is not surfaced, and timeout／failure uses bounded child-process cleanup rather than an unbounded wait. The adapter uses local stdio only and opens no WebSocket, model, telemetry, cloud, or network boundary.

Synthetic contract tests and a content-free live `thread/list` compatibility smoke cover the implemented boundary. The live smoke did not select a candidate or call `thread/read`. Access to one real private thread remains a separate exact-source authorization and UAT gate; until that passes, ADR-0005 stays Proposed and durable-memory access remains visibly off.

## Implemented pet-first desktop shell

The user-confirmed product direction is “two surfaces, one life,” recorded in proposed [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md) and the detailed [pet-first desktop shell draft](drafts/pet-first-desktop-shell-2026-08-11.md). The 0.2.0 implementation now follows this architecture, but ADR-0003 remains **Proposed** until the remaining live Windows DPI, multi-monitor, hitbox, accessibility, and session-lifecycle gates pass.

```text
one Tauri process
  ├─ pet window: transparent, undecorated, render-safe creature state only
  ├─ main window: hidden until requested, full detail and lineage controls
  ├─ native context menu + tray + single-instance recovery
  └─ Rust lifecycle + canonical SQLite state
```

Both WebViews are pre-created hidden; Rust setup shows only `pet`, avoiding handler-time WebView construction. The pet is transparent, undecorated, skip-taskbar, non-closable, always-on-top by default, and resizes from the 360 × 430 logical onboarding envelope to the 320 × 320 compact envelope while preserving its screen anchor. Rust owns show, hide, focus, `main` `CloseRequested` interception, pending-preview reset, native menu, tray, position recovery, and explicit Quit. Lifecycle transitions use compensating rollback so a failed second window operation leaves one recoverable surface rather than two visible windows or none.

The build uses `tauri_build::AppManifest::commands` and exact local-only `main`／`pet` capabilities; neither surface inherits `core:default`, remote scopes, wildcards, or deny-pattern ambiguity. Fixture／Codex content commands and all Daily Scout management／network commands are `main`-only and also require a `MainCaller` whose WebView and native-window labels both match. `pet` receives only render-safe state, shell state, menu, onboarding, and its guarded drag command plus event listen／unlisten. The drag command acts only on the caller's pet window, so pet JavaScript cannot select and drag `main` through a generic core window API. Production-authority and empty-authority invoke harnesses prove both ACL and caller-label denial across the full sensitive-command manifest; a `main` list invoke is the positive control.

`CreatureRenderState` v5 contains only bounded appearance parameters, coarse `off`／`codex-local` access, opaque completion-star／memory-halo mark IDs, and a 64-hex revision. Approve, sync, and forget emit the same content-free `{revision}` notification to both surfaces, which then refetch their typed state; event-delivery failure does not roll back a committed memory transaction. No memory text, path, locator, explanation, source identity, or content hash enters pet IPC, native menu labels, tray labels, window titles, or operating-system notifications. Closing details cancels any pending preview in Rust before hiding because hiding a WebView does not unmount it; minimizing preserves the preview.

Right-click is the primary entry. When `pet` has focus, Enter／Space／Menu key／`Shift+F10` invokes the same native menu at a fixed pet anchor. Native menu, transparent pet, onboarding, one-detail-window lifecycle, minimize／restore, single-instance recovery, and explicit native Quit passed the core Windows smoke; position persistence／clamp and tray actions have automated coverage. A normal Explorer-launched current-user NSIS install and the actual installed Start shortcut passed both cold launch and resident single-instance relaunch: the second launch focused the existing detail surface without creating another process. Direct tray-action and `Win+B` traversal remain pending live acceptance.

Pet position is stored in a content-free JSON record with monitor identity, work-area dimensions, logical coordinates, normalized coordinates, and scale factor. Rust restores and clamps it on launch and recovery, coalesces move／scale persistence, and polls monitor work-area topology. Pure geometry tests cover negative origins, taskbar work-area offsets, oversized windows, removed-monitor fallback, and logical 320-pixel sizing at 100／125／150／200%; live 125–200% and mixed-DPI movement, hot-unplug, and taskbar relocation remain acceptance work. Browser mode continues to show the detail preview and does not imitate native floating-window behavior.

### 0.2.0 verification snapshot

- Historical v0.2.0 automated evidence: 23 frontend tests and 29 Rust tests passed. That snapshot covered concurrent first-open migration, lifecycle compensation, position／anchor recovery, content-minimized DTOs, exact capabilities, and the then-current six-command denial layers. Source v0.3.0 extended both layers to eight memory commands; source v0.4.0 also covers all ten Daily Scout commands. This current evidence remains separate from the unchanged installer baseline.
- Native and packaged evidence: transparent pet／first-run onboarding, pointer and focused-keyboard native menu paths, close／minimize／restore, single-instance recovery, explicit native Quit, raw movement／second-monitor observation, and core pet／main state transitions pass on the current Windows host. Tray actions and position recovery have automated evidence; their remaining live matrix is not inferred from that.
- Fixture evidence: raw bundled fixture preview and approval, restart persistence, source → event → signal → completion-star lineage, cross-surface state, and complete forgetting pass; no real source was used.
- Installer evidence: a normal Explorer-launched NSIS current-user install, actual installed Start shortcut cold launch and resident relaunch, explicit Quit, and uninstall with retained app data pass. The retained files were checked only as local app-data state, not committed or printed.
- Artifact: `Memoryling_0.2.0_x64-setup.exe`, 2,875,965 bytes, SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`, `FileVersion`／`ProductVersion` 0.2.0, `NotSigned`.
- Harness trap: an early agent-direct installer launch produced Windows virtualization behavior and is not valid product evidence or a product failure. Acceptance uses normal Explorer and installed-shortcut paths.
- Still pending before ADR-0003 acceptance: live 125–200%／mixed-DPI testing, hot-unplug／taskbar relocation, adjacent-desktop hitbox probing, `Win+B`, Narrator／NVDA, sign-out／shutdown, and compact／wide／tall／long growth-envelope coverage. WebView2-missing bootstrapper testing remains deferred. The source-only work-record pilot has separate private-UAT and production-support gates; creature growth has not started.

## Implemented v1 records and future shape

### Source import and memory event

- `source_imports` retains the adapter ID and version, display label, fixed locator, and source-content hash.
- `memory_events` retains schema version 1, an opaque source-record ID, source and observed timestamps, the approved normalized text, and its content hash.
- Event schema v1 accepts `completion` and `agent-memory-document` only through their matching adapters; future record kinds require explicit schema and derivation work.

### DerivedSignal

- The implemented type is `completion`, with confidence and derivation version 1.
- `derived_signal_sources` links every signal to its memory event.
- Recurrence, promise, value, conflict, preference, and relationship signals remain future work.

### WorldEffect

- The implemented type is one active `visual-mark` with style `completion-star`.
- `world_effect_signals` links the mark to its signal, and an explanation key supports the lineage inspector.
- Traits, habitat changes, story events, dialogue facts, and reminder candidates remain future work.

The complete source → event → signal → effect graph is queried back from SQLite for the “Why did this happen?” view.

## Future creature-growth boundary

Everything in this section is a future Phase 2 proposal unless explicitly identified as current fixture, Agent-memory connector, work-record pilot, or Daily Scout behavior. Source v0.5.0 implements automatic future updates only inside the unchanged schema-v2 Codex Agent-memory scope. It does not authorize source expansion, live Agent observation, A／B／C evidence classification, outcome-group accumulation, or morphology compilation. Daily Scout retains separate consent and excludes Agent-memory text.

### Future expanded source-consent scope

The implemented Agent-memory schema-v2 `SourceConsentScope` binds one explicit consent to one exact root hash, adapter version, two allowlisted document categories, local derivation, and automatic read-only sync. Fixture and work-record scopes remain schema v1 and import-bound. Another source, a new category or purpose, or materially changed mapping semantics must stop at a new scope-revision preview and fresh consent.

Disabling and re-enabling a consent scope are not implemented. The future rule remains that disabling a scope makes its evidence ineligible for active derivation and triggers the same deterministic downstream recomputation as forgetting or correcting evidence; it never writes to or deletes the source. That lifecycle and retention UX requires synthetic acceptance before any broader private-data testing.

The user-confirmed, not-yet-implemented product direction has the following proposed deterministic shape:

    approved events → derived signals
        ├─→ path contributions with lineage → deterministic EvolutionPathProfile ─┐
        ├─→ structural growth contributions ─────────────────────────────────────┴─→ recomputable CreatureGenome
        └─→ WorldEffects → marks / habitat / story projections

    IdentityCore + stage + CreatureGenome + versioned local module catalog
        → bounded deterministic MorphologyRecipe + EvolutionBridge

    MorphologyRecipe + active WorldEffects + EphemeralState
        → render-safe CreatureState → local layered renderer

Permanent growth may substantially change morphology. Distant stages do not need to remain immediately recognizable as the same form, but every adjacent stage transition must produce a versioned `EvolutionBridge` describing preserved, grown, split, merged, relocated, or retired traits.

The proposed evidence responsibilities are deliberately asymmetric:

| Evidence lane | Permitted permanent or persistent influence | Hard boundary |
|---|---|---|
| **A — authorized Agent-use behavior** | Primary morphology: silhouette, proportions, major organic modules, and posture family through `EvolutionPathProfile` | It cannot advance maturity by elapsed use time or select a fixed pre-authored pet |
| **C — authorized history outcomes** | Maturity and lineage-bearing `WorldEffect` marks; multiple deduplicated independent outcome-qualified canonical groups gate whether an A-shaped permanent structural reshape may commit | It cannot select or override the main silhouette; one completion, repeated copies, source count, raw record volume, calendar time, tokens, or open hours cannot act as XP |
| **B — authorized collaboration mode** | Bounded local pose, gesture, and continuing-motion tendencies | It cannot select the main silhouette, stage, or maturity |
| **Approved content domain** | At most a bounded secondary material／surface accent | It cannot drive body shape, maturity, or sensitive personality inference |

The A／B／C labels describe versioned evidence lanes, not personalities or fixed branches. All durable inputs must remain inside `SourceConsentScope`, use finite structured categories, and retain source／event／signal lineage. A alone shapes the available primary recipe direction; C supplies the outcome-qualified maturity gate and marks but does not select a silhouette when A is absent. B and content domain remain subordinate visual influences.

An in-scope newly normalized record may derive an allowlisted, content-minimized `EphemeralActivityHint` without a per-record prompt. The hint is memory-only, has a TTL, defaults to neutral, and is cleared when its evidence is disabled, unavailable, or expired. It may affect only recent expression, pose, motion, or light; it never enters SQLite, logs, telemetry, contribution records, maturity, or a permanent `MorphologyRecipe`. No live Agent monitoring exists today. Any future live-presence adapter is a distinct, separately consented feature and follows the same content-free TTL boundary.

Permanent reshape requires a versioned gate over multiple deduplicated, independent, outcome-qualified canonical groups. Time may order evidence and expire an `EphemeralActivityHint`, but cannot add support. Record quantity, duplicate sources, token counts, session length, and application open time likewise add no maturity or morphology support. Only accepting newly normalized evidence inside an active consent scope, correcting authorized evidence, forgetting it, disabling its consent scope, applying an explicit local correction, or running an acknowledged version migration may trigger permanent recomputation.

The user has confirmed the high-level biological／organic plus restrained sacred-premium direction and a content-responsive space with many variants. The current concept forms are visual-vocabulary and adjacent-bridge references, not a fixed pre-authored pet roster or one-to-one schema. The proposed `EvolutionPathProfile` is a recomputable weighted projection from approved, lineage-backed durable Agent-activity signals—not a branch selector or personality class. A finite versioned module catalog, compatibility allowlist, visual-slot caps, and quantized parameters compile the profile and genome into a `MorphologyRecipe`. This yields many but bounded, enumerable variants whose exact taxonomy and rules remain proposed. Direct raw-text classification, sensitive inference, sentiment branches, runtime model output, and unsaved randomness are forbidden permanent inputs.

Path contributions, other structural growth contributions, and existing WorldEffects are parallel projections from derived signals. Path contributions form weighted influences; structural contributions affect other genome axes; WorldEffects remain the only source of lineage-bearing marks. The genome does not replace current effect lineage or form a circular dependency with it. The growth core must not accept runtime LLM or image-generation inference as permanent input. A future model-produced candidate would require a separate product decision, explicit user approval, and machine-readable lineage before entering the graph; a conversation provider cannot directly or indirectly mutate permanent growth by default. The same authorized event set, identity seed, and version set must compile to the same lineage-bearing `MorphologyRecipe`. Profile axes, weights, and activity labels remain inside Rust and the authorized aggregate explanation boundary. Agent or source identity is not a profile／recipe input; if retained as source metadata, it may appear only behind a separately verified source-detail unlock gate and must update or disappear on forgetting. The pet renderer receives only final visual-module IDs, bounded quantized geometry／material／motion parameters, stage, revision, and mark IDs—not raw memory content, recipe lineage, or profile semantics. Forgetting a source or record, correcting its authorized category, or disabling its consent scope must atomically recompute the profile, maturity, recipe, stages, bridges, marks, habitat effects, explanations, and caches from the evidence that remains, with no ghost module or stale lineage. The proposed product and technical details are in the [evolving creature system design draft](drafts/deep-interview-evolving-creature-system-2026-08-11.md), [Agent-memory variation rules](drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md), and proposed [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md); none of that future layer is claimed by the current completion-star fixture.

## Fixture lifecycle and persistence

1. Tauri resolves one bundled resource path; the WebView cannot submit an arbitrary file path.
2. The adapter enforces a size limit, UTF-8 JSON, source identity, format version, and the supported record kind. Unknown input fails closed.
3. A preview token binds approval to the records prepared in Rust memory. Previewing or canceling does not persist source content, although desktop startup may initialize an empty SQLite schema.
4. Explicit approval writes the selected normalized record, canonical consent scope and hash, source contract, hashes, signal, effect, and lineage in local transactions.
5. The database lives at Tauri's app-local data directory as `memoryling.sqlite3`. Migrations 0001–0003 established fixture, consent, and Daily Scout state; migration 0004 adds Agent-memory event types and sync state and sets `PRAGMA user_version = 4`. Known earlier stores migrate deterministically; unknown future schemas fail closed.
6. Forgetting clears derived state, deletes the selected local source and its cascading consent scope／events, then re-runs deterministic derivation over supported records that remain, all in one transaction. The current store permits only one approved source total.

SQLite foreign keys and `secure_delete` are enabled for each connection. This supports application-level deletion; it is not a promise of cryptographic or physically irrecoverable erasure from storage media or backups.

## Connector contract

A connector must:

1. declare exactly which paths and formats it can read;
2. perform no writes to the source tool's files;
3. show an import preview before persistence;
4. normalize deterministic, testable records;
5. use synthetic fixtures in the repository;
6. fail closed when a format is unknown;
7. never collect credentials from source files.

The fixture adapter satisfies this contract for its fixed synthetic resource. The v0.5.0 `codex-local-memory-store` adapter is the primary, source-consented two-file connector described above. The `codex-app-server-thread` adapter remains a supplementary version-bound path for one explicitly selected completed thread. Neither authorizes arbitrary home-directory scanning or source writes.

## Trust boundaries

- **Bundled fixture:** fictional, repository-visible, fixed-path, read-only test input. It is not user memory.
- **Codex Agent-memory source:** one exact local `memories` root with two top-level allowlisted generated files; read-only, content-redacted at IPC, and auto-synced only after source-level consent.
- **Experimental Codex App Server source:** one user-selected work thread read through fixed local stdio `thread/list`／`thread/read`; it is untrusted, read-only, version-bound, and not a durable-memory source. Raw identifiers and selected text remain Rust-only.
- **Pending preview:** source content prepared in Rust process memory and bound to a short-lived catalog／preview token until approved or discarded. The frontend receives only redacted metadata and hashes.
- **Local Memoryling store:** contains the one approved normalized record, canonical consent scope and hash, content-free external lineage, and derived state; never print or commit the database.
- **UI:** displays explanations but must not render source content as trusted HTML.
- **Future model provider:** optional boundary requiring a separate ADR and explicit consent before any memory-derived context leaves the device.

## Open decisions

- embedded local model versus optional remote conversation provider;
- a production-supported Codex durable-memory export／API or supported successor to the experimental App Server pilot;
- migration strategy after SQLite schema v4;
- remaining Windows resident-shell acceptance across live DPI／monitor／taskbar changes, desktop hitbox, accessibility, and session shutdown;
- derivations and signal-to-genome mappings beyond the deterministic completion-star boundary;
- approved-activity taxonomy, signal-to-profile mapping, quantization rules, and mapping-version migration;
- final EvolutionBridge grammar for stage and recipe changes, stage names, and renderer implementation after synthetic visual prototyping.

Major decisions are recorded in [docs/adr](adr/INDEX.md), including the fixture-first SQLite boundary in [ADR-0002](adr/0002-sqlite-v1-fixture-first-memory.md), pet-first shell in [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md), proposed growth model in [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md), supplementary work-record pilot in [ADR-0005](adr/0005-codex-thread-history-source-pilot.md), and accepted primary Agent-memory source in [ADR-0007](adr/0007-codex-agent-memory-auto-sync.md).
