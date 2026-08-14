---
name: memoryling-operation
description: Update an already-open Memoryling from the Agent conversation. Compile authorized context into a bounded bilingual pet update, submit it to the local app, and await application without launching an executable. Use when the user says "寵物醒來" or "Memoryling, wake up", asks to update their open pet, or explicitly invokes $memoryling-operation.
---

# Operate Memoryling

Treat the slogan as permission to derive one pet update from sources the current Agent can already read. Do not treat it as permission to add connectors or widen access.

Do not run this workflow merely because the user asks to read a wake-up file, wake the project／Agent／self, or inspect project context. Require the exact pet phrase or an explicit request to update the already-open Memoryling.

The installed App is the visible entry surface; the Agent conversation is the update surface. The user installs and opens Memoryling through its EXE or Start menu, then follows the App's activation-phrase reminder. Never launch an executable on the user's behalf.

## App readiness

The submit helper requires a compatible Memoryling process to already be running. If it reports that Memoryling is not open, do not write a package or start the App. Tell the user to open the installed Memoryling App and use the activation phrase again.

## Workflow

1. Read the current thread, the repository wake-up/SSOT chain, recent work available to this Agent, and this Agent's own durable memory when available and relevant.
2. Do not open unrelated private files, another Agent's storage, credentials, prompts, databases, mail, cloud apps, or external services. Never write to Agent memory.
3. Reduce the authorized context to:
   - one dominant activity and optional secondary activity;
   - one journey state;
   - 1-12 opaque evidence references;
   - 3-12 short dialogue cards in English and Traditional Chinese.
4. Never put raw memory, private prompts, file contents, names, emails, secrets, or source paths in the package. Hash stable source pointers locally; make dialogue a gentle abstraction, not a quotation or work-log dump.
5. Read [protocol-v1.md](references/protocol-v1.md), then create a temporary JSON package matching `schemas/agent-operation-v1.schema.json`. Use `examples/agent-operation-v1.synthetic.json` only as a structural example.
6. Validate and submit to the already-open pet, then wait for local application confirmation:

   ```powershell
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path <temporary-json>
   ```

7. Delete the temporary package if it contains any user-derived dialogue. Report only the operation ID, activity, dialogue count, and whether the open pet applied the operation. Never echo the package or a local executable path.

The helper fails before submission unless a running `Memoryling.exe` has product identity `Memoryling` and version 0.6.0 or newer. It never searches `PATH`, starts a process, or treats an installed-but-closed App as ready.

## Dialogue rules

- Keep each language natural, specific enough to feel alive, and under 240 characters on one line.
- Include at least one `on-open` and one `on-interact` card; use `ambient` sparingly.
- Set expiry for time-sensitive lines, cooldowns for repeated lines, and no more than 20 uses.
- Do not state unverified facts, diagnoses, moral judgments, or confidential details.
- The app owns timing, quiet hours, budgets, persistence, and rendering. The Agent owns semantic compilation only.
