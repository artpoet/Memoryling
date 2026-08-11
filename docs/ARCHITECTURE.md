# Architecture

## Status

This document separates the intended product architecture from the subset implemented as of 2026-08-11. The implemented local pipeline runs end to end for exactly one fictional Codex-shaped resource bundled with the desktop app. No user-owned Codex file, tool-home, or other external source is connected.

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
| Desktop shell | Native window, lifecycle, notifications | One standard Tauri window and memory IPC commands exist; the proposed pet-first two-window resident shell, tray, and single-instance lifecycle are not implemented |
| Experience UI | Creature, habitat, stories, controls, explanations | Bilingual concept UI plus fixture selection, preview, consent, lineage, and forgetting; fixture state is separate and visible real-memory access remains off in desktop and browser |
| Source adapters | Read selected durable-memory formats without mutating them | Fixture adapter v1 reads one fixed bundled JSON resource; no external path or Codex tool-home access |
| Import gate | Preview scope, explain access, obtain consent | Implemented for the fixture, with pending preview state held in Rust memory; no real-source picker |
| Normalizer | Convert source records into a versioned local event schema | Schema v1 supports the fixture's `completion` record only |
| Derivation engine | Produce traits, tensions, story hooks, reminder candidates | One deterministic `completion` signal and `completion-star` world effect only |
| Local store | Persist normalized events, derived effects, lineage, and settings | SQLite schema v1 stores approved fixture records and lineage under Tauri app-local data; general settings are not included |
| Conversation layer | Ground dialogue in approved local context | Not implemented; provider decision open |
| Reminder policy | Enforce quiet hours, budget, urgency, and snooze state | UI concept only |

## Proposed pet-first desktop shell

The user-confirmed product direction is “two surfaces, one life,” recorded in proposed [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md) and the detailed [pet-first desktop shell draft](drafts/pet-first-desktop-shell-2026-08-11.md). This section describes intended architecture, not the current runtime.

```text
one Tauri process
  ├─ pet window: transparent, undecorated, render-safe creature state only
  ├─ main window: hidden until requested, full detail and lineage controls
  ├─ native context menu + tray + single-instance recovery
  └─ Rust lifecycle + canonical SQLite state
```

The `main` WebView should be created at startup but remain hidden, because Tauri documents a Windows deadlock risk when a WebView window is created inside a synchronous command or event handler. Rust should own show, hide, focus, `main` CloseRequested interception, native menu, tray, position recovery, and explicit quit without blocking Windows session shutdown. Pet frontend core capability should remain narrow—normally only window dragging and render-state event listening—rather than receiving cross-window creation, focus, menu, or tray permissions. `main` also needs an explicit reviewed permission set instead of inheriting the current broad `core:default`.

The pet surface must not call the existing full memory-state API because its lineage contains approved normalized text. A safe DTO alone is insufficient: Tauri app commands registered through `invoke_handler` are available to every window by default. The build must use `tauri_build::AppManifest::commands` to generate command permissions, assign all list／preview／cancel／full-state／approve／forget permissions only to `main`, give `pet` only a separate `CreatureRenderState` command plus necessary interactions, and also reject non-`main` callers inside sensitive commands. Negative tests must invoke each sensitive command from `pet` and prove fail-closed behavior.

`CreatureRenderState` contains only appearance parameters, neutral state, and an opaque revision. Approve, forget, or future genome commits emit a content-free revision event; each surface then refetches only the typed state allowed for that surface. No memory text, path, locator, or explanation payload belongs in pet IPC, native menu labels, tray labels, window titles, or operating-system notifications. Closing details must cancel any pending preview in Rust before hiding because hiding a WebView does not unmount it; minimizing preserves the preview.

Right-click is the primary entry. When `pet` has focus, Enter／Space／Menu key／`Shift+F10` opens the same native menu at a fixed pet anchor; reliable no-focus keyboard recovery is `Win+B` system tray, Start Menu, or a packaged UAT-confirmed installed shortcut. Opening or restoring `main` hides or docks `pet`; closing or minimizing `main` restores it, and only an explicit Quit ends the process. Pet position is restored in logical coordinates and clamped at launch, show／recovery, drag end, scale change, and single-instance callbacks; immediate topology／taskbar handling needs a verified Windows hook or polling strategy. Browser mode continues to show the detail preview and must not imitate native floating-window behavior.

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

The user-confirmed, not-yet-implemented product direction has the following proposed deterministic shape:

    approved events → derived signals
        ├─→ path contributions with lineage → deterministic EvolutionPathProfile ─┐
        ├─→ structural growth contributions ─────────────────────────────────────┴─→ recomputable CreatureGenome → stage snapshots + EvolutionBridge
        └─→ WorldEffects → marks / habitat / story projections

    IdentityCore + current stage snapshot + active WorldEffects + EphemeralState
        → render-safe CreatureState → local layered renderer

Permanent growth may substantially change morphology. Distant stages do not need to remain immediately recognizable as the same form, but every adjacent stage transition must produce a versioned `EvolutionBridge` describing preserved, grown, split, merged, relocated, or retired traits. Growth is driven only by approved memories and local derivations; raw record count and calendar time are not XP. Time supplies reversible day, season, anniversary, expression, and activity state only.

The user has confirmed the high-level biological／organic plus restrained sacred-premium direction and the goal of more than one content-responsive route. The proposed architecture realizes that goal with a versioned `EvolutionPathProfile` that can produce different or blended morphology routes from approved, lineage-backed activity signals. It is a recomputable weighted projection, not a personality class, and its exact taxonomy and blending rules remain proposed. Direct raw-text classification, sensitive inference, sentiment branches, runtime model output, and unsaved randomness are forbidden route inputs.

Path contributions, other structural growth contributions, and existing WorldEffects are parallel projections from derived signals. Path contributions form route weights; structural contributions affect other genome axes; WorldEffects remain the only source of lineage-bearing marks. The genome does not replace current effect lineage or form a circular dependency with it. The growth core must not accept runtime LLM or image-generation inference as permanent input. A future model-produced candidate would require a separate product decision, explicit user approval, and machine-readable lineage before entering the graph; a conversation provider cannot directly or indirectly mutate permanent growth by default. Path IDs, weights, and activity labels remain inside Rust and the authorized detail explanation boundary. The pet renderer receives only final visual-module IDs, bounded geometry／motion parameters, stage, revision, and mark IDs—not raw memory content or profile semantics. Forgetting must recompute the path profile, morphology, stages, bridges, marks, habitat effects, explanations, and caches from the events that remain. The proposed product and technical details are in the [evolving creature system design draft](drafts/deep-interview-evolving-creature-system-2026-08-11.md) and proposed [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md); none of that future layer is claimed by the current completion-star fixture.

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
- Windows resident-app lifecycle and notification integration;
- derivations and signal-to-genome mappings beyond the deterministic completion-star boundary;
- evolution-path taxonomy, signal-to-path mapping, dominant／secondary blending rules, and mapping-version migration;
- final EvolutionBridge grammar for stage and route changes, stage names, and renderer implementation after synthetic visual prototyping.

Major decisions are recorded in [docs/adr](adr/INDEX.md), including the fixture-only SQLite v1 boundary in [ADR-0002](adr/0002-sqlite-v1-fixture-first-memory.md), the proposed pet-first shell in [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md), and the proposed content-derived route model in [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md).
