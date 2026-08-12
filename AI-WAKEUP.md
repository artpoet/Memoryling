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
- Stage: fixture-only first-memory vertical slice with a local Windows x64 NSIS test build, v0.1.0
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
- WebView2 download-bootstrapper support when that Windows prerequisite is missing
- generated Memoryling test icon and in-app brand asset with PNG transparency checked
- no telemetry and no external font or runtime content dependency in the fixture memory path

Not implemented:

- access to any user-owned Codex or other agent memory
- arbitrary path scanning or a user file picker
- a production connector for an externally selected durable-memory file
- derivations beyond the deterministic completion-star rule
- the designed identity core, content-derived `EvolutionPathProfile`, lineage-bearing `MorphologyRecipe`, creature genome, adjacent-stage／recipe-change EvolutionBridges, large-form evolution stages, layered renderer, and growth journal
- the user-confirmed pet-first desktop shell: transparent pet window, hidden detail window, native context menu, tray, single-instance recovery, safe position restore, and narrow render-state IPC
- real conversation model
- native reminder delivery
- completed human installer/uninstaller click-through UAT
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

The user-confirmed future interaction direction is “two surfaces, one life”: a floating `pet` is normally visible, while the standard `main` detail window opens on demand. The two-window／Rust lifecycle remains a proposed technical design. Right-click is the primary entry; `Win+B` tray, Start Menu, and packaged installed shortcuts provide recovery. This lifecycle is documented but not implemented—the current app still opens one standard 1180 × 780 window.

## Key paths

- src/App.tsx — bilingual concept behavior and copy
- src/App.css — visual system and CSS creature
- src/FirstMemoryFlow.tsx — source selection, preview, consent, lineage, and forgetting UI
- src/memoryClient.ts — typed Tauri command boundary
- src-tauri/src/memory/ — strict adapter, pending preview, SQLite store, derivation, and forgetting
- src-tauri/migrations/0001_first_memory.sql — local schema v1
- src-tauri/fixtures/codex-first-memory-v1.json — fictional test-only source
- src-tauri/icons/icon-source.png — built-in ImageGen test-art source; not public release-approved
- src-tauri/ — native Tauri shell and minimal capabilities
- package.json — frontend, validation, and Windows NSIS build commands
- src-tauri/tauri.conf.json — current-user NSIS, WebView2 prerequisite, resource, and icon configuration
- docs/USER_GUIDE.md — English Windows x64 fixture-only installation and use guide
- docs/zh-TW/USER_GUIDE.md — Traditional Chinese user guide
- docs/PRODUCT_VISION.md — product intent
- docs/zh-TW/PRODUCT_VISION.md — Traditional Chinese product intent
- docs/drafts/deep-interview-evolving-creature-system-2026-08-11.md — user-confirmed growth direction with proposed implementation details; design only, not implemented
- docs/drafts/pet-first-desktop-shell-2026-08-11.md — user-confirmed pet-only presence plus proposed Windows lifecycle and acceptance plan; design only, not implemented
- docs/adr/0003-pet-first-two-window-desktop-shell.md — proposed two-window, Rust-owned resident-shell decision
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
- Uninstall may retain `%LOCALAPPDATA%\app.memoryling.desktop` unless the delete-app-data option is explicitly selected; that click-through still needs human UAT.
- A missing WebView2 prerequisite may cause installer network access to Microsoft. Do not confuse this with the network-free fixture memory pipeline.
- Generated test artwork and transparent PNGs are not evidence of signing, store review, or public release readiness.
- The evolving-creature design draft is not a live procedural renderer, genome engine, or completed Phase 2 slice.
- The branching concept art is a visual-family and adjacent-transition reference only—not a fixed pre-authored route／pet roster, production sprite set, or one-to-one mapping from an activity axis to a body. The current runtime has no path profile, activity taxonomy, morphology-recipe compiler, or real-memory signal mapping.
- The pet-first design draft is not a transparent window, native menu, tray, single-instance process, or completed desktop shell. Do not fake those native behaviors in browser mode or expose full memory state to a future pet surface.
- Tauri `invoke_handler` app commands are callable by every window by default. A future `pet` must receive generated app-command permissions plus caller-label denial tests; a narrow DTO alone does not protect full memory／approve／forget commands.

## Working conventions

- Keep English and Traditional Chinese user-facing copy meaningfully aligned.
- Prefer small vertical slices that finish UI, logic, tests, privacy behavior, and documentation together.
- Use synthetic fixtures only.
- Record architectural decisions as ADRs.
- Treat route labels as observable approved activity signals, never as sensitive personality or sentiment classifications.
- Treat `EvolutionPathProfile` as a weighted influence vector that may compile into many bounded, versioned `MorphologyRecipe` variants. Only approved durable Agent-activity evidence may affect permanent form; live active／idle／session presence stays ephemeral.
- No live Agent monitoring exists. Any future `LiveAgentPresence` adapter is a separate consented feature: allowlisted enum only, memory-only with TTL, no SQLite／logs／telemetry, neutral by default, and cleared immediately when disabled or unavailable.
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

    src-tauri/target/release/bundle/nsis/Memoryling_0.1.0_x64-setup.exe

## Current coherent next bundle

Close the Windows x64 fixture-only test-build gate:

1. complete a human current-user NSIS install → open → fixture preview／approve／explain／forget → uninstall click-through on Windows x64;
2. verify the uninstall delete-app-data option and inspect `%LOCALAPPDATA%\app.memoryling.desktop` after both retention and deletion choices;
3. exercise the WebView2-missing installer path when a safe clean Windows environment is available;
4. retain the recorded SHA-256 checksum and Windows CI evidence for commit `2aead61`, and regenerate the checksum after every rebuild;
5. decide code-signing and distribution readiness without describing the current test art or unsigned installer as a public release.

After that gate, complete the pet-first shell against synthetic state before widening source access: pre-created `pet`／`main` surfaces, Rust-owned menu／tray／single-instance lifecycle, per-window app-command permissions with pet-denial tests, content-minimized `CreatureRenderState`, close／minimize／restore behavior, position／DPI recovery, approve／forget synchronization, bilingual accessibility, and packaged native smoke. Do not stop at window connectors or a browser mock.

Only after that shell passes should the first user-selected Codex-source pilot resume: validate a stable supported format, add a Rust-owned narrow picker and redacted preview, and require explicit authorization before private-data UAT. Do not scan tool-home directories, generalize arbitrary filesystem access, or skip directly to open-ended AI chat.

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
