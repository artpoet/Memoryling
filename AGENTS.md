# AGENTS.md

## Start here

Read, in order:

1. AI-WAKEUP.md
2. PROJECT_STATUS.md
3. README.md
4. docs/ARCHITECTURE.md
5. docs/PRIVACY_PRINCIPLES.md

## Non-negotiable boundaries

- This is a local-first product. Do not add telemetry, cloud sync, or external AI calls without an explicit product decision and privacy review.
- Never commit or print real agent memories, private prompts, credentials, tokens, local databases, or tool-home files.
- Connectors are read-only by default and may only access sources the user explicitly selects.
- Do not write into Codex, Claude, or other agent-owned memory stores.
- Every derived trait, event, reminder, and visual change must retain source lineage.
- Deleting a source must delete or recompute all downstream effects.
- Reminder initiative must respect quiet hours, daily budgets, and user controls.
- Never present mock data or planned behavior as live functionality.

## Language and UX

- English is the primary public and application language.
- Traditional Chinese is a first-class language; keep meaningful user-facing copy and core product documents in parity.
- Preserve the visible “memory access is off” state until a real connector and consent flow exist.

## Verification

Run:

    npm run check
    cargo fmt --manifest-path src-tauri/Cargo.toml --check

Visible UI changes also require a browser or desktop smoke check. Update PROJECT_STATUS.md when the current state, risks, or next step changes.
