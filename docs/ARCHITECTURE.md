# Architecture

## Status

AS_OF: 2026-08-13. Source version 0.6.0 implements the Agent-operated vertical slice in [ADR-0008](adr/0008-agent-operated-memoryling-protocol.md). The last installed-UAT artifact remains v0.2.0; no v0.6.0 packaged acceptance is claimed.

## System shape

```text
User says “Run Memoryling” in an Agent project
  │
  ▼
Project Agent skill
  reads only already-authorized context
  compiles protocol-v1 JSON
  rejects raw/private source payloads
  │ local atomic file handoff
  ▼
%LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v1.json
  │ exact-file polling; 64 KiB cap; no symlinks
  ▼
Rust protocol validator
  │ authoritative replacement transaction
  ▼
SQLite schema v5
  operation + hashed evidence + bilingual dialogue + runtime counters
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
| Produce activity, journey, evidence hashes, and dialogue | Agent skill |
| Validate size, schema, enums, IDs, timestamps, and bounds | Submit helper and Rust |
| Persist current pet state and dialogue counters | Rust + SQLite |
| Choose eligible dialogue | Local deterministic rule engine |
| Enforce quiet hours, cooldowns, expiry, and daily budget | Local deterministic rule engine |
| Render pet, status, and clear control | React surfaces |

## Agent operation protocol v1

The public contract is `schemas/agent-operation-v1.schema.json`; the human workflow is `skills/memoryling-operation/SKILL.md`.

- one operation ID and generated timestamp;
- `codex`, `claude`, or `other` Agent family;
- one dominant and optional secondary activity;
- one journey state;
- 1–12 evidence records with kind, timestamp, and lowercase SHA-256 reference hash;
- 3–12 English／Traditional Chinese dialogue cards;
- triggers `on-open`, `on-interact`, or `ambient`;
- per-card priority, optional time bounds, cooldown, max uses, and evidence IDs;
- strict unknown-field rejection and no raw-source field.

The package is a lossy derived artifact. Dialogue may evoke the work but must not quote private source content. `sourceDigest` detects unsafe operation-ID reuse; it is not a content-export channel.

## Inbox and failure semantics

The PowerShell submit helper validates without printing package content, serializes UTF-8 without BOM, and renames a temporary file inside the inbox directory. The app polls the exact file every five seconds.

Rust accepts only a non-symlink regular file from 1 byte through 64 KiB. Invalid JSON or protocol data is deleted and recorded as a bounded error code. A successful package is applied transactionally and the inbox file is removed. Re-sending the same operation ID and digest is idempotent; reusing an ID with another digest fails. A new valid operation is an authoritative snapshot that atomically replaces the prior operation, evidence, dialogue, and usage state.

## SQLite schema v5

Migration `0005_agent_operation_protocol.sql` adds:

- `agent_operations` — current compiled profile and source digest;
- `agent_operation_evidence` — opaque reference hashes only;
- `agent_dialogue_cards` — localized text and delivery bounds;
- `agent_dialogue_evidence` — package-local lineage;
- `agent_operation_runtime` — current dialogue and inbox status;
- `agent_dialogue_daily_usage` — ambient budget by local date.

Only the newest operation is retained. `clear_agent_operation` deletes the current derived package and resets runtime state. Foreign keys cascade; SQLite uses `secure_delete`, while documentation makes no cryptographic-erasure promise.

## Dialogue and render rules

- applying an operation selects the highest-priority `on-open` card;
- clicking the pet requests an eligible `on-interact` card;
- an in-process check may request `ambient` no more than every 15 minutes;
- ambient delivery is silent from 22:00 through 09:00 and capped at two lines per local day;
- expired, not-yet-valid, cooling-down, exhausted, and currently visible cards are skipped;
- no eligible card means the current render state stays unchanged.

`CreatureRenderState` schema v6 contains only stage, body module, palette, motion, safe marks, operation state, coarse activity, and the current localized dialogue. It excludes source hashes, evidence IDs, prompts, paths, operation digest, and full memory state.

## Desktop trust boundary

Rust owns the `pet` and `main` windows, tray, context menu, position persistence, single-instance recovery, and Quit. Exact Tauri capabilities and independent caller guards protect sensitive main commands. The pet can fetch only its render DTO, advance dialogue, move, dismiss onboarding, and open native UI. Revision events contain only an opaque hash.

Browser preview has no native inbox or persistence. It always states that memory access is off and performs no Agent operation.

## Compatibility code

The v0.1–v0.5 fixture import, exact Codex work-record pilot, direct Codex-memory connector, and BYOK Daily Scout remain in source for migration and research continuity. In v0.6.0 they are not started automatically or rendered in the primary flow. Daily Scout scheduling is disabled. They must not be presented as the current product path and may be removed after migration evidence is no longer needed.

## Future growth boundary

Protocol v1 activity accents and milestone marks are intentionally small. Permanent morphology needs versioned evidence grouping, a deterministic growth profile, a finite module catalog, correction and forgetting recomputation, accessible rendering, and a separate accepted decision. Runtime image generation, silent live Agent monitoring, sensitive personality inference, and time-as-XP remain out of scope.

## Open decisions

1. Cross-project skill discovery and installer experience.
2. A safe operation preview before submission without surfacing private source text.
3. Packaged v0.6.0 native acceptance and migration from older local databases.
4. Whether compatibility connectors should be removed or moved to a separate experimental build.
5. Versioned morphology recipe and growth-journal schemas.
