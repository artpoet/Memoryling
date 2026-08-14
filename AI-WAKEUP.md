# AI-WAKEUP — Memoryling

AS_OF: 2026-08-14 (Asia/Taipei)

## Read order

1. `AGENTS.md`
2. `AI-WAKEUP.md`
3. `PROJECT_STATUS.md`
4. `README.md`
5. `docs/ARCHITECTURE.md`
6. `docs/PRIVACY_PRINCIPLES.md`
7. `docs/adr/0010-installed-app-teaches-agent-activation.md`
8. `docs/adr/0008-agent-operated-memoryling-protocol.md`

If the user says `醒來吧我的寵物` or `Memoryling, wake up`, read `skills/memoryling-operation/SKILL.md` and execute the entire bounded workflow: confirm the installed pet is already open before reading memory for this workflow, compile, submit, await local application, and report here. Never start an executable from the Agent workflow. If the App is closed, stop before memory read or inbox write and tell the user to open the installed Memoryling App before using the phrase again. Requests to read this wake-up file or wake the project／Agent／self do not trigger the pet workflow without the exact pet phrase.

## Project identity

Memoryling is a local-first Windows desktop pet for Agent users. The user installs and opens the App normally; the pet itself teaches the phrase to enter in the current Agent project. The current Agent interprets already-authorized memory and recent work, submits a privacy-minimized result to the already-running pet, and waits for local application; the app turns that result into continuing appearance and dialogue through deterministic local rules.

Core formula:

```text
Agent understands and compiles
  + local app persists and lives
  = a pet shaped by the Agent relationship the user already has
```

## Current product truth

Source v0.7.0 implements the Agent-operated vertical slice from ADR-0008, ADR-0010, and ADR-0011:

- project trigger phrases and a validated `memoryling-operation` skill;
- strict protocol-v2 JSON Schema and 48-card synthetic example;
- PowerShell validation／submission to one exact app-local inbox file;
- strict running-App verification, no process launch from submission, and bounded inbox-consumption wait;
- five-second exact-file polling with regular-file, symlink, UTF-8, 64 KiB, and schema guards;
- SQLite schema v6 for the newest operation, hashed evidence, rolling bilingual dialogue counters, ambient usage, and current／pending appearance;
- authoritative semantic replacement, unchanged-card counter retention, duplicate idempotency, conflicting-ID rejection, and complete local clear;
- render-state schema v6 with coarse activity accent, milestone mark, and current dialogue;
- exactly 48 semi-specific cards: 8 opening, 20 interaction, 16 ambient, and 4 appearance;
- evidence-qualified appearance, at most one visible change per local day, and one pending plan;
- `on-open`, `on-interact`, and `ambient` rules with expiry, cooldown, max uses, 22:00–09:00 quiet hours, 35–70 minute ambient cadence, ten-minute spacing, and seven ambient lines per day;
- a dismissible, three-line, seven-second speech bubble on the floating pet;
- manual cold launch that shows the pet and bilingual activation reminder with a locale-specific copy button, OS-locale selection, and no blocking setup screen;
- browser preview that performs no memory read or native operation.

The app does **not** scan Agent storage and does **not** call an AI API for the core loop. The operation package must not contain raw memory, prompts, paths, secrets, names, tool output, or reasoning.

The v0.1–v0.5 fixture, one-thread, direct Codex-memory, and Daily Scout code remains only for compatibility and research continuity. Direct memory sync and Daily Scout scheduling are not started. Their UI is not part of the primary product path.

The unsigned v0.2.0 installer remains the last installed-UAT artifact. v0.7.0 is source-only until packaged install／upgrade／restart／clear／uninstall acceptance is completed.

## Runtime flow

```text
installed EXE launch → pet shows activation phrase
  → user phrase in current Agent project
  → Agent skill reads already-authorized context
  → temporary bounded JSON package
  → scripts/Submit-MemorylingOperation.ps1
  → readiness-only helper confirms Memoryling 0.7.0+ is already running before memory read
  → %LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v2.json
  → already-open pet consumes the inbox
  → Rust validates, rolls retained dialogue forward, and gates daily appearance
  → SQLite schema v6
  → render-safe DTO schema v6
  → pet and detail surfaces
```

The Agent owns semantics. Rust owns trust checks, persistence, clocks, eligibility, and deletion. React owns presentation and user controls.

## Key paths

| Area | Path |
|---|---|
| Agent trigger | `AGENTS.md` |
| Agent workflow | `skills/memoryling-operation/SKILL.md` |
| Protocol reference | `skills/memoryling-operation/references/protocol-v2.md` |
| Machine schema | `schemas/agent-operation-v2.schema.json` |
| Synthetic package | `examples/agent-operation-v2.synthetic.json` |
| Submit helper | `scripts/Submit-MemorylingOperation.ps1` |
| Inbox worker／validation | `src-tauri/src/memory/agent_operation.rs` |
| SQLite operation store | `src-tauri/src/memory/store.rs` |
| Migration | `src-tauri/migrations/0006_agent_operation_protocol_v2.sql` |
| Render DTO | `src-tauri/src/memory/model.rs`, `src/creatureClient.ts` |
| Detail operation UX | `src/AgentOperationPanel.tsx` |
| Pet dialogue UX | `src/PetSurface.tsx` |
| Current ADR | `docs/adr/0011-memory-grounded-daily-growth-and-dialogue-v2.md` |

## Hard boundaries

- Never read, print, commit, screenshot, or test with real Agent memory without exact authorization.
- Never write to Codex, Claude, or another Agent-owned memory store.
- Never add telemetry, cloud sync, an external AI call, or a new connector without an explicit product decision and privacy review.
- The slogan authorizes one bounded derived package only.
- Opening the App authorizes no memory read; only the activation phrase authorizes one package.
- The package may contain generated pet state and opaque hashes, never raw source content.
- The app polls one exact inbox file; do not add tool-home discovery or arbitrary path access.
- Only the newest semantic operation is retained. Unchanged stable dialogue may keep counters; retired content is deleted; clear removes operation, counters, and current／pending appearance.
- Every dialogue must cite package-local evidence IDs and obey length／trigger／cooldown／expiry／use bounds.
- Pet DTOs, labels, events, and logs remain content-minimized.
- English is primary public language; Traditional Chinese remains first-class and meaningfully equivalent.
- Never present compatibility code, mock data, source-only behavior, or the v0.2.0 installer as v0.7.0 packaged proof.
- Do not submit the Codex for Open Source application without a new explicit user instruction.

## Known traps

- A direct Agent-memory connector is the superseded v0.5.0 direction, not the current core.
- Daily Scout exists in source but its scheduler and primary UI are intentionally disabled.
- Browser preview cannot prove inbox, SQLite, tray, transparent-pet, or native persistence behavior.
- Hashing a secret does not make it acceptable package data.
- Source deletion cannot be observed because the app does not scan Agent storage. The next operation is authoritative; immediate removal uses Clear.
- Reusing an operation ID with different evidence must fail; a new snapshot needs a new ID.
- Generated user-derived temporary packages must be deleted after submission and never committed.
- Source checks do not prove installed EXE launch, visible phrase teaching, or running-process handoff; packaged native smoke is required.

## Commands

```powershell
npm run check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

Synthetic submit-helper smoke:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path examples/agent-operation-v2.synthetic.json
```

Skill validation uses the official `skill-creator` validator. On this Traditional Chinese Windows host set UTF-8 mode; the validator also requires PyYAML.

## Fresh-session opening response

Report, in this order:

1. Memoryling is now Agent-operated: slogan → Agent compilation → local rule-driven pet.
2. The user opens the installed App first; the pet teaches the phrase, and Agent submission never launches a process.
3. v0.7.0 source implements the protocol-v2 synthetic vertical slice; the app does not scan memories or call AI.
4. v0.2.0 remains the installed-UAT baseline; v0.7.0 packaging is still open.
5. Name one coherent next bundle and do it end to end.

## Current coherent next bundle

**Install and accept the current v0.7.0 artifact on Windows without private data.**

1. Test clean install or upgrade from the retained v0.2.0 baseline using `src-tauri/target/release/bundle/nsis/Memoryling_0.7.0_x64-setup.exe` (4,966,744 bytes; SHA-256 `5EB39108F3468FB6DB383C70C055F8B3B11D0958F3EB904EEA991FB98FEA741B`; `NotSigned`).
2. Confirm the installed App opens pet-first, shows the bilingual activation reminder before any operation, and copies the exact locale-specific phrase.
3. Use only `examples/agent-operation-v2.synthetic.json`.
4. Verify no-setup cold launch, exact phrase copy, running-process submission without auto-launch, inbox pickup, one-change-per-day appearance, pending appearance, opening／click／ambient dialogue, rolling counters, speech-bubble dismissal, restart persistence, replacement, clear, manual single-instance pet recovery, and uninstall data choices.
5. Record content-free evidence, version, size, checksum, signature state, and remaining WebView2／accessibility／mixed-DPI gaps.

Do not switch this bundle into private-memory UAT or reinstall the skill globally without a separate instruction.

## Closeout checklist

- update `PROJECT_STATUS.md`, `CHANGELOG.md`, and affected docs;
- sync this `AI-WAKEUP.md` when architecture, traps, structure, or the current bundle changes;
- run frontend, Rust, formatting, clippy, and visible smoke checks;
- verify no user-derived package, local database, secret, or tool-home file entered git;
- commit and push tracked changes;
- verify GitHub Actions rather than inferring from local success;
- run the repository Final Gate through the governance checkout when available.
