---
name: memoryling-operation
description: Operate Memoryling entirely from the Agent conversation. Compile authorized context into a bounded bilingual pet update, submit it, and automatically wake the local pet; or perform a wake-only request without reading memory. Use when the user says "運作 Memoryling", "執行 Memoryling", "Run Memoryling", "叫出 Memoryling", "Show Memoryling", asks to wake or update their pet, or explicitly invokes $memoryling-operation.
---

# Operate Memoryling

Treat the slogan as permission to derive one pet update from sources the current Agent can already read. Do not treat it as permission to add connectors or widen access.

The conversation is the primary control surface. In the ordinary flow, run the local helpers yourself; do not tell the user to open Memoryling or paste a command.

## Wake only

If the user asks only to show or wake the existing pet, run `scripts/Start-Memoryling.ps1`. Do not inspect Agent memory, recent work, or repository content and do not create a package. Report only whether the local wake request succeeded.

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
6. Validate, submit, wake the installed pet, and wait for local application confirmation:

   ```powershell
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path <temporary-json>
   ```

7. Delete the temporary package if it contains any user-derived dialogue. Report only the operation ID, activity, dialogue count, and whether the pet opened and applied the operation. Never echo the package or a local executable path.

The helper fails before submission when Memoryling 0.6.0 or newer cannot be resolved. It checks an explicit development path when provided, an already-running Memoryling process, the current-user uninstall registration, and two exact current-user install candidates. It does not search `PATH` or launch an arbitrary executable.

## Dialogue rules

- Keep each language natural, specific enough to feel alive, and under 240 characters on one line.
- Include at least one `on-open` and one `on-interact` card; use `ambient` sparingly.
- Set expiry for time-sensitive lines, cooldowns for repeated lines, and no more than 20 uses.
- Do not state unverified facts, diagnoses, moral judgments, or confidential details.
- The app owns timing, quiet hours, budgets, persistence, and rendering. The Agent owns semantic compilation only.
