# AGENTS.md

## Start here

Read, in order:

1. AI-WAKEUP.md
2. PROJECT_STATUS.md
3. README.md
4. docs/ARCHITECTURE.md
5. docs/PRIVACY_PRINCIPLES.md

## Operate Memoryling

When the user says `寵物醒來` or `Memoryling, wake up` in this repository:

1. Read `skills/memoryling-operation/SKILL.md` completely.
2. Follow that skill using only context the current Agent is already authorized to read.
3. Compile and submit one protocol-v1 operation package to the local Memoryling app.
4. Submit only when a compatible installed pet is already open, then wait for local application confirmation.
5. Report the bounded result in this conversation. Do not ask the user to run the helper manually.

Do not launch Memoryling from the Agent workflow. The user installs and opens the App through its EXE or Start menu, then follows the visible activation-phrase reminder. If the App is closed, stop before inbox write and ask the user to open it before using the phrase again.

Do not trigger the pet workflow when the user asks to read `AI-WAKEUP.md`, wake the project／Agent／self, or inspect wake-up documents. Those requests only load project context unless the user also says the exact pet activation phrase.

The slogan authorizes this bounded pet update. It does not authorize new connectors, broader private-data access, external AI calls, or writes to Agent memory.

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
