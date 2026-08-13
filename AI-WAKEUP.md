# AI-WAKEUP — Memoryling

> Canonical entry point for AI agents working in this repository.

## Read order

1. This file
2. PROJECT_STATUS.md
3. README.md
4. docs/ARCHITECTURE.md
5. docs/PRIVACY_PRINCIPLES.md
6. docs/ROADMAP.md
7. docs/USER_GUIDE.md
8. AGENTS.md
9. Relevant ADRs under docs/adr/

## Project identity

- Product: Memoryling｜記憶獸
- Stage: v0.4.0 source-only Daily Memory Scout on the experimental work-record pilot and completed v0.2.0 pet-first／NSIS baseline
- Repository: https://github.com/artpoet/Memoryling
- Primary public language: English
- First-class personal language: Traditional Chinese
- License: MIT

## Product in one paragraph

Memoryling is a local-first desktop creature intended to grow from approved Agent memories. Its differentiator is source-traceable causality: every meaningful change must be explainable, correctable, and removable.

## Reality check

Implemented now:

- Tauri 2 desktop shell
- React 19 + TypeScript + Vite bilingual experience
- English and Traditional Chinese interface with remembered locale
- interactive creature plus honestly labeled concept event and bounded-initiative panels
- versioned normalized memory events, bundled synthetic fixture, experimental work-record adapter, and SQLite schema v3
- explicit fixture-source selection, in-memory preview token, record selection, and consent gate
- local SQLite v3 store under Tauri app-local data with import／Daily Scout consent, attempt budget, citations, and source lineage
- deterministic completion signal and explainable completion-star world effect
- transactional source forgetting followed by deterministic recomputation
- visible real-memory access remains off; fixture approval is reported separately as a local synthetic pilot
- pet-first native desktop shell: transparent floating `pet`, hidden on-demand `main`, native context／keyboard menu, tray recovery, and single-instance behavior
- Rust-owned close／minimize／restore／explicit-Quit lifecycle with content-free shell settings, bounded position recovery, and one-time bilingual onboarding
- exact per-window capabilities plus caller-label guards; `pet` receives only a narrow `CreatureRenderState`, revision events, and its own guarded drag command
- lazy surface routing: native labels are authoritative, browser mode stays an honest detail preview, and the pet bundle does not load full-memory APIs
- a Rust-owned, exact-version `codex app-server` pilot for user-triggered Codex work-record listing and one explicitly selected thread read; it is not durable-memory access
- content-minimized catalogs, Rust-only raw IDs／final text before approval, redacted preview, exact consent hash, single-active-source enforcement, content-free external lineage, and local forgetting
- one shared 10-second connector deadline, bounded process cleanup, stale-session／concurrent-source guards, and dual ACL／caller denial coverage for eight sensitive commands
- optional Daily Memory Scout: off by default, user BYOK in Windows Credential Manager, visible coarse-context preview, explicit consent, fixed Rust-only OpenAI Responses／Web Search, pinned `gpt-5.6-terra`, `store: false`, and annotation-only citations
- transactional one-attempt-per-local-date guard, no same-day automatic retry／backfill, source-linked forgetting, and main-only controls; the pet receives only neutral `off`／`waiting`／`ready`
- no telemetry; the ordinary pet, fixture, and work-record connector remain API-free

Not implemented:

- a supported API／schema for Codex durable memories or any production durable-memory connector
- arbitrary path scanning, tool-home parsing, or a user file picker
- authorized private-thread UAT, paid Daily Scout live smoke, or a packaged v0.4.0 installer
- derivations beyond the deterministic completion-star rule
- the designed identity core, content-derived `EvolutionPathProfile`, lineage-bearing `MorphologyRecipe`, creature genome, adjacent-stage／recipe-change EvolutionBridges, large-form evolution stages, layered renderer, and growth journal
- real conversation model
- native reminder delivery
- full live pet-shell acceptance at 125／150／175／200% and mixed DPI, monitor hot-unplug／taskbar relocation, adjacent-desktop hitbox, Win+B keyboard tray access, Narrator／NVDA, and sign-out／shutdown
- WebView2-missing installer UAT in a safe disposable Windows environment
- code signing or a public release-ready package

Do not describe roadmap items as working features.

## Architecture map

    approved durable-memory source
        → read-only adapter
        → preview + explicit consent
        → normalized local event
        → derivation engine
        → lineage-aware local store
        → trait / story / dialogue / reminder candidate
        → bilingual desktop experience

The fixture implements this path for one bundled synthetic resource. The v0.4.0 source also includes the exact-version Codex work-record pilot: only a user-selected completed final answer can enter the local pipeline after redacted preview and consent. It neither reads durable-memory state nor scans arbitrary files.

Daily Scout is a separate opt-in branch: approved work event → deterministic coarse allowlist context → purpose-specific consent → one reserved local-date attempt → fixed OpenAI Web Search → validated cited insight. Synthetic fixtures and raw source prose are ineligible.

The shell is “two surfaces, one life”: Rust owns the transparent `pet`, on-demand `main`, menu／tray／single-instance lifecycle, saved bounds, and Quit. The pet receives only a minimized DTO; browser mode stays an honest detail preview.

## Key paths

- src/App.tsx — on-demand bilingual detail surface and safe revision refresh
- src/PetSurface.tsx — transparent pet interaction, onboarding, menu, keyboard, and drag behavior
- src/SurfaceRouter.tsx — native-label-authoritative surface routing and browser honesty
- src/creatureClient.ts — whitelisted pet DTO, revision events, and narrow pet commands
- src/useCreatureRenderState.ts — race-safe render-state subscription and refresh
- src/App.css and src/PetSurface.css — detail and transparent pet visual systems
- src/FirstMemoryFlow.tsx — source selection, preview, consent, lineage, and forgetting UI
- src/memoryClient.ts — typed Tauri command boundary
- src/DailyScoutPanel.tsx and dailyScoutClient.ts — bilingual BYOK, context consent, status, cited insight, and controls
- src-tauri/src/memory/ — fixture adapter, version-bound work-record pilot, pending consent, SQLite store, derivation, and forgetting
- src-tauri/src/daily_scout/ — coarse compiler, credential vault, fixed OpenAI client, scheduler, daily budget, lineage, and commands
- src-tauri/src/memory/codex_thread.rs — bounded local App Server process, minimized catalog, selected-record parser, and consent binding
- src-tauri/migrations/0001_first_memory.sql through 0003_daily_memory_scout.sql — local schema v3
- src-tauri/fixtures/codex-first-memory-v1.json — fictional test-only source
- src-tauri/icons/icon-source.png — built-in ImageGen test-art source; not public release-approved
- src-tauri/src/desktop_shell/ — Rust-owned resident lifecycle, tray／menu, settings, position, and recovery
- src-tauri/src/caller.rs — unspoofable window／webview caller-label guards
- src-tauri/capabilities/main.json and pet.json — exact per-surface capabilities
- package.json — frontend, validation, and Windows NSIS build commands
- src-tauri/tauri.conf.json — current-user NSIS, WebView2 prerequisite, resource, and icon configuration
- docs/USER_GUIDE.md — English Windows x64 fixture-only installation and use guide
- docs/zh-TW/USER_GUIDE.md — Traditional Chinese user guide
- docs/PRODUCT_VISION.md — product intent
- docs/drafts/deep-interview-evolving-creature-system-2026-08-11.md and deep-interview-agent-memory-variation-rules-2026-08-12.md — proposed growth／variation designs; not implemented
- docs/research/2026-08-12_codex-for-open-source-readiness.md — official program facts, current public-evidence snapshot, application readiness gates, truthful draft answers, and submission boundaries
- docs/adr/0003-pet-first-two-window-desktop-shell.md — implemented two-window decision under extended acceptance; status remains Proposed
- docs/adr/0004-deterministic-content-derived-evolution-paths.md — proposed local, lineage-aware bounded-variant growth decision
- docs/adr/0005-codex-thread-history-source-pilot.md — proposed version-bound experimental work-record pilot contract
- docs/adr/0006-optional-byok-daily-memory-scout.md — proposed optional online boundary and acceptance gates
- docs/ARCHITECTURE.md — intended system and connector contract
- docs/PRIVACY_PRINCIPLES.md — non-negotiable trust model
- docs/ROADMAP.md — staged delivery
- docs/adr/ — architecture decisions
- PROJECT_STATUS.md — live state, evidence, and next bundle

## Hard boundaries

- Never commit, log, display, or upload real agent memories, private prompts, credentials, tokens, local databases, or tool-home content.
- Do not scan arbitrary home directories.
- Connectors must be source-specific, user-approved, and read-only.
- Never write into another agent's memory store.
- Derived effects must retain source lineage.
- Forgetting must delete or recompute downstream effects.
- Reminder initiative must respect quiet hours, daily budgets, snooze state, and a global off switch.
- Any network transmission of memory-derived content requires a dedicated ADR, explicit consent, and a visible data-flow explanation.
- Preserve Daily Scout's fixed OpenAI／Web-Search-only, coarse-context, one-attempt-per-date boundary. A generic continuation is not paid-request authorization.
- Preserve visible durable／real-memory access as off; the work-record pilot must be labeled separately and cannot imply access to Codex memories.

## Known traps

- Official Codex documentation exposes no stable durable-memory export API or promised file schema. Do not parse `~/.codex/memories`, call work records “Codex memories,” or present the version-bound App Server pilot as a production connector.
- The pilot accepts only exact local CLI `0.134.0`, lists content-minimized candidates on user action, and reads one selected thread only. A version mismatch fails closed; private UAT requires exact source authorization.
- Approved normalized text is stored in app-local SQLite. Never print, stage, commit, attach, or screenshot a real local database.
- “Complete forgetting” is scoped to Memoryling's local imported copy and supported downstream graph; it never modifies the source and is not a physical secure-erasure guarantee.
- Browser preview has no native memory runtime. Do not add mock persistence or present planned behavior as live.
- The supported tester entry is the unsigned current-user NSIS installer. The raw release exe requires its generated fixture sidecar and is not portable.
- Installer UAT must start from normal Explorer; reject agent-package LocalAppData virtualization as evidence. v0.2.0 verified retain-data uninstall, while v0.1.0 historically verified both choices; no database content was read.
- The host already has WebView2 151.0.4129.78 and no safe Windows Sandbox／Hyper-V clean environment. The missing-runtime bootstrapper branch is deferred; never remove the host runtime just to test it. Any prerequisite download is separate from the network-free fixture memory pipeline.
- Generated test artwork and transparent PNGs are not evidence of signing, store review, or public release readiness.
- Growth drafts and branching concept art are design vocabulary only—not a live renderer, fixed route／pet roster, production sprite set, or real-memory mapping. See ADR-0004 and the two indexed growth drafts for the full boundary.
- Native window labels, not query strings, select the pet surface. Never fake pet／tray behavior in browser mode or widen the pet DTO with memory text.
- Tauri app commands are not safely window-scoped by default. Preserve exact capabilities plus Rust caller-label guards and real-invoke denial tests for every sensitive memory and Daily Scout command.
- `store: false` is not zero retention. The key must remain in Windows Credential Manager and never return to the WebView; do not search with a real key without explicit paid-smoke authorization.
- Codex for Open Source is a discretionary maintainer program, not a contest or guaranteed benefit. Memoryling still lacks a public Release, source proof, adoption, and a maintainer loop; refresh live evidence only before an authorized submission.

## Working conventions

- Keep English and Traditional Chinese user-facing copy meaningfully aligned.
- Prefer small vertical slices that finish UI, logic, tests, privacy behavior, and documentation together.
- Use synthetic fixtures only.
- Record architectural decisions as ADRs.
- Treat route labels as observable approved activity signals, never as sensitive personality or sentiment classifications.
- For future growth, follow the indexed variation draft and ADR-0004: exact source scope, lineage-backed bounded recipes, activity-first morphology, outcome-gated durability, ephemeral recent hints, and no usage-volume XP or silent monitoring.
- Update PROJECT_STATUS.md after a meaningful change.
- Avoid widening Tauri capabilities without a demonstrated need.

## Commands

Install:

    npm install

Frontend:

    npm run dev

Desktop:

    npm run tauri dev

Validation:

    npm run check
    cargo fmt --manifest-path src-tauri/Cargo.toml --check

Windows x64 current-user NSIS test build:

    npm run build:windows

Last installed-UAT artifact (v0.2.0; do not silently substitute a v0.4.0 build):

    src-tauri/target/release/bundle/nsis/Memoryling_0.2.0_x64-setup.exe

## Current coherent next bundle

Fresh-session instruction: start with the first unfinished gate in this section and finish that one coherent bundle through verification, SSOT, commit, and push. Do not stop at planning or a native connector skeleton, and do not submit the external application without a new explicit user instruction.

No-redo baseline: v0.1.0 closed the full fixture／both-uninstall cycle; v0.2.0 closed pet security／lifecycle, raw fixture UAT, and normal installed smoke. Its exact unsigned NSIS is 2,875,965 bytes, SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`. Do not rebuild it to repeat closed gates.

The v0.4.0 Daily Scout source slice is complete through synthetic tests and visible UI smoke: ADR-0006, SQLite v3, Windows Credential Manager BYOK, minimized context, Rust-only OpenAI boundary, daily budget, citations, deletion, and bilingual UX. No real key／paid request, private record, or v0.4.0 package is accepted.

Authorization-gated: a real API smoke requires explicit paid-request approval and key entry through the product; private UAT separately requires one user-named work record and exact read／storage／derivation scope. Generic continuation authorizes neither. Use `PROJECT_STATUS.md` → **Fresh-chat handoff** for safe work. ADR-0003／0005／0006 stay Proposed.

Later Phase 2 growth must start from the indexed synthetic fixture matrix; do not create a fixed sprite route, per-record consent spam, silent scope expansion, or usage-volume leveling.

Codex for Open Source is not the next bundle. Do not submit without a new explicit instruction; first close private pilot UAT, release evidence, genuine adoption, and a maintainer feedback loop.

## Closeout checklist

Before ending a change bundle:

1. run the relevant tests plus npm run check;
2. run cargo fmt check;
3. smoke-check visible UI changes;
4. verify no real memory, local database, or secret is staged;
5. update PROJECT_STATUS.md and affected docs;
6. inspect git diff and git status;
7. commit and non-force push the current branch when authorized;
8. verify the remote branch and CI state;
9. update this file if architecture, boundaries, read order, or the next bundle changed.
