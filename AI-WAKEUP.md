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
- Tagline: Your agent memories, alive.
- Chinese tagline: 讓你的 Agent 記憶，長成一個生命。
- Stage: v0.2.0 fixture-only pet-first vertical slice with current-host Windows x64 NSIS UAT
- Repository: https://github.com/artpoet/Memoryling
- Primary public language: English
- First-class personal language: Traditional Chinese
- License: MIT

## Product in one paragraph

Memoryling is a local-first desktop creature intended to grow from durable memories stored by AI agents. Approved memories eventually shape its appearance, dialogue, habitat, continuing events, and occasional reminders. The differentiator is source-traceable memory causality: every meaningful change should be explainable, correctable, and removable.

## Reality check

Implemented now:

- Tauri 2 desktop shell
- React 19 + TypeScript + Vite bilingual experience
- English and Traditional Chinese interface with remembered locale
- interactive creature plus honestly labeled concept event and bounded-initiative panels
- versioned normalized memory-event schema and a fixed-resource adapter v1 for one bundled synthetic Codex-shaped fixture
- explicit fixture-source selection, in-memory preview token, record selection, and consent gate
- local SQLite v1 store under Tauri app-local data with source lineage
- deterministic completion signal and explainable completion-star world effect
- transactional source forgetting followed by deterministic recomputation
- visible real-memory access remains off; fixture approval is reported separately as a local synthetic pilot
- current-user Windows x64 NSIS test-build configuration with English／Traditional Chinese installer languages
- configured WebView2 download-bootstrapper support; the missing-runtime branch still needs a safe disposable Windows environment
- generated Memoryling test icon and in-app brand asset with PNG transparency checked
- pet-first native desktop shell: transparent floating `pet`, hidden on-demand `main`, native context／keyboard menu, tray recovery, and single-instance behavior
- Rust-owned close／minimize／restore／explicit-Quit lifecycle with content-free shell settings, bounded position recovery, and one-time bilingual onboarding
- exact per-window capabilities plus caller-label guards; `pet` receives only a narrow `CreatureRenderState`, revision events, and its own guarded drag command
- lazy surface routing: native labels are authoritative, browser mode stays an honest detail preview, and the pet bundle does not load full-memory APIs
- current-host v0.2.0 raw-native and normally installed NSIS smoke through fixture approve／restart／lineage／forget, Start Menu single-instance recovery, and retain-data uninstall
- no telemetry and no external font or runtime content dependency in the fixture memory path

Not implemented:

- access to any user-owned Codex or other agent memory
- arbitrary path scanning or a user file picker
- a production connector for an externally selected durable-memory file
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

The current code implements this path end to end for exactly one bundled synthetic Codex-shaped resource. It does not read a user's Codex tool-home or other external files.

The implemented interaction is “two surfaces, one life”: a transparent floating `pet` is normally visible and the standard `main` detail window opens on demand. Rust owns visibility, menu／tray／single-instance transitions, saved bounds, and explicit Quit; right-click or the keyboard menu opens detail, while Start Menu and the installed shortcut recover the same process. The pet reads only a content-minimized render DTO. Browser mode deliberately shows the honest detail preview because it has no trustworthy native window label or resident-shell runtime.

## Key paths

- src/App.tsx — on-demand bilingual detail surface and safe revision refresh
- src/PetSurface.tsx — transparent pet interaction, onboarding, menu, keyboard, and drag behavior
- src/SurfaceRouter.tsx — native-label-authoritative surface routing and browser honesty
- src/creatureClient.ts — whitelisted pet DTO, revision events, and narrow pet commands
- src/useCreatureRenderState.ts — race-safe render-state subscription and refresh
- src/App.css and src/PetSurface.css — detail and transparent pet visual systems
- src/FirstMemoryFlow.tsx — source selection, preview, consent, lineage, and forgetting UI
- src/memoryClient.ts — typed Tauri command boundary
- src-tauri/src/memory/ — strict adapter, pending preview, SQLite store, derivation, and forgetting
- src-tauri/migrations/0001_first_memory.sql — local schema v1
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
- docs/zh-TW/PRODUCT_VISION.md — Traditional Chinese product intent
- docs/drafts/deep-interview-evolving-creature-system-2026-08-11.md — user-confirmed growth direction with proposed implementation details; design only, not implemented
- docs/drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md — five-round PM specification for scoped Agent-memory derivation, information priority, recent-versus-durable growth, and variant acceptance; design only, not implemented
- docs/drafts/pet-first-desktop-shell-2026-08-11.md — implemented v0.2.0 foundation plus remaining Windows acceptance matrix
- docs/research/2026-08-12_codex-for-open-source-readiness.md — official program facts, current public-evidence snapshot, application readiness gates, truthful draft answers, and submission boundaries
- docs/adr/0003-pet-first-two-window-desktop-shell.md — implemented two-window decision under extended acceptance; status remains Proposed
- docs/adr/0004-deterministic-content-derived-evolution-paths.md — proposed local, lineage-aware bounded-variant growth decision
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
- Preserve the visible “real memory access is off” state until a production connector and consent flow are verified, including while the fixture pilot is active.

## Known traps

- The bundled Codex-shaped JSON fixture is not evidence of a real Codex durable-memory format, connector, or completed Phase 1 exit.
- Approved normalized text is stored in app-local SQLite. Never print, stage, commit, attach, or screenshot a real local database.
- “Complete forgetting” is scoped to Memoryling's local imported copy and supported downstream graph; it never modifies the source and is not a physical secure-erasure guarantee.
- Browser preview has no native memory runtime. Do not add mock persistence or present planned behavior as live.
- The supported tester entry is the unsigned current-user NSIS installer. The raw release exe requires its generated fixture sidecar and is not portable.
- Launch installer UAT from normal Windows Explorer. A direct packaged-agent launch can be redirected into the agent package's virtualized LocalAppData; reject that run as harness evidence and clean only its virtualized install／shortcut, never the real app-data tree.
- v0.2.0 UAT verified that leaving delete-app-data clear retained `%LOCALAPPDATA%\app.memoryling.desktop`; the earlier checksum-matched v0.1.0 cycle separately verified the checked-delete option. Neither is a physical secure-erasure guarantee, and no database content was read.
- The host already has WebView2 151.0.4129.78 and no safe Windows Sandbox／Hyper-V clean environment. The missing-runtime bootstrapper branch is deferred; never remove the host runtime just to test it. Any prerequisite download is separate from the network-free fixture memory pipeline.
- Generated test artwork and transparent PNGs are not evidence of signing, store review, or public release readiness.
- Growth drafts and branching concept art are design vocabulary only—not a live renderer, fixed route／pet roster, production sprite set, or real-memory mapping. See ADR-0004 and the two indexed growth drafts for the full boundary.
- Native window labels, not query strings, select the pet surface. Never fake pet／tray behavior in browser mode or widen the pet DTO with memory text.
- Tauri app commands are not safely window-scoped by default. Preserve both layers now in place: exact `main`／`pet` capabilities and Rust caller-label guards, including the real-invoke denial test for all six sensitive memory commands.
- Codex for Open Source is a discretionary OSS-maintainer program, not a product contest or guaranteed `$1,200`. Memoryling is eligible in form but not application-ready: it still lacks a public Release, real-source proof, external adoption, and a demonstrated maintainer loop. Do not re-research the settled program basics every session; refresh official terms and live GitHub metrics immediately before submission.

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

Expected local artifact:

    src-tauri/target/release/bundle/nsis/Memoryling_0.2.0_x64-setup.exe

## Current coherent next bundle

Fresh-session instruction: start with the first unfinished gate in this section and finish that one coherent bundle through verification, SSOT, commit, and push. Do not stop at planning or a native connector skeleton, and do not submit the external application without a new explicit user instruction.

The v0.1.0 current-host installer gate closed on 2026-08-12 and is a no-redo historical baseline: its checksum-matched artifact passed the full fixture tour and both uninstall data choices. The WebView2-missing branch remains deferred to a safe disposable Windows environment; never remove the host runtime to test it.

The v0.2.0 pet-first synthetic gate also closed on 2026-08-12: automated security／lifecycle／DPI-position tests, raw-native fixture UAT, and a normal Explorer-launched current-user install all passed. Installed Start Menu cold／resident launch kept one process, pet↔detail lifecycle worked, explicit Quit ended the process, and unchecked uninstall removed program state while retaining app data. The artifact is unsigned, local-test-only, 2,875,965 bytes, SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`. Extended live DPI／monitor／hitbox／assistive-tech／shutdown checks remain acceptance work, so ADR-0003 stays Proposed and this is not a public release.

First unfinished implementation gate: resume the first user-selected Codex-source pilot by validating one stable supported format, then add a Rust-owned narrow picker and redacted preview. Stop before private-data UAT unless the user explicitly authorizes the exact source and scope. Do not scan tool-home directories, generalize arbitrary filesystem access, or skip to open-ended AI chat. Run the remaining pet-shell environment matrix before public release claims, but do not rebuild the completed v0.2.0 shell to do so.

Later Phase 2 growth must start from the indexed synthetic fixture matrix; do not create a fixed sprite route, per-record consent spam, silent scope expansion, or usage-volume leveling.

Codex for Open Source is not the next bundle. Follow the indexed readiness plan only after the real-source, release, genuine-adoption, and maintainer-loop gates; refresh dynamic evidence immediately before submission. Never manufacture traction, upload private memories for the application, or submit without a new explicit user instruction.

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
