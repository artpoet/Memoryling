---
name: memoryling-operation
description: Update an already-open Memoryling from the Agent conversation. Compile authorized context into a bounded bilingual pet update, submit it to the local app, and await application without launching an executable. Use when the user says "醒來吧我的寵物" or "Memoryling, wake up", asks to update their open pet, or explicitly invokes $memoryling-operation.
---

# Operate Memoryling

Treat the slogan as permission to derive one pet update from sources the current Agent can already read. Do not treat it as permission to add connectors or widen access.

Do not run this workflow merely because the user asks to read a wake-up file, wake the project／Agent／self, or inspect project context. Require the exact pet phrase or an explicit request to update the already-open Memoryling.

The installed App is the visible entry surface; the Agent conversation is the update surface. The user installs and opens Memoryling through its EXE or Start menu, then follows the App's activation-phrase reminder. Never launch an executable on the user's behalf.

## App readiness

Before reading memory or recent work, run the readiness-only helper command below. It requires a compatible Memoryling process to already be running and does not read or write package content. If it reports `MEMORYLING_APP_NOT_READY`, do not read memory, write a package, inspect install locations, or start the App. Give one locale-appropriate reminder and stop:

- Traditional Chinese: `Memoryling 還沒準備好。若尚未安裝，請先安裝；接著打開寵物，再重新輸入「醒來吧我的寵物」。`
- English: `Memoryling is not ready. Install it if needed, open the pet, then enter “Memoryling, wake up” again.`

Do not ask the user to run the helper manually. Do not continue the operation in the same turn after this reminder.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -CheckAppReadyOnly
```

## Workflow

1. Confirm App readiness with the readiness-only helper. If it fails, give the fixed reminder above and stop. Only after it succeeds, read the current thread, the repository wake-up/SSOT chain, recent work available to this Agent, and this Agent's own durable memory when available and relevant.
2. Do not open unrelated private files, another Agent's storage, credentials, prompts, databases, mail, cloud apps, or external services. Never write to Agent memory.
3. Reduce the authorized context to:
   - one dominant activity and optional secondary activity;
   - one journey state and one evidence-qualified appearance decision;
   - 1-12 opaque evidence references;
   - exactly 48 short dialogue cards in English and Traditional Chinese: 8 opening, 20 interaction, 16 ambient, and 4 appearance cards.
4. Never put raw memory, private prompts, file contents, names, emails, secrets, or source paths in the package. Hash stable source pointers locally; make dialogue a gentle abstraction, not a quotation or work-log dump.
5. Read [protocol-v2.md](references/protocol-v2.md), then create a temporary JSON package matching `schemas/agent-operation-v2.schema.json`. Use `examples/agent-operation-v2.synthetic.json` only as a structural example.
6. Validate and submit to the already-open pet, then wait for local application confirmation:

   ```powershell
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path <temporary-json>
   ```

7. Delete the temporary package if it contains any user-derived dialogue. Report only the operation ID, activity, dialogue count, and whether the open pet applied the operation. Never echo the package or a local executable path.

The helper fails before memory read or submission unless a running `Memoryling.exe` has product identity `Memoryling` and version 0.7.0 or newer. It deliberately does not scan install locations, so not installed, installed-but-closed, stale, and mismatched states share the same safe install／open reminder. It never searches `PATH`, starts a process, or treats an installed-but-closed App as ready.

## Appearance rules

- Use `hold` unless the authorized context contains either one explicit milestone or at least two independent, directionally consistent signals.
- A `change` must cite the qualifying evidence and select one bounded activity accent／journey mark from the local catalog. The App applies no more than one persistent appearance change per local day; a second qualified change is queued for the next eligible day.
- Use `reset` only when current authorized context explicitly establishes that the prior visual basis was removed or retracted. Do not infer deletion from silence.
- Speaking expressions and motion are temporary UI reactions, not persistent appearance changes.

## Dialogue rules

- Keep each language natural, semi-specific, and under 160 characters on one line. Let the user recognize the theme without exposing project names, file names, people, or source wording.
- Produce exactly 8 `opening`, 20 `interaction`, 16 `ambient`, and 4 `appearance` cards. Category and trigger must match protocol v2.
- Give every card a stable `themeId`, `semanticGroup`, and deterministic ID. Reuse an unchanged ID and unchanged bilingual text when the same theme and meaning remain relevant; the App preserves its use history. Omit stale meanings and add new stable IDs so the deck rolls forward instead of accumulating.
- Vary intent within each theme: observation, encouragement, playful metaphor, gentle check-in, and progress reflection. Do not invent new facts merely to reach 48 cards.
- Set expiry for time-sensitive lines, cooldowns for repeated lines, and no more than 20 uses.
- Do not state unverified facts, diagnoses, moral judgments, or confidential details.
- The app owns timing, quiet hours, budgets, persistence, and rendering. The Agent owns semantic compilation only.
