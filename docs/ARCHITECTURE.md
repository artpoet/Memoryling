# Architecture

## Status

AS_OF: 2026-08-14. Source version 0.7.0 implements the Agent-operated base in [ADR-0008](adr/0008-agent-operated-memoryling-protocol.md), the installed-App activation contract in [ADR-0010](adr/0010-installed-app-teaches-agent-activation.md), and protocol-v2 daily growth／rolling dialogue in [ADR-0011](adr/0011-memory-grounded-daily-growth-and-dialogue-v2.md). The last installed-UAT artifact remains v0.2.0; no v0.7.0 packaged acceptance is claimed yet.

## System shape

```text
User installs and opens Memoryling through the Windows EXE
  │ pet shows the activation phrase while idle
  ▼
User says “Memoryling, wake up” or “醒來吧我的寵物” in the current Agent project
  │
  ▼
Project Agent skill
  reads only already-authorized context
  checks that the compatible App is open before memory read
  compiles protocol-v2 JSON
  rejects raw/private source payloads
  │ confirms compatible app is already running before submission
  ▼
%LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v2.json
  │ atomic handoff to the already-running pet
  │ exact-file polling; 64 KiB cap; no symlinks
  ▼
Rust protocol validator
  │ rolling dialogue + evidence-gated daily appearance transaction
  ▼
SQLite schema v6
  operation + hashed evidence + 48 bilingual cards + rolling counters + appearance
  │ render-safe DTO / opaque revision
  ├──────────────► transparent pet window
  └──────────────► detail window
```

The semantic boundary ends at the package. Memoryling does not discover tool homes, read Agent memory, run a model, or call an AI provider for the core loop.

## Responsibility split

| Concern | Owner |
|---|---|
| Understand memory, recent work, current thread, and SSOT | Current Agent |
| Honor the Agent environment's existing authorization | Current Agent |
| Produce activity, journey, appearance plan, evidence hashes, and 48-card dialogue deck | Agent skill |
| Validate size, schema, enums, IDs, timestamps, and bounds | Submit helper and Rust |
| Verify a trusted compatible pet is already running and await inbox consumption | Agent-side local helper |
| Persist current pet state, rolling dialogue counters, and current／pending appearance | Rust + SQLite |
| Choose eligible dialogue | Local deterministic rule engine |
| Enforce quiet hours, cooldowns, expiry, and daily budget | Local deterministic rule engine |
| Render pet, status, and clear control | React surfaces |

## Agent operation protocol v2

The public contract is `schemas/agent-operation-v2.schema.json`; the human workflow is `skills/memoryling-operation/SKILL.md`.

- one operation ID and generated timestamp;
- `codex`, `claude`, or `other` Agent family;
- one dominant and optional secondary activity;
- one journey state;
- 1–12 evidence records with kind, timestamp, and lowercase SHA-256 reference hash;
- one `hold`, `change`, or `reset` appearance plan with package-local evidence lineage;
- exactly 48 English／Traditional Chinese dialogue cards: 8 opening, 20 interaction, 16 ambient, and 4 appearance;
- category-bound triggers `on-open`, `on-interact`, or `ambient`;
- stable theme IDs and semantic groups for rolling retention and repetition avoidance;
- per-card priority, optional time bounds, cooldown, max uses, and evidence IDs;
- strict unknown-field rejection and no raw-source field.

The package is a lossy derived artifact. Dialogue may evoke recognizable work themes but must not quote private source content or name private projects. Each localized line is capped at 160 characters. `sourceDigest` detects unsafe operation-ID reuse; it is not a content-export channel.

## Inbox, launch, and failure semantics

Before any pet-workflow memory read, the Agent runs the helper's readiness-only check for an already-running Memoryling 0.7.0 or newer process. Submission repeats the same check before writing. The helper accepts only `Memoryling.exe` with product identity `Memoryling`; a development-only explicit path must also match a running process. It rejects another filename, product identity, stale version, a closed App, and arbitrary `PATH` resolution.

The submit helper validates without printing package content, serializes UTF-8 without BOM, and renames a temporary file inside the inbox directory. It never starts a process. The helper waits at most 15 seconds for the exact inbox item to be consumed, then reports a bounded outcome to the Agent conversation. If the App is not open, it fails before inbox write; if consumption is not confirmed in time, it removes the exact unconfirmed item so no update can apply unexpectedly on a later launch.

Rust polls the exact file every five seconds and also checks immediately when its worker starts. It accepts only a non-symlink regular file from 1 byte through 64 KiB. Invalid JSON or protocol data is deleted and recorded as a bounded error code. A successful package is applied transactionally and the inbox file is removed. Re-sending the same operation ID and digest is idempotent; reusing an ID with another digest fails. A new valid operation is an authoritative snapshot that atomically replaces the prior operation, evidence, dialogue, and usage state.

## SQLite schema v6

Migration `0006_agent_operation_protocol_v2.sql` replaces the protocol-v1 operation tables with:

- `agent_operations` — current compiled profile and source digest;
- `agent_operation_evidence` — opaque reference hashes only;
- `agent_dialogue_cards` — localized text, category, stable theme／semantic identity, and delivery bounds;
- `agent_dialogue_evidence` — package-local lineage;
- `agent_operation_runtime` — current dialogue and inbox status;
- `agent_dialogue_daily_usage` — ambient budget by local date;
- `agent_current_appearance` and `agent_current_appearance_evidence` — the visible evidence-qualified appearance and opaque lineage;
- `agent_appearance_daily_usage` — the one-change-per-local-day gate;
- `agent_pending_appearance` and `agent_pending_appearance_evidence` — one qualified deferred plan for the next eligible day.

Only the newest operation is retained. Unchanged dialogue IDs with identical bilingual text retain use count and last-used time; changed or absent cards and their content are deleted. `clear_agent_operation` deletes the operation, counters, current／pending appearance, lineage, and runtime state. Foreign keys cascade; SQLite uses `secure_delete`, while documentation makes no cryptographic-erasure promise.

## Dialogue and render rules

- applying an operation selects the highest-priority `on-open` card;
- an applied appearance change may select an eligible appearance card instead;
- clicking the pet requests an eligible `on-interact` card;
- click delivery has a two-second anti-stack interval and does not consume the ambient budget;
- an in-process check requests `ambient` on a locally selected 35–70 minute cadence;
- ambient delivery is silent from 22:00 through 09:00, waits at least ten minutes after any shown line, and is capped at seven lines per local day;
- selection favors least-used／least-recently-used eligible cards and avoids the current semantic group or theme when alternatives exist;
- expired, not-yet-valid, cooling-down, exhausted, and currently visible cards are skipped;
- no eligible card means the current render state stays unchanged.

An evidence-qualified visible appearance may apply once per local day. A later qualified same-day plan replaces the single pending plan and can apply on the next local day. `hold` makes no change; `reset` requires explicit source-removal evidence. Protocol v2 still uses the finite activity accent and completion-star render vocabulary rather than runtime image generation.

`CreatureRenderState` schema v6 contains only stage, body module, palette, motion, safe marks, operation state, coarse activity, and the current localized dialogue. It excludes source hashes, evidence IDs, prompts, paths, operation digest, and full memory state.

## Desktop trust boundary

Rust owns the `pet` and `main` windows, tray, context menu, position persistence, single-instance recovery, and Quit. Manual cold launch and resident relaunch both end pet-first; no blocking setup window is required. The first-run guide teaches the activation phrase with a local clipboard-copy control, and the idle pet keeps showing the phrase. Exact Tauri capabilities and independent caller guards protect sensitive main commands. The pet can fetch only its render DTO, advance dialogue, move, dismiss onboarding, and open native UI. Revision events contain only an opaque hash.

Browser preview has no native inbox or persistence. It always states that memory access is off and performs no Agent operation.

## Compatibility code

The v0.1–v0.5 fixture import, exact Codex work-record pilot, direct Codex-memory connector, and BYOK Daily Scout remain in source for migration and research continuity. In v0.7.0 they are not started automatically or rendered in the primary flow. Daily Scout scheduling is disabled. They must not be presented as the current product path and may be removed after migration evidence is no longer needed.

## Future growth boundary

Protocol v2 adds evidence-gated persistence and daily queuing to the existing activity accents and milestone marks. Rich permanent morphology still needs versioned evidence grouping, a deterministic module catalog, correction and forgetting recomputation, accessible rendering, and a separate accepted decision. Runtime image generation, silent live Agent monitoring, sensitive personality inference, and time-as-XP remain out of scope.

## Open decisions

1. Cross-project skill discovery and installer experience, including running-App handoff from arbitrary projects.
2. A safe operation preview before submission without surfacing private source text.
3. Packaged v0.7.0 native acceptance and migration from older local databases.
4. Whether compatibility connectors should be removed or moved to a separate experimental build.
5. Versioned morphology recipe and growth-journal schemas.
