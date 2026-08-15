# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-15 (Asia/Taipei)

## Current milestone

Memoryling is publicly positioned as an open-source, local-first **Agent companion layer**: the creature is the emotional interface for traceable growth, dialogue, and bounded reflection, rather than the whole product. Source v0.7.0 implements the user-confirmed **memory-grounded daily growth and rolling dialogue** model end to end with synthetic data:

```text
install and open Memoryling EXE
  → pet visibly teaches and copies “醒來吧我的寵物”／“Memoryling, wake up”
  → user enters the phrase in the current Agent project
  → current Agent reads already-authorized context
  → Agent first verifies the App is open, then compiles one bounded 48-card package
  → Agent submits to the already-running pet without launching a process
  → local app rolls useful dialogue forward, gates daily appearance, renders, and speaks by deterministic rules
```

The core requires no app-side AI API and no direct app scan of Agent memory. The current source is entering packaged synthetic acceptance; it is not yet a public release and has not performed private-memory UAT.

## Overall state

- Version: 0.7.0 source vertical slice
- Product surface: bilingual transparent pet plus on-demand detail; browser remains an honest detail-only preview
- Agent integration: project trigger phrases plus validated `memoryling-operation` skill
- Primary control: installed App for launch and visible phrase teaching; Agent conversation for semantic updates
- Handoff: strict protocol-v2 JSON, pre-read／pre-submit running-process validation, exact app-local inbox, Rust revalidation
- Local store: SQLite schema v6; newest semantic operation, retained counters for unchanged cards, and current／pending appearance only
- Pet boundary: render DTO schema v6 with coarse activity, safe marks, and current dialogue only
- Dialogue: exactly 48 bilingual cards with opening／interaction／ambient／appearance categories, rolling usage, cooldown, expiry, max uses, quiet hours, and ambient budget
- Appearance: evidence-qualified; at most one visible persistent change per local day and one pending plan
- User control: speech bubble dismisses locally; clear removes operation, counters, and current／pending appearance; rerunning the slogan rebuilds from current Agent context
- Core network behavior: none; no API key and no model request
- Future utility positioning: the opt-in Daily Memory Scout roadmap may use user-reviewed, minimized signals for cited, task-relevant suggestions; it remains inactive compatibility／research code in v0.7.0
- Legacy compatibility: fixture, one-thread, direct Codex-memory, and Daily Scout code retained but not started or shown as core
- Public presentation: GitHub uses `docs/assets/memoryling-social-preview.jpg` as the repository social preview; the 1280×640, 133,310-byte image presents the Agent-operated, local-first, privacy-minimized positioning without claiming inactive features
- Installed baseline: unsigned v0.2.0 current-user installer; v0.7.0 package not yet accepted
- Current build: `Memoryling_0.7.0_x64-setup.exe`, 4,966,744 bytes, SHA-256 `5EB39108F3468FB6DB383C70C055F8B3B11D0958F3EB904EEA991FB98FEA741B`, `NotSigned`; built but not installed／accepted
- Repository: public `main` at https://github.com/artpoet/Memoryling; completed functional baseline `899258e` with CI run `31803437123` successful

## Implemented v0.7.0 vertical slice

### Agent side

- `AGENTS.md` recognizes the exact pet phrases `醒來吧我的寵物` and `Memoryling, wake up`, while project／Agent wake-up requests remain context-only
- `skills/memoryling-operation/SKILL.md` defines the authorization, minimization, compilation, submission, and reporting workflow
- skill metadata passes the official `skill-creator` validator
- protocol-v2 reference and JSON Schema require one appearance plan and exactly 48 cards in the 8／20／16／4 category split
- committed synthetic package demonstrates the full structure without user content
- submit helper has a readiness-only mode used before memory read, validates without echoing content, requires a running Memoryling 0.7.0+, atomically renames a UTF-8 file inside the exact inbox, never launches a process, and waits for consumption
- missing／closed／stale／mismatched App states return one content-free readiness code; the Agent presents a fixed locale-appropriate install／open reminder and stops before memory read or inbox write

### App side

- inbox worker polls only `%LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v2.json`
- accepts only a non-symlink regular file from 1 byte through 64 KiB
- strict Rust deserialization rejects unknown fields and all invalid protocol bounds
- same operation ID＋digest is idempotent; same ID＋different digest fails closed
- each new operation transactionally replaces prior semantic content while preserving usage only for identical stable dialogue IDs and text
- SQLite migration 0006 stores operation profile, opaque evidence hashes, category／theme／semantic dialogue identity, rolling counters, current／pending appearance lineage, and daily usage gates
- evidence-qualified appearance applies at most once per local day; a later same-day qualified plan is pending for the next eligible day
- opening or appearance dialogue is selected on apply; pet clicks request least-used eligible interaction dialogue with two-second anti-stack
- ambient checks use a local 35–70 minute cadence, 09:00–22:00 delivery, ten-minute spacing after any line, and seven lines per local day
- detail clear removes operation, evidence, dialogue, usage state, current／pending appearance, and render effects
- Rust emits content-free revision events and render-safe DTOs

### UX

- manual cold launch shows the pet immediately; OS locale selects initial language and no setup page blocks the flow
- resident relaunch returns to the existing pet rather than opening detail or creating another instance
- first-run guide shows the exact bilingual activation phrase with a local copy button, visible result feedback, and an explicit instruction to paste it into the Agent chat for the current project; idle pet dialogue and the primary detail panel keep showing the phrase
- primary UI no longer asks for a memory connector or API key
- activity changes the pet aura color; milestone state adds a star
- floating pet displays localized operation dialogue in a styled three-line speech bubble that can be dismissed and auto-hides after seven seconds
- browser preview states that desktop inbox is unavailable and performs no AI or memory read
- English and Traditional Chinese meaning stays in parity

## Privacy and deletion truth

The operation package may contain generated pet state and opaque hashes. It may not contain raw memory, prompts, reasoning, paths, names, secrets, credentials, tool output, or source text. Evidence hashes do not cross the pet render DTO.

The app cannot observe Agent-memory deletion because it deliberately does not scan Agent storage. A later successful operation is an authoritative semantic snapshot: unchanged stable dialogue may keep only counters, while absent or changed content is deleted. Immediate local removal uses **Clear this pet update**. Neither action writes to or deletes Agent-owned memory.

Automated checks and submit-helper smoke use synthetic data only. No private Agent memory, prompt, database, or tool-home content is authorized for v0.7.0 UAT.

## Verification evidence

- PASS — `npm run check`
  - 31／31 frontend tests
  - production TypeScript／Vite build
  - 53／53 runnable Rust tests; one private-source live smoke remains intentionally ignored
  - `cargo check`
- PASS — `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- PASS — `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- PASS — targeted Agent-operation Rust tests prove strict v2 validation, idempotency, conflicting-ID rejection, rolling usage retention, seven-line ambient cap, daily appearance queue, persistence, render minimization, replacement, and clear
- PASS — frontend tests prove exact activation copy, immediate Agent-operated detail, activity appearance, dismissible speech bubble, dialogue interaction, browser honesty, bilingual parity, revision refresh, clear, pet menu／drag／onboarding, safe DTO sanitization, and race handling
- PASS — submit helper rejects a mismatched executable and fails before inbox write when the App is closed; the isolated harness changes no Memoryling process count and removes an unconfirmed inbox item after the bounded wait
- PASS — readiness-only helper failure returns `MEMORYLING_APP_NOT_READY` with install／open guidance, accepts no package path, and creates no inbox item; the Agent skill maps it to the fixed Traditional Chinese／English reminder before any memory read
- PASS — `npm run build:windows` produced the content-free unsigned v0.7.0 NSIS artifact recorded above; the release binary reports product／file version 0.7.0 and product name `Memoryling`
- PASS — the freshly built release pet consumed `operation.synthetic-v2-001` with all 48 cards, removed `operation-v2.json`, and preserved one process with the same PID before／after
- PASS — Computer Use observed the idle Traditional Chinese pet show `醒來吧我的寵物`, then observed the evidence-qualified accent／completion mark, two distinct click dialogues in styled speech bubbles, accessible dismiss labels, and seven-second auto-hide
- PASS — official skill validator reported `Skill is valid!` under an isolated temporary PyYAML dependency and UTF-8 mode; system Python was not modified
- PASS — refreshed v0.7.0 native desktop visible smoke; automated frontend coverage separately proves exact clipboard text and click dismissal
- PASS — GitHub Issue #1 first-use feedback is implemented and closed as completed at functional commit `899258e`: targeted onboarding coverage proves the bilingual paste destination and accessible copy-button description, while native 360×430 desktop smoke shows the complete Traditional Chinese and English guides without clipping either action
- PASS — GitHub Settings visibly shows the custom social preview; the public repository `og:image` resolves to `repository-images.githubusercontent.com` and downloads as the exact 1280×640, 133,310-byte asset with SHA-256 `FE71469D205B2A14C0199A2D56415E51343C0520C8FB4F8BC0B943A76505CFBA`
- PASS — `git diff --check`

Installed NSIS launch and running-process handoff remain a separate packaged acceptance gate. Current native proof used the freshly built trusted release binary; it does not claim installer execution or installed-App acceptance.

## Historical baseline and no-redo boundary

- v0.2.0 already proved the current-user NSIS install, pet-first lifecycle, tray／shortcut recovery, fixture persistence／forgetting, explicit Quit, and retained-data uninstall on this host
- its installer was unsigned, local-test-only, and is not a public release
- do not rebuild the shell or repeat historical fixture UAT merely because v0.7.0 changes the semantic source path
- v0.5.0 direct Codex-memory auto-sync is superseded by ADR-0008; do not restore its scheduler or primary UI without a new product decision
- Daily Scout is not the core value path; do not re-enable its scheduler or first-run API setup without explicit instruction and privacy review
- do not read private Agent memory for testing without exact authorization naming source and scope
- do not submit the Codex for Open Source application without a new explicit instruction

## Known gaps

- v0.7.0 has no packaged Windows install／upgrade／manual-launch／running-process handoff／restart／clear／uninstall acceptance yet
- skill discovery from arbitrary external projects is documented but not installed or automated
- no real Agent-project slogan smoke has been authorized; synthetic package proof covers the protocol, not private semantics
- screenshot／streaming privacy mode remains open before real-context public testing
- mixed-DPI, monitor hot-unplug, taskbar, `Win+B`, Narrator／NVDA, sign-out, and shutdown acceptance remains open from the pet-shell track
- richer morphology, growth journal, multi-day stories, reminders, and semantic pet conversation remain planned; protocol v2 only ships evidence-gated activity accents／milestone marks
- compatibility connectors should eventually be removed or isolated after migration evidence is no longer needed

## Next coherent bundle

**Install and accept the current v0.7.0 Windows artifact using synthetic data only.**

1. Verify clean install or upgrade from the retained v0.2.0 baseline using `src-tauri/target/release/bundle/nsis/Memoryling_0.7.0_x64-setup.exe` and the recorded checksum／size／signature above.
2. Confirm the installed App opens pet-first, displays the exact bilingual activation reminder, and copies the locale-specific phrase.
3. Submit only `examples/agent-operation-v2.synthetic.json`.
4. Check no-setup cold launch, idle phrase reminder, running-process submission without auto-launch, inbox pickup, daily appearance gate／pending plan, opening／click／ambient dialogue, rolling counters, speech-bubble dismissal, restart persistence, replacement, clear, manual single-instance pet recovery, and uninstall data choices.
5. Record content-free version／size／checksum／signature evidence and remaining release gaps.

Stop before private-memory UAT, global skill installation, code signing purchase, or public release unless the user explicitly authorizes that expansion.

## Fresh-chat handoff

Read `AGENTS.md` → `AI-WAKEUP.md` → this file → `docs/adr/0011-memory-grounded-daily-growth-and-dialogue-v2.md` → `docs/adr/0010-installed-app-teaches-agent-activation.md`. Treat functional commit `899258e`, successful CI run `31803437123`, the active social preview, and closed Issue #1 as completed baselines—do not redo them. In no more than six short Traditional Chinese bullets, state that the installed App owns launch and visible phrase teaching; the Agent owns authorized compilation; the App owns rolling dialogue, daily appearance gating, and local life with no app-side AI API. Present the packaged synthetic acceptance bundle, state that it needs no private memory, API key, or paid call, and wait for the user's explicit start instruction. Once approved, execute it end to end; ask again before crossing any stop gate.
