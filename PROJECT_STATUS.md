# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-13 (Asia/Taipei)

## Current milestone

Source v0.6.0 now implements the user-confirmed **Agent-operated Memoryling** model end to end with synthetic data:

```text
install and open Memoryling EXE
  → pet visibly teaches “醒來吧我的寵物”／“Wake up, my pet”
  → user enters the phrase in the current Agent project
  → current Agent reads already-authorized context
  → Agent compiles one bounded operation package
  → Agent submits to the already-running pet without launching a process
  → local app validates, persists, renders, and speaks by deterministic rules
```

The core requires no app-side AI API and no direct app scan of Agent memory. The current source is ready for packaged synthetic acceptance; it is not yet a public release and has not performed private-memory UAT.

## Overall state

- Version: 0.6.0 source vertical slice
- Product surface: bilingual transparent pet plus on-demand detail; browser remains an honest detail-only preview
- Agent integration: project trigger phrases plus validated `memoryling-operation` skill
- Primary control: installed App for launch and visible phrase teaching; Agent conversation for semantic updates
- Handoff: strict protocol-v1 JSON, PowerShell running-process validation, exact app-local inbox, Rust revalidation
- Local store: SQLite schema v5; only the newest authoritative Agent operation is retained
- Pet boundary: render DTO schema v6 with coarse activity, safe marks, and current dialogue only
- Dialogue: 3–12 bilingual cards with open／interaction／ambient triggers, cooldown, expiry, max uses, quiet hours, and ambient budget
- User control: local clear removes the current derived operation; rerunning the slogan rebuilds from current Agent context
- Core network behavior: none; no API key and no model request
- Legacy compatibility: fixture, one-thread, direct Codex-memory, and Daily Scout code retained but not started or shown as core
- Installed baseline: unsigned v0.2.0 current-user installer; v0.6.0 package not yet accepted
- Current build: `Memoryling_0.6.0_x64-setup.exe`, 4,940,888 bytes, SHA-256 `D0CB52780FEED79A8522DD7D656F60B7CEFA65C3721F02F26B38C1356109A2F0`, `NotSigned`; built but not installed／accepted
- Repository: public `main` at https://github.com/artpoet/Memoryling

## Implemented v0.6.0 vertical slice

### Agent side

- `AGENTS.md` recognizes `醒來吧我的寵物` and `Wake up, my pet`
- `skills/memoryling-operation/SKILL.md` defines the authorization, minimization, compilation, submission, and reporting workflow
- skill metadata passes the official `skill-creator` validator
- protocol reference and JSON Schema define exact fields, counts, enums, IDs, timestamps, hash format, dialogue length, and delivery bounds
- synthetic package demonstrates structure without user content
- submit helper validates without echoing content, requires a running Memoryling 0.6.0+, atomically renames a UTF-8 file inside the exact inbox, never launches a process, and waits for consumption

### App side

- inbox worker polls only `%LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v1.json`
- accepts only a non-symlink regular file from 1 byte through 64 KiB
- strict Rust deserialization rejects unknown fields and all invalid protocol bounds
- same operation ID＋digest is idempotent; same ID＋different digest fails closed
- each new operation transactionally replaces the prior operation and cascaded evidence／dialogue state
- SQLite migration 0005 stores operation profile, opaque evidence hashes, bilingual dialogue, lineage, counters, runtime state, and daily ambient usage
- opening dialogue is selected on apply; pet clicks request eligible interaction dialogue
- ambient checks are bounded to 15-minute opportunities, 09:00–22:00 delivery, and two lines per local day
- detail clear removes operation, evidence, dialogue, usage state, and render effects
- Rust emits content-free revision events and render-safe DTOs

### UX

- manual cold launch shows the pet immediately; OS locale selects initial language and no setup page blocks the flow
- resident relaunch returns to the existing pet rather than opening detail or creating another instance
- first-run guide, idle pet dialogue, and primary detail panel show the exact bilingual activation phrase
- primary UI no longer asks for a memory connector or API key
- activity changes the pet aura color; milestone state adds a star
- floating pet displays localized dialogue and advances through the local rule engine
- browser preview states that desktop inbox is unavailable and performs no AI or memory read
- English and Traditional Chinese meaning stays in parity

## Privacy and deletion truth

The operation package may contain generated pet state and opaque hashes. It may not contain raw memory, prompts, reasoning, paths, names, secrets, credentials, tool output, or source text. Evidence hashes do not cross the pet render DTO.

The app cannot observe Agent-memory deletion because it deliberately does not scan Agent storage. A later successful operation is an authoritative snapshot and removes all prior operation data. Immediate local removal uses **Clear this pet update**. Neither action writes to or deletes Agent-owned memory.

Automated checks, browser smoke, and submit-helper smoke used synthetic data only. No private Agent memory, prompt, database, or tool-home content was read for v0.6.0 UAT.

## Verification evidence

- PASS — `npm run check`
  - 28／28 frontend tests
  - production TypeScript／Vite build
  - 52／52 runnable Rust tests; one private-source live smoke remains intentionally ignored
  - `cargo check`
- PASS — `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- PASS — `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- PASS — targeted Agent-operation Rust tests prove strict validation, idempotency, conflicting-ID rejection, persistence, render minimization, authoritative replacement, and clear
- PASS — frontend tests prove immediate Agent-operated detail, activity appearance, dialogue interaction, browser honesty, bilingual parity, revision refresh, clear, pet menu／drag／onboarding, safe DTO sanitization, and race handling
- PASS — submit helper rejects a mismatched executable and fails before inbox write when the App is closed; the isolated harness changes no Memoryling process count and removes an unconfirmed inbox item after the bounded wait
- PASS — `npm run build:windows` produced the content-free unsigned v0.6.0 NSIS artifact recorded above
- PASS — submission to the already-running freshly built release binary consumed the four-card synthetic inbox, left no inbox item, and preserved the same one process／PID before and after
- PASS — Computer Use observed the Traditional Chinese first-run guide explicitly say `回到你目前工作的 Agent 專案，輸入發動語：「醒來吧我的寵物」。`; the detailed native surface also showed the three-step installed-App flow
- PASS — official skill validator reported `Skill is valid!` under an isolated temporary PyYAML dependency and UTF-8 mode; system Python was not modified
- PASS — in-app browser smoke observed the new installed-App／activation-phrase copy in Traditional Chinese and English, verified the memory-off boundary, and found no horizontal overflow (`scrollWidth` 1265 = `clientWidth` 1265)
- PASS — `git diff --check`

Installed NSIS launch and running-process handoff remain a separate packaged acceptance gate. Current native proof used the freshly built trusted release binary; it does not claim installer execution or installed-App acceptance.

## Historical baseline and no-redo boundary

- v0.2.0 already proved the current-user NSIS install, pet-first lifecycle, tray／shortcut recovery, fixture persistence／forgetting, explicit Quit, and retained-data uninstall on this host
- its installer was unsigned, local-test-only, and is not a public release
- do not rebuild the shell or repeat historical fixture UAT merely because v0.6.0 changes the semantic source path
- v0.5.0 direct Codex-memory auto-sync is superseded by ADR-0008; do not restore its scheduler or primary UI without a new product decision
- Daily Scout is not the core value path; do not re-enable its scheduler or first-run API setup without explicit instruction and privacy review
- do not read private Agent memory for testing without exact authorization naming source and scope
- do not submit the Codex for Open Source application without a new explicit instruction

## Known gaps

- v0.6.0 has no packaged Windows install／upgrade／manual-launch／running-process handoff／restart／clear／uninstall acceptance
- skill discovery from arbitrary external projects is documented but not installed or automated
- no real Agent-project slogan smoke has been authorized; synthetic package proof covers the protocol, not private semantics
- screenshot／streaming privacy mode remains open before real-context public testing
- mixed-DPI, monitor hot-unplug, taskbar, `Win+B`, Narrator／NVDA, sign-out, and shutdown acceptance remains open from the pet-shell track
- later-stage morphology, growth journal, multi-day stories, reminders, and conversation remain planned
- compatibility connectors should eventually be removed or isolated after migration evidence is no longer needed

## Next coherent bundle

**Install and accept the current v0.6.0 Windows artifact using synthetic data only.**

1. Verify clean install or upgrade from the retained v0.2.0 baseline using the recorded NSIS artifact.
2. Confirm the installed App opens pet-first and displays the exact bilingual activation reminder.
3. Submit only `examples/agent-operation-v1.synthetic.json`.
4. Check no-setup cold launch, idle phrase reminder, running-process submission without auto-launch, inbox pickup, activity appearance, opening dialogue, click dialogue, restart persistence, replacement, clear, manual single-instance pet recovery, and uninstall data choices.
5. Record content-free version／size／checksum／signature evidence and remaining release gaps.

Stop before private-memory UAT, global skill installation, code signing purchase, or public release unless the user explicitly authorizes that expansion.

## Fresh-chat handoff

Read `AGENTS.md` → `AI-WAKEUP.md` → this file → `docs/adr/0010-installed-app-teaches-agent-activation.md` → `docs/adr/0008-agent-operated-memoryling-protocol.md`. State that the installed App owns launch and visible phrase teaching; the Agent owns compilation and submission to an already-running pet; the App owns local life with no app-side AI API. Then execute the packaged synthetic acceptance bundle above end to end, or ask before crossing its explicit stop gates.
