# ADR-0007: Codex Agent memory is the primary read-only source

- Status: Superseded as primary by ADR-0008
- Date: 2026-08-13
- Supersedes: ADR-0005 only as the primary-source priority and its prohibition on reading documented local memory state
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0004](0004-deterministic-content-derived-evolution-paths.md), [ADR-0005](0005-codex-thread-history-source-pilot.md)
- Research: [2026-08-13 Codex local-memory source update](../research/2026-08-13_codex-local-memory-source-update.md)

> Historical note (2026-08-13): the implementation remains in source as a compatibility experiment, but it is no longer started automatically or presented as the product's core. ADR-0008 moves semantic reading back to the user-invoked Agent and keeps the app outside Agent-owned memory stores.

## Context

Memoryling's product promise is an evolving companion shaped by Agent memory. Requiring the user to choose one work record at a time makes thread history, rather than Agent memory, the product's main source.

Official Codex documentation now says local Codex clients maintain separate local memory under the Codex home, normally `~/.codex/memories/`, and update it in the background. The documentation describes generated state and its current main artifacts, but does not promise a stable third-party file schema or write interface. The connector therefore remains versioned, allowlisted, local-only, and fail-closed.

## Decision

1. **One source-level consent.** The user explicitly chooses the current Codex Agent-memory source, reviews a content-redacted scope, and approves read-only local storage, derivation, and future automatic sync. Individual memory files or records are not separately selected.
2. **Exact local source.** Resolve `CODEX_HOME` when configured; otherwise use `%USERPROFILE%\.codex`. Consent binds a hash of the exact resolved `memories` root. A changed root requires fresh consent.
3. **Two-file allowlist.** Adapter v1 reads only top-level `memory_summary.md` and `MEMORY.md`. It does not enumerate or read rollout summaries, sessions, state databases, prompts, logs, credentials, or arbitrary paths.
4. **Filesystem safety.** The root and files must not be symlinks. Each file is capped at 2 MiB and the combined source at 4 MiB. Files must be regular UTF-8 files with non-empty content. Unknown or unsafe states fail closed.
5. **No private preview.** Rust returns only logical record IDs, timestamps, character counts, and hashes. Raw Agent-memory text never enters the WebView, logs, pet DTO, or test fixtures.
6. **Local persistence and lineage.** After consent, normalized documents are stored only in Memoryling's local SQLite database with source, adapter, content hash, consent-scope hash, and event lineage.
7. **Automatic sync.** Memoryling checks the approved source at startup, every 15 minutes while running, and on explicit `Sync now`. Sync is read-only and makes no network request.
8. **Transactional recomputation.** A successful change replaces that source's local events and recomputes downstream signals and effects in one transaction. Agent-memory documents aggregate into one `agent-memory-continuity` signal and one visible `memory-halo` effect.
9. **Failure semantics.** If the approved source disappears, Memoryling retains consent but removes its local events and downstream effects until the same source returns. Unsafe, unreadable, oversized, or scope-changed states keep the last valid local state and show `needs-attention`.
10. **Forgetting.** Disconnect removes consent, Memoryling's local copy, lineage, sync state, and downstream effects. It never edits or deletes Codex memory.
11. **No Daily Scout reuse.** Agent-memory text is not eligible input for Daily Memory Scout. The existing Scout compiler remains restricted to its separately approved work-record adapter and consent.
12. **Supplementary work records.** ADR-0005 remains available as an explicitly selected, version-bound compatibility source. It is no longer the recommended or primary memory path.

## Consequences

- The product now matches its Agent-memory promise without repeated per-record prompts.
- Memory access remains visibly off until the source-level preview and consent succeed.
- Generated Codex state can drift, so adapter changes require a new version, safety review, and consent when scope changes.
- Only synthetic fixtures may be used in automated verification. Private-memory UAT requires a separate exact authorization.

## Rejected alternatives

- **Keep per-record thread selection as the main path:** does not represent accumulated Agent memory.
- **Scan the whole Codex home:** violates minimization and creates unstable private-data coupling.
- **Watch all filesystem changes continuously:** unnecessary complexity and a wider behavioral footprint than bounded polling.
- **Send Agent memories to an external model for interpretation:** violates the current local-first product boundary.

## Rollback

Disable the Agent-memory command and scheduler, forget the Memoryling-local source, and retain the synthetic fixture／ADR-0005 work-record paths. Codex-owned files remain untouched.
