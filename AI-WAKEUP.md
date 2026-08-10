# AI-WAKEUP — Memoryling

> Canonical entry point for AI agents working in this repository.

## Read order

1. This file
2. PROJECT_STATUS.md
3. README.md
4. docs/ARCHITECTURE.md
5. docs/PRIVACY_PRINCIPLES.md
6. docs/ROADMAP.md
7. AGENTS.md
8. Relevant ADRs under docs/adr/

## Project identity

- Product: Memoryling｜記憶獸
- Tagline: Your agent memories, alive.
- Chinese tagline: 讓你的 Agent 記憶，長成一個生命。
- Stage: fixture-backed first-memory vertical slice, v0.1.0
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
- no telemetry and no external font or runtime content dependency

Not implemented:

- access to any user-owned Codex or other agent memory
- arbitrary path scanning or a user file picker
- a production connector for an externally selected durable-memory file
- derivations beyond the deterministic completion-star rule
- real conversation model
- native reminder delivery
- packaged public releases

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

## Key paths

- src/App.tsx — bilingual concept behavior and copy
- src/App.css — visual system and CSS creature
- src/FirstMemoryFlow.tsx — source selection, preview, consent, lineage, and forgetting UI
- src/memoryClient.ts — typed Tauri command boundary
- src-tauri/src/memory/ — strict adapter, pending preview, SQLite store, derivation, and forgetting
- src-tauri/migrations/0001_first_memory.sql — local schema v1
- src-tauri/fixtures/codex-first-memory-v1.json — fictional test-only source
- src-tauri/ — native Tauri shell and minimal capabilities
- docs/PRODUCT_VISION.md — product intent
- docs/zh-TW/PRODUCT_VISION.md — Traditional Chinese product intent
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

## Working conventions

- Keep English and Traditional Chinese user-facing copy meaningfully aligned.
- Prefer small vertical slices that finish UI, logic, tests, privacy behavior, and documentation together.
- Use synthetic fixtures only.
- Record architectural decisions as ADRs.
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

## Current coherent next bundle

Prepare the first user-selected Codex-source pilot without weakening the proven fixture path:

1. verify and document a stable public Codex durable-memory format instead of treating private `MEMORY.md` content as a specification;
2. add a Rust-owned native file picker with regular-file, size, UTF-8, canonicalization, and strict-format checks;
3. keep preview data in RAM and bind approval to its pending token;
4. add redaction and invalid-input tests before any signed-in or private-data UAT;
5. run real-data UAT only with an explicitly selected source and explicit user authorization.

Do not scan tool-home directories, generalize arbitrary filesystem access, or skip directly to open-ended AI chat.

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
