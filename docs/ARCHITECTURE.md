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
| Desktop shell | Native window, lifecycle, notifications | Tauri 2 shell and memory IPC commands exist; notifications are not implemented |
| Experience UI | Creature, habitat, stories, controls, explanations | Bilingual concept UI plus fixture selection, preview, consent, lineage, and forgetting; fixture state is separate and visible real-memory access remains off in desktop and browser |
| Source adapters | Read selected durable-memory formats without mutating them | Fixture adapter v1 reads one fixed bundled JSON resource; no external path or Codex tool-home access |
| Import gate | Preview scope, explain access, obtain consent | Implemented for the fixture, with pending preview state held in Rust memory; no real-source picker |
| Normalizer | Convert source records into a versioned local event schema | Schema v1 supports the fixture's `completion` record only |
| Derivation engine | Produce traits, tensions, story hooks, reminder candidates | One deterministic `completion` signal and `completion-star` world effect only |
| Local store | Persist normalized events, derived effects, lineage, and settings | SQLite schema v1 stores approved fixture records and lineage under Tauri app-local data; general settings are not included |
| Conversation layer | Ground dialogue in approved local context | Not implemented; provider decision open |
| Reminder policy | Enforce quiet hours, budget, urgency, and snooze state | UI concept only |

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
        ├─→ growth contributions with lineage → recomputable CreatureGenome → stage snapshots + EvolutionBridge
        └─→ WorldEffects → marks / habitat / story projections

    IdentityCore + current stage snapshot + active WorldEffects + EphemeralState
        → render-safe CreatureState → local layered renderer

Permanent growth may substantially change morphology. Distant stages do not need to remain immediately recognizable as the same form, but every adjacent stage transition must produce a versioned `EvolutionBridge` describing preserved, grown, split, merged, relocated, or retired traits. Growth is driven only by approved memories and local derivations; raw record count and calendar time are not XP. Time supplies reversible day, season, anniversary, expression, and activity state only.

Growth contributions and the existing WorldEffects are parallel projections from derived signals; the genome does not replace the current effect lineage or form a circular dependency with it. The growth core must not accept runtime LLM or image-generation inference as permanent input. A future model-produced candidate would require a separate product decision, explicit user approval, and machine-readable lineage before entering the graph; a conversation provider cannot directly or indirectly mutate permanent growth by default. The renderer receives no raw memory content, and forgetting must recompute morphology, stages, marks, habitat effects, explanations, and caches from the events that remain. The proposed product and technical details are in the [evolving creature system design draft](drafts/deep-interview-evolving-creature-system-2026-08-11.md); none of that future layer is claimed by the current completion-star fixture.

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
- final EvolutionBridge grammar, stage names, and renderer implementation after synthetic visual prototyping.

Major decisions are recorded in [docs/adr](adr/INDEX.md), including the fixture-only SQLite v1 boundary in [ADR-0002](adr/0002-sqlite-v1-fixture-first-memory.md).
