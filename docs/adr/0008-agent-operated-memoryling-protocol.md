# ADR-0008: Agent compiles authorized context; the local app owns persistent life

- Status: Accepted
- Date: 2026-08-13
- Supersedes: ADR-0007 as the primary product path; ADR-0006 as a core product feature
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0003](0003-pet-first-two-window-desktop-shell.md), [ADR-0004](0004-deterministic-content-derived-evolution-paths.md)

## Context

Memoryling is a pet for people who already work through Agents such as Codex or Claude. The intended interaction is a phrase inside the project—not another connector setup screen and not an AI API embedded in the app.

The Agent already has the semantic ability and authorization model needed to understand its memory, recent work, current thread, and repository SSOT. The desktop app is better at persistence, deterministic rules, rendering, quiet hours, budgets, and user controls. Combining those responsibilities inside the app would duplicate Agent capabilities and widen the privacy boundary.

## Decision

1. The primary trigger is a user phrase such as `運作 Memoryling` or `Run Memoryling` in an Agent project.
2. The project Agent skill reads only context the current Agent is already authorized to use. The slogan does not grant new connector, account, file, or external-service access.
3. The Agent compiles a protocol-v1 package with one activity profile, one journey state, 1–12 opaque evidence references, and 3–12 bilingual dialogue cards.
4. Raw memories, prompts, reasoning, paths, names, secrets, and source text are forbidden in the package. Evidence is represented only by lowercase SHA-256 reference hashes.
5. A local submit helper validates the package and atomically writes the exact inbox file under Memoryling's app-local data. The app polls only that file; it never scans Agent storage.
6. Rust validates the package again, rejects unknown or unbounded data, and stores only the newest authoritative operation in SQLite schema v5. A new operation transactionally replaces the prior package and its derived dialogue lineage.
7. The app owns dialogue triggers, expiry, cooldown, max uses, quiet hours (22:00–09:00), an ambient budget of two per local day, persistence, and render-safe DTOs. It makes no AI request for this core loop.
8. The user can clear the current pet update locally. Re-running the slogan recreates it from current authorized context.
9. Direct Codex-memory and thread connectors plus Daily Scout remain compatibility experiments in source only. They are not started automatically, shown in the primary UX, or prerequisites for the ordinary pet.

## Consequences

- One short phrase produces a meaningful pet update without API-key setup.
- Agent semantics stay in the Agent; the app remains deterministic and provider-agnostic.
- The app cannot observe an Agent-memory deletion by itself. The next successful operation is an authoritative snapshot and removes all prior derived operation data; the local clear control handles immediate deletion.
- Each Agent environment needs a discoverable project skill or equivalent instructions.
- Automated verification must use synthetic packages. Private Agent content is not required for protocol UAT.

## Rejected alternatives

- **App scans Agent memory directly:** creates brittle tool-home coupling and puts the app inside a private storage boundary.
- **App calls an AI API to interpret memory:** adds cost, network exposure, credentials, and duplicate semantic machinery.
- **Store a long operation history:** retains generated material after it stops shaping the current pet and complicates forgetting.
- **Ship only static dialogue:** cannot express the user's recent Agent work or continuity.

## Rollback

Stop the inbox worker, remove the Agent-operation commands from capabilities, and clear the local operation tables. Agent-owned context remains unchanged.
