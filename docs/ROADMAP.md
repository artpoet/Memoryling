# Roadmap

AS_OF: 2026-08-13. This roadmap describes intent, not a delivery promise.

## Phase 0 — Honest local shell

- [x] Tauri 2 + React + TypeScript foundation
- [x] English and Traditional Chinese parity
- [x] visible memory-off browser and desktop states
- [x] local SQLite state, restrictive CSP, no telemetry or cloud sync
- [x] public repository, governance, CI, and synthetic fixtures
- [x] unsigned v0.2.0 current-user Windows installer UAT
- [ ] WebView2-missing, signing, keyboard, and screen-reader release gates

## Phase 0.5 — Pet-first desktop presence

- [x] transparent floating pet plus on-demand detail window
- [x] native menu, tray, single-instance recovery, explicit Quit
- [x] saved position, work-area clamping, reduced motion, bilingual onboarding
- [x] per-window Tauri capabilities, independent caller guards, render-safe DTO
- [ ] complete mixed-DPI, monitor hot-unplug, taskbar, assistive-tech, sign-out, and shutdown UAT

The verified v0.2.0 installer is a historical no-redo baseline. It does not prove v0.6.0.

## Phase 1 — Agent-operated Memoryling

- [x] accept [ADR-0008](adr/0008-agent-operated-memoryling-protocol.md)
- [x] supersede automatic wake with the installed-App activation flow in [ADR-0010](adr/0010-installed-app-teaches-agent-activation.md)
- [x] add project trigger phrases and `memoryling-operation` Agent skill
- [x] define strict JSON Schema and a synthetic example
- [x] add a no-echo PowerShell validator and atomic local inbox submission
- [x] require an already-running compatible pet, submit without launching, and await bounded application confirmation
- [x] remove the blocking setup gate, keep manual cold／single-instance launch pet-first, and show a persistent bilingual activation reminder
- [x] poll one exact non-symlink inbox file with a 64 KiB cap
- [x] add SQLite schema v5 for operation, hashed evidence, dialogue, counters, and runtime
- [x] apply each package as an authoritative replacement; duplicate ID＋digest is idempotent
- [x] add activity appearance accents and milestone mark
- [x] add bilingual `on-open`, `on-interact`, and `ambient` dialogue
- [x] enforce expiry, cooldown, max uses, 22:00–09:00 quiet hours, and two ambient lines per day
- [x] expose local clear control and render-safe state schema v6
- [x] remove direct memory connector, API key, and Daily Scout from the primary UX
- [x] stop direct Agent-memory and Daily Scout schedulers
- [x] validate with synthetic Rust, frontend, package-helper, and browser evidence
- [ ] packaged v0.6.0 Windows install／upgrade／restart／clear／uninstall UAT
- [ ] install or discover the skill from arbitrary user projects without copying unsafe governance
- [ ] test the slogan in one explicitly authorized non-private sandbox Agent project

Exit condition: the installed EXE opens a pet that teaches the activation phrase; from an Agent project, that phrase creates and replaces a bounded local update in the already-running pet; the app persists and speaks it without scanning Agent storage or making an AI request; clear removes it; packaged Windows behavior is accepted.

**Current status: source vertical slice implemented; packaged acceptance remains open.**

## Phase 2 — A life that continues

- [ ] versioned identity core and persistent growth journal
- [ ] deduplicated activity and outcome evidence groups
- [ ] TTL expression／pose／light hints for recent work
- [ ] deterministic maturity and morphology profile
- [ ] finite versioned module catalog and compatibility matrix
- [ ] deterministic morphology recipe and adjacent evolution bridges
- [ ] correction and forgetting recomputation with no ghost state
- [ ] multi-day story and habitat state machine
- [ ] accessible compact／wide／tall rendering and 200% zoom proof
- [ ] screenshot／streaming privacy mode

Permanent change requires multiple independent, outcome-qualified evidence groups. Time, tokens, session length, source volume, and app-open duration are not XP. Runtime model image generation is not part of shipped growth.

## Phase 3 — Useful initiative

- [x] protocol-v1 quiet hours, cooldowns, expiry, and ambient daily budget
- [ ] reminder-candidate schema separated from dialogue
- [ ] visible reason and source category for every reminder
- [ ] urgency threshold, snooze, global off, and per-topic suppression
- [ ] native notifications and accessibility acceptance
- [ ] helpful／not now／never feedback loop

The old BYOK Daily Scout remains compatibility source code, not the current core roadmap. Any network revival needs a new product decision and privacy review.

## Phase 4 — Optional semantic conversation with the pet

- [ ] explicit provider and privacy decision
- [ ] local-context assembly with redaction controls
- [ ] optional local-model evaluation
- [ ] bilingual grounded conversation
- [ ] explanation and correction for remembered claims

This future in-pet semantic conversation is distinct from the current Agent conversation used as the control surface. It must remain optional; the pet's persistent life cannot depend on a continuous chat API.

## Public beta and OSS evidence track

- [ ] publish a signed, checksum-backed Windows beta
- [ ] ship a 60–90 second synthetic demo with honest current／next labels
- [ ] recruit external testers with reproducible consented evidence
- [ ] complete one public issue → fix → follow-up release loop
- [ ] publish contributor-ready protocol and conformance fixtures
- [ ] refresh all public metrics and program terms before any application

Do not submit the Codex for Open Source application without a new explicit user instruction and real evidence for these gates.
