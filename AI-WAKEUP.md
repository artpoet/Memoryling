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
- Stage: bilingual interactive concept shell, v0.1.0
- Repository: https://github.com/artpoet/Memoryling
- Primary public language: English
- First-class personal language: Traditional Chinese
- License: MIT

## Product in one paragraph

Memoryling is a local-first desktop creature intended to grow from durable memories stored by AI agents. Approved memories eventually shape its appearance, dialogue, habitat, continuing events, and occasional reminders. The differentiator is source-traceable memory causality: every meaningful change should be explainable, correctable, and removable.

## Reality check

Implemented now:

- Tauri 2 desktop shell
- React 19 + TypeScript + Vite concept UI
- English and Traditional Chinese interface with remembered locale
- interactive sample creature, memory signals, continuing event, and bounded-initiative panel
- strict visible label that memory access is off
- no telemetry and no external font or runtime content dependency

Not implemented:

- any real memory access
- source selection or import consent
- connector framework
- normalized memory schema or local database
- derivation engine
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

The current code implements only the final desktop-experience shell.

## Key paths

- src/App.tsx — bilingual concept behavior and copy
- src/App.css — visual system and CSS creature
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
- Preserve the “memory access is off” label until the first connector and consent gate are genuinely functional.

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

Build the first-memory vertical slice:

1. define a versioned normalized event schema;
2. implement a read-only Codex durable-memory adapter against synthetic fixtures;
3. add explicit source selection and an import preview;
4. store approved records locally with source lineage;
5. create one deterministic, explainable creature mark;
6. prove source deletion removes or recomputes that mark;
7. update bilingual UI, tests, architecture docs, and PROJECT_STATUS.md.

Do not skip directly to open-ended AI chat.

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
