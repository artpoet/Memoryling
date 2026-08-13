# AI-WAKEUP — Memoryling

AS_OF: 2026-08-13 (Asia/Taipei)

## Read order

1. `AGENTS.md`
2. `AI-WAKEUP.md`
3. `PROJECT_STATUS.md`
4. `README.md`
5. `docs/ARCHITECTURE.md`
6. `docs/PRIVACY_PRINCIPLES.md`
7. `docs/adr/0009-conversation-first-pet-wake.md`
8. `docs/adr/0008-agent-operated-memoryling-protocol.md`

If the user says `運作 Memoryling`, `執行 Memoryling`, or `Run Memoryling`, read `skills/memoryling-operation/SKILL.md` and execute the entire bounded workflow: compile, submit, wake the installed pet, await local application, and report here. Do not ask the user to open the App manually. If they say only `叫出 Memoryling`／`Show Memoryling`, use wake-only without reading memory or creating a package.

## Project identity

Memoryling is a local-first Windows desktop pet for Agent users. The user controls the ordinary flow from the Agent conversation with one phrase. The current Agent interprets already-authorized memory and recent work, submits a privacy-minimized result, automatically wakes the installed pet, and waits for local application; the app turns that result into continuing appearance and dialogue through deterministic local rules.

Core formula:

```text
Agent understands and compiles
  + local app persists and lives
  = a pet shaped by the Agent relationship the user already has
```

## Current product truth

Source v0.6.0 implements the Agent-operated vertical slice from ADR-0008:

- project trigger phrases and a validated `memoryling-operation` skill;
- strict protocol-v1 JSON Schema and synthetic example;
- PowerShell validation／submission to one exact app-local inbox file;
- strict installed-App resolution, automatic cold／single-instance pet wake, and bounded inbox-consumption wait;
- five-second exact-file polling with regular-file, symlink, UTF-8, 64 KiB, and schema guards;
- SQLite schema v5 for the newest operation, hashed evidence, bilingual dialogue, runtime counters, and ambient daily usage;
- authoritative replacement, duplicate idempotency, conflicting-ID rejection, and local clear;
- render-state schema v6 with coarse activity accent, milestone mark, and current dialogue;
- `on-open`, `on-interact`, and `ambient` rules with expiry, cooldown, max uses, 22:00–09:00 quiet hours, and two ambient lines per day;
- cold launch that shows the pet directly with OS-locale selection and no blocking setup screen;
- browser preview that performs no memory read or native operation.

The app does **not** scan Agent storage and does **not** call an AI API for the core loop. The operation package must not contain raw memory, prompts, paths, secrets, names, tool output, or reasoning.

The v0.1–v0.5 fixture, one-thread, direct Codex-memory, and Daily Scout code remains only for compatibility and research continuity. Direct memory sync and Daily Scout scheduling are not started. Their UI is not part of the primary product path.

The unsigned v0.2.0 installer remains the last installed-UAT artifact. v0.6.0 is source-only until packaged install／upgrade／restart／clear／uninstall acceptance is completed.

## Runtime flow

```text
user phrase in Agent project
  → Agent skill reads already-authorized context
  → temporary bounded JSON package
  → scripts/Submit-MemorylingOperation.ps1
  → scripts/Start-Memoryling.ps1 resolves Memoryling 0.6.0+
  → %LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v1.json
  → cold launch or single-instance return-to-pet
  → Rust validates and atomically replaces prior operation
  → SQLite schema v5
  → render-safe DTO schema v6
  → pet and detail surfaces
```

The Agent owns semantics. Rust owns trust checks, persistence, clocks, eligibility, and deletion. React owns presentation and user controls.

## Key paths

| Area | Path |
|---|---|
| Agent trigger | `AGENTS.md` |
| Agent workflow | `skills/memoryling-operation/SKILL.md` |
| Protocol reference | `skills/memoryling-operation/references/protocol-v1.md` |
| Machine schema | `schemas/agent-operation-v1.schema.json` |
| Synthetic package | `examples/agent-operation-v1.synthetic.json` |
| Submit helper | `scripts/Submit-MemorylingOperation.ps1` |
| Pet launcher | `scripts/Start-Memoryling.ps1` |
| Inbox worker／validation | `src-tauri/src/memory/agent_operation.rs` |
| SQLite operation store | `src-tauri/src/memory/store.rs` |
| Migration | `src-tauri/migrations/0005_agent_operation_protocol.sql` |
| Render DTO | `src-tauri/src/memory/model.rs`, `src/creatureClient.ts` |
| Detail operation UX | `src/AgentOperationPanel.tsx` |
| Pet dialogue UX | `src/PetSurface.tsx` |
| Current ADR | `docs/adr/0009-conversation-first-pet-wake.md` |

## Hard boundaries

- Never read, print, commit, screenshot, or test with real Agent memory without exact authorization.
- Never write to Codex, Claude, or another Agent-owned memory store.
- Never add telemetry, cloud sync, an external AI call, or a new connector without an explicit product decision and privacy review.
- The slogan authorizes one bounded derived package only.
- Wake-only authorizes no memory read and no new package.
- The package may contain generated pet state and opaque hashes, never raw source content.
- The app polls one exact inbox file; do not add tool-home discovery or arbitrary path access.
- Only the newest operation is retained. New operation replaces old; clear removes current derived state.
- Every dialogue must cite package-local evidence IDs and obey length／trigger／cooldown／expiry／use bounds.
- Pet DTOs, labels, events, and logs remain content-minimized.
- English is primary public language; Traditional Chinese remains first-class and meaningfully equivalent.
- Never present compatibility code, mock data, source-only behavior, or the v0.2.0 installer as v0.6.0 packaged proof.
- Do not submit the Codex for Open Source application without a new explicit user instruction.

## Known traps

- A direct Agent-memory connector is the superseded v0.5.0 direction, not the current core.
- Daily Scout exists in source but its scheduler and primary UI are intentionally disabled.
- Browser preview cannot prove inbox, SQLite, tray, transparent-pet, or native persistence behavior.
- Hashing a secret does not make it acceptable package data.
- Source deletion cannot be observed because the app does not scan Agent storage. The next operation is authoritative; immediate removal uses Clear.
- Reusing an operation ID with different evidence must fail; a new snapshot needs a new ID.
- Generated user-derived temporary packages must be deleted after submission and never committed.
- Source checks do not prove installed executable discovery or visible single-instance wake; packaged native smoke is required.

## Commands

```powershell
npm run check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

Synthetic submit-helper smoke:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path examples/agent-operation-v1.synthetic.json
```

Skill validation uses the official `skill-creator` validator. On this Traditional Chinese Windows host set UTF-8 mode; the validator also requires PyYAML.

## Fresh-session opening response

Report, in this order:

1. Memoryling is now Agent-operated: slogan → Agent compilation → local rule-driven pet.
2. The ordinary control surface is the Agent conversation; submission automatically wakes the pet with no blocking setup page.
3. v0.6.0 source implements the complete synthetic vertical slice; the app does not scan memories or call AI.
4. v0.2.0 remains the installed-UAT baseline; v0.6.0 packaging is still open.
5. Name one coherent next bundle and do it end to end.

## Current coherent next bundle

**Install and accept the current v0.6.0 artifact on Windows without private data.**

1. Test clean install or upgrade from the retained v0.2.0 baseline using `src-tauri/target/release/bundle/nsis/Memoryling_0.6.0_x64-setup.exe`.
2. Confirm installed-App resolution without the development-only explicit executable path.
3. Use only the committed synthetic operation package.
4. Verify no-setup cold launch, automatic inbox pickup, appearance, opening dialogue, click dialogue, restart persistence, replacement, clear, conversation-driven single-instance pet recovery, and uninstall data choices.
5. Record content-free evidence, version, size, checksum, signature state, and remaining WebView2／accessibility／mixed-DPI gaps.

Do not switch this bundle into private-memory UAT or reinstall the skill globally without a separate instruction.

## Closeout checklist

- update `PROJECT_STATUS.md`, `CHANGELOG.md`, and affected docs;
- run frontend, Rust, formatting, clippy, and visible smoke checks;
- verify no user-derived package, local database, secret, or tool-home file entered git;
- commit and push tracked changes;
- verify GitHub Actions rather than inferring from local success;
- run the repository Final Gate through the governance checkout when available.
