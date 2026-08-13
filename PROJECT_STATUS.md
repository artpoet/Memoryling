# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-13 (Asia/Taipei)

## Current milestone

Source v0.6.0 now implements the user-confirmed **Agent-operated Memoryling** model end to end with synthetic data:

```text
“運作 Memoryling”／“Run Memoryling”
  → current Agent reads already-authorized context
  → Agent compiles one bounded operation package
  → local app validates, persists, renders, and speaks by deterministic rules
```

The core requires no app-side AI API and no direct app scan of Agent memory. The current source is ready for packaged synthetic acceptance; it is not yet a public release and has not performed private-memory UAT.

## Overall state

- Version: 0.6.0 source vertical slice
- Product surface: bilingual transparent pet plus on-demand detail; browser remains an honest detail-only preview
- Agent integration: project trigger phrases plus validated `memoryling-operation` skill
- Handoff: strict protocol-v1 JSON, PowerShell validation, exact app-local inbox, Rust revalidation
- Local store: SQLite schema v5; only the newest authoritative Agent operation is retained
- Pet boundary: render DTO schema v6 with coarse activity, safe marks, and current dialogue only
- Dialogue: 3–12 bilingual cards with open／interaction／ambient triggers, cooldown, expiry, max uses, quiet hours, and ambient budget
- User control: local clear removes the current derived operation; rerunning the slogan rebuilds from current Agent context
- Core network behavior: none; no API key and no model request
- Legacy compatibility: fixture, one-thread, direct Codex-memory, and Daily Scout code retained but not started or shown as core
- Installed baseline: unsigned v0.2.0 current-user installer; v0.6.0 package not yet accepted
- Repository: public `main` at https://github.com/artpoet/Memoryling

## Implemented v0.6.0 vertical slice

### Agent side

- `AGENTS.md` recognizes `運作 Memoryling`, `執行 Memoryling`, and `Run Memoryling`
- `skills/memoryling-operation/SKILL.md` defines the authorization, minimization, compilation, submission, and reporting workflow
- skill metadata passes the official `skill-creator` validator
- protocol reference and JSON Schema define exact fields, counts, enums, IDs, timestamps, hash format, dialogue length, and delivery bounds
- synthetic package demonstrates structure without user content
- submit helper validates without echoing content and atomically renames a UTF-8 file inside the exact inbox

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

- first run asks only for language and explains the Agent-operated route
- primary detail panel shows one slogan and the three responsibility steps
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
  - 32／32 frontend tests
  - production TypeScript／Vite build
  - 52／52 runnable Rust tests; one private-source live smoke remains intentionally ignored
  - `cargo check`
- PASS — `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
- PASS — `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- PASS — targeted Agent-operation Rust tests prove strict validation, idempotency, conflicting-ID rejection, persistence, render minimization, authoritative replacement, and clear
- PASS — frontend tests prove Agent-operated detail／setup, activity appearance, dialogue interaction, browser honesty, bilingual parity, revision refresh, clear, pet menu／drag／onboarding, safe DTO sanitization, and race handling
- PASS — submit helper wrote the four-card synthetic example to an isolated temporary app-data inbox and printed only operation ID＋count
- PASS — official skill validator reported `Skill is valid!` under an isolated temporary PyYAML dependency and UTF-8 mode; system Python was not modified
- PASS — in-app browser smoke observed the live local page in Traditional Chinese and English, verified the slogan／three-step flow／memory-off boundary, found no horizontal overflow, and reported zero console warnings or errors
- PASS — `git diff --check`

## Historical baseline and no-redo boundary

- v0.2.0 already proved the current-user NSIS install, pet-first lifecycle, tray／shortcut recovery, fixture persistence／forgetting, explicit Quit, and retained-data uninstall on this host
- its installer was unsigned, local-test-only, and is not a public release
- do not rebuild the shell or repeat historical fixture UAT merely because v0.6.0 changes the semantic source path
- v0.5.0 direct Codex-memory auto-sync is superseded by ADR-0008; do not restore its scheduler or primary UI without a new product decision
- Daily Scout is not the core value path; do not re-enable its scheduler or first-run API setup without explicit instruction and privacy review
- do not read private Agent memory for testing without exact authorization naming source and scope
- do not submit the Codex for Open Source application without a new explicit instruction

## Known gaps

- v0.6.0 has no packaged Windows install／upgrade／restart／clear／uninstall acceptance
- skill discovery from arbitrary external projects is documented but not installed or automated
- no real Agent-project slogan smoke has been authorized; synthetic package proof covers the protocol, not private semantics
- screenshot／streaming privacy mode remains open before real-context public testing
- mixed-DPI, monitor hot-unplug, taskbar, `Win+B`, Narrator／NVDA, sign-out, and shutdown acceptance remains open from the pet-shell track
- later-stage morphology, growth journal, multi-day stories, reminders, and conversation remain planned
- compatibility connectors should eventually be removed or isolated after migration evidence is no longer needed

## Next coherent bundle

**Package and accept v0.6.0 on Windows using synthetic data only.**

1. Build the current-user NSIS artifact.
2. Verify clean install or upgrade from the retained v0.2.0 baseline.
3. Submit only `examples/agent-operation-v1.synthetic.json`.
4. Check first run, inbox pickup, activity appearance, opening dialogue, click dialogue, restart persistence, replacement, clear, single-instance recovery, and uninstall data choices.
5. Record content-free version／size／checksum／signature evidence and remaining release gaps.

Stop before private-memory UAT, global skill installation, code signing purchase, or public release unless the user explicitly authorizes that expansion.

## Fresh-chat handoff

Read `AGENTS.md` → `AI-WAKEUP.md` → this file → `docs/adr/0008-agent-operated-memoryling-protocol.md`. State that v0.6.0 source now implements slogan-driven Agent compilation and local rule-driven life with no app-side AI API. Then execute the packaged synthetic acceptance bundle above end to end, or ask before crossing its explicit stop gates.
