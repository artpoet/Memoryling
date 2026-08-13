---
name: memoryling-operation
description: Compile authorized Agent context into a bounded, bilingual Memoryling pet update and submit it to the local app. Use when the user says "運作 Memoryling", "執行 Memoryling", "Run Memoryling", asks to wake or update their Memoryling from recent Agent work, or explicitly invokes $memoryling-operation in the Memoryling project.
---

# Operate Memoryling

Treat the slogan as permission to derive one pet update from sources the current Agent can already read. Do not treat it as permission to add connectors or widen access.

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
6. Validate and submit it:

   ```powershell
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path <temporary-json>
   ```

7. Delete the temporary package if it contains any user-derived dialogue. Report only the operation ID, activity, dialogue count, and whether local submission succeeded. Never echo the package.

## Dialogue rules

- Keep each language natural, specific enough to feel alive, and under 240 characters on one line.
- Include at least one `on-open` and one `on-interact` card; use `ambient` sparingly.
- Set expiry for time-sensitive lines, cooldowns for repeated lines, and no more than 20 uses.
- Do not state unverified facts, diagnoses, moral judgments, or confidential details.
- The app owns timing, quiet hours, budgets, persistence, and rendering. The Agent owns semantic compilation only.
