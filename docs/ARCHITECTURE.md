# Architecture

## Status

This document separates the intended product architecture from the subset implemented as of 2026-08-12. Memoryling 0.2.0 now implements the pet-first resident desktop shell and runs the local fixture pipeline end to end for exactly one fictional Codex-shaped resource bundled with the desktop app. No user-owned Codex file, tool-home, or other external source is connected, and real-source and creature-growth implementation have not started.

## System shape

    External durable-memory sources
        → read-only source adapters
        → import preview and consent gate
        → normalized local memory events
        → derivation engine
        → lineage-aware local store
        → creature state, stories, conversations, reminders
        → bilingual Tauri UI

The v1 fixture slice follows this shape but replaces the external source with a fixed bundled resource and produces only one deterministic completion-star effect. It is test infrastructure and a product-flow proof, not a production Codex connector.

## Layers

| Layer | Responsibility | Current state |
|---|---|---|
| Desktop shell | Native window, lifecycle, notifications | 0.2.0 pre-creates transparent `pet` and hidden `main` windows; Rust owns native menu, tray, single-instance recovery, show／hide／focus, close／minimize／restore compensation, explicit Quit, and pet position recovery |
| Experience UI | Creature, habitat, stories, controls, explanations | Bilingual pet and detail surfaces plus fixture selection, preview, consent, lineage, forgetting, one-time onboarding, reduced-motion handling, and a visible real-memory-off state in desktop and browser |
| Source adapters | Read selected durable-memory formats without mutating them | Fixture adapter v1 reads one fixed bundled JSON resource; no external path or Codex tool-home access |
| Import gate | Preview scope, explain access, obtain consent | Implemented for the fixture, with pending preview state held in Rust memory; no real-source picker |
| Normalizer | Convert source records into a versioned local event schema | Schema v1 supports the fixture's `completion` record only |
| Derivation engine | Produce traits, tensions, story hooks, reminder candidates | One deterministic `completion` signal and `completion-star` world effect only |
| Local store | Persist normalized events, derived effects, lineage, and settings | SQLite schema v1 stores approved fixture records and lineage under Tauri app-local data; content-free shell preferences and pet position use a separate atomic JSON file in the same app-local directory |
| Conversation layer | Ground dialogue in approved local context | Not implemented; provider decision open |
| Reminder policy | Enforce quiet hours, budget, urgency, and snooze state | UI concept only |

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

The build uses `tauri_build::AppManifest::commands` and exact local-only `main`／`pet` capabilities; neither surface inherits `core:default`, remote scopes, wildcards, or deny-pattern ambiguity. List／preview／cancel／full-state／approve／forget are `main`-only and also require a `MainCaller` whose WebView and native-window labels both match. `pet` receives only render-safe state, shell state, menu, onboarding, and `start_pet_dragging` app commands plus event listen／unlisten. The drag command acts only on the caller's pet window, so pet JavaScript cannot select and drag `main` through a generic core window API. A production-authority invoke harness denies all six sensitive commands at the ACL layer, and a separate empty-authority harness proves the caller guard denies the same six before handler body entry; a `main` list invoke is the positive control.

`CreatureRenderState` contains only bounded appearance parameters, neutral fixture state, opaque mark IDs, and a 64-hex revision. Approve and forget emit the same content-free `{revision}` notification to both surfaces, which then refetch their typed state; event-delivery failure does not roll back a committed memory transaction. No memory text, path, locator, explanation, source identity, or content hash enters pet IPC, native menu labels, tray labels, window titles, or operating-system notifications. Closing details cancels any pending preview in Rust before hiding because hiding a WebView does not unmount it; minimizing preserves the preview.

Right-click is the primary entry. When `pet` has focus, Enter／Space／Menu key／`Shift+F10` invokes the same native menu at a fixed pet anchor. Native menu, transparent pet, onboarding, one-detail-window lifecycle, minimize／restore, single-instance recovery, and explicit native Quit passed the core Windows smoke; position persistence／clamp and tray actions have automated coverage. A normal Explorer-launched current-user NSIS install and the actual installed Start shortcut passed both cold launch and resident single-instance relaunch: the second launch focused the existing detail surface without creating another process. Direct tray-action and `Win+B` traversal remain pending live acceptance.

Pet position is stored in a content-free JSON record with monitor identity, work-area dimensions, logical coordinates, normalized coordinates, and scale factor. Rust restores and clamps it on launch and recovery, coalesces move／scale persistence, and polls monitor work-area topology. Pure geometry tests cover negative origins, taskbar work-area offsets, oversized windows, removed-monitor fallback, and logical 320-pixel sizing at 100／125／150／200%; live 125–200% and mixed-DPI movement, hot-unplug, and taskbar relocation remain acceptance work. Browser mode continues to show the detail preview and does not imitate native floating-window behavior.

### 0.2.0 verification snapshot

- Automated evidence: 23 frontend tests and 29 Rust tests pass. Rust coverage includes concurrent first-open migration, lifecycle compensation, position／anchor recovery, content-minimized DTOs, exact capabilities, and the two independent six-command denial layers described above.
- Native and packaged evidence: transparent pet／first-run onboarding, pointer and focused-keyboard native menu paths, close／minimize／restore, single-instance recovery, explicit native Quit, raw movement／second-monitor observation, and core pet／main state transitions pass on the current Windows host. Tray actions and position recovery have automated evidence; their remaining live matrix is not inferred from that.
- Fixture evidence: raw bundled fixture preview and approval, restart persistence, source → event → signal → completion-star lineage, cross-surface state, and complete forgetting pass; no real source was used.
- Installer evidence: a normal Explorer-launched NSIS current-user install, actual installed Start shortcut cold launch and resident relaunch, explicit Quit, and uninstall with retained app data pass. The retained files were checked only as local app-data state, not committed or printed.
- Artifact: `Memoryling_0.2.0_x64-setup.exe`, 2,875,965 bytes, SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`, `FileVersion`／`ProductVersion` 0.2.0, `NotSigned`.
- Harness trap: an early agent-direct installer launch produced Windows virtualization behavior and is not valid product evidence or a product failure. Acceptance uses normal Explorer and installed-shortcut paths.
- Still pending before ADR acceptance: live 125–200%／mixed-DPI testing, hot-unplug／taskbar relocation, adjacent-desktop hitbox probing, `Win+B`, Narrator／NVDA, sign-out／shutdown, and compact／wide／tall／long growth-envelope coverage. WebView2-missing bootstrapper testing remains deferred. Real-source and growth work have not started.

## Implemented v1 records and future shape

### Source import and memory event

- `source_imports` retains the adapter ID and version, display label, fixed locator, and source-content hash.
- `memory_events` retains schema version 1, an opaque source-record ID, source and observed timestamps, the approved normalized text, and its content hash.
- The current schema accepts only `completion`; future record kinds require explicit schema and derivation work.

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

Everything in this section is a future Phase 2 proposal unless explicitly identified as current fixture behavior. The current fixture preview binds one pending token to selected records from one bundled fictional resource; explicit approval persists those selected normalized records and can derive only the completion star. It does not create a reusable consent scope, observe live Agent use, classify A／B／C evidence, accumulate outcome groups, or compile a creature morphology.

### Proposed source-consent scope

A future `SourceConsentScope` would bind one explicit consent to one specific read-only source and adapter version, allowed data categories, and named local derivation purposes. Records that remain inside all boundaries may be normalized and deterministically derived without asking for approval for every downstream visual change. Another source requires another scope. Admitting a new data category, using authorized data for a new purpose, or materially changing mapping semantics must stop at a new scope-revision preview and require fresh consent before that expansion contributes anything.

Disabling a consent scope makes its evidence ineligible for active derivation and triggers the same deterministic downstream recomputation as forgetting or correcting evidence; it never writes to or deletes the source. The exact persistence, re-enable, and retention UX remains proposed and requires synthetic acceptance before private-data testing.

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
4. Explicit approval writes the selected normalized record, source contract, hashes, signal, effect, and lineage in local transactions.
5. The database lives at Tauri's app-local data directory as `memoryling.sqlite3`. Migration 0001 sets `PRAGMA user_version = 1`; unknown future versions fail closed.
6. Forgetting clears derived state, deletes the selected local source and its cascading events, then re-runs deterministic derivation over supported records that remain, all in one transaction. The current adapter exposes only one source.

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

The current fixture adapter satisfies this contract only for its fixed synthetic resource. It does not establish the format, discovery path, or permission UX for user-owned Codex memory. A future production adapter must validate a real supported format and accept only a source explicitly selected by the user; it must not scan arbitrary home directories.

## Trust boundaries

- **Bundled fixture:** fictional, repository-visible, fixed-path, read-only test input. It is not user memory.
- **Future external source files:** user-owned, untrusted input, and read-only; none are connected today.
- **Pending preview:** source content prepared in Rust process memory and bound to a preview token until approved or discarded.
- **Local Memoryling store:** contains approved normalized text, hashes, lineage, and derived state; never print or commit the database.
- **UI:** displays explanations but must not render source content as trusted HTML.
- **Future model provider:** optional boundary requiring a separate ADR and explicit consent before any memory-derived context leaves the device.

## Open decisions

- embedded local model versus optional remote conversation provider;
- validated Codex durable-memory format, native source selection, and permission UX;
- migration strategy after SQLite schema v1;
- remaining Windows resident-shell acceptance across live DPI／monitor／taskbar changes, desktop hitbox, accessibility, and session shutdown;
- derivations and signal-to-genome mappings beyond the deterministic completion-star boundary;
- approved-activity taxonomy, signal-to-profile mapping, quantization rules, and mapping-version migration;
- final EvolutionBridge grammar for stage and recipe changes, stage names, and renderer implementation after synthetic visual prototyping.

Major decisions are recorded in [docs/adr](adr/INDEX.md), including the fixture-only SQLite v1 boundary in [ADR-0002](adr/0002-sqlite-v1-fixture-first-memory.md), the proposed pet-first shell in [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md), and the proposed content-derived route model in [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md).
