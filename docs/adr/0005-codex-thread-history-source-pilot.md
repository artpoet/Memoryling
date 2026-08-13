# ADR-0005: Codex thread-history source pilot

- Status: Superseded as primary by [ADR-0007](0007-codex-agent-memory-auto-sync.md); retained as a supplementary compatibility source
- Date: 2026-08-12
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0002](0002-sqlite-v1-fixture-first-memory.md), [ADR-0004](0004-deterministic-content-derived-evolution-paths.md)
- Research: [Codex source format evaluation](../research/2026-08-12_codex-source-format-evaluation.md)

## Context

Memoryling needs one real, user-selected, read-only source before it can move beyond synthetic fixtures. Official OpenAI documentation describes local Codex memories as generated state and does not publish a stable export API, memory API, or compatibility-guaranteed memory-file schema. Directly scanning Codex home would therefore create an unsupported privacy-sensitive dependency.

OpenAI does document local App Server stdio operations for listing and reading Codex threads, and those methods do not require the opt-in `experimentalApi` capability. However, OpenAI still documents the overall App Server command／transport as experimental and unsupported for production workloads. Thread history is not durable memory, but it can support a narrow version-bound pilot when the product names it honestly and the user selects exactly one completed thread.

The decision remains Proposed through implementation because its acceptance also requires a separately authorized private-data UAT and does not establish a production-supported interface.

## Proposed decision

The first user-selected source **pilot** will be **Codex work/thread history**, not “Codex memory.” It remains experimental and does not close the Phase 1 supported-real-source gate. Its contract is:

1. **Official documented methods, experimental host.** The adapter launches a tested local Codex App Server over its default stdio JSON-RPC transport. It may call only documented `thread/list` and `thread/read`, which do not require `experimentalApi`. The overall host／transport remains experimental and unsupported for production, so the adapter requires an exact match to the single Codex CLI version proved by its compatibility test and fails closed on every other version. This is an exact pin, not a claimed compatible range. It must not parse Codex-owned memory, session, database, or rollout files directly.
2. **User-triggered discovery.** Listing begins only after the user chooses `Choose Codex work`. There is no startup scan, scheduled scan, filesystem watcher, background synchronization, or silent retry.
3. **Opaque, redacted catalog.** `thread/list` results are converted in memory to short-lived opaque selection handles. The UI may show only bounded redacted metadata such as a generic label, coarse date, completion eligibility, and an explicit redaction notice. Raw thread IDs, titles, previews, repository names, paths, prompts, responses, and tool output are not displayed, logged, or persisted by the catalog.
4. **Exactly one explicit selection.** The user selects one catalog item. A second active source or second selected thread is rejected before read or consent. Multi-select, select-all, implicit latest-thread selection, and automatic source expansion are out of scope.
5. **User-confirmed completion.** The adapter rejects a thread reported as active. Before import, the user must explicitly confirm that the selected thread represents completed work. Memoryling does not infer success, quality, personality, topic, or outcome from thread text.
6. **Minimal read and preview.** Only after selection may the adapter call `thread/read` with turns. It may select only the final `agentMessage` whose phase is `final_answer` from the last completed turn; user prompts, commentary, reasoning, tool output, and all other items are excluded. Memoryling produces a bounded redacted preview containing source kind, coarse time range, eligible item count, proposed `completion` event count, and excluded-data categories. Selected text remains backend-transient before consent, is not a preview field, and never enters the pet render boundary.
7. **Purpose-bound scope hash.** Approval creates one content-free scope hash derived from adapter identity/version, source kind, opaque external thread identity, consent-schema revision, mapping version, and approved purpose. Hash inputs are not logged or exposed. Unknown adapter, schema, mapping, or response shapes fail closed.
8. **Deterministic user-confirmed event.** The pilot emits at most one allowlisted `completion` event for the selected thread, and only after preview, consent, and the explicit completion confirmation. Thread volume, token count, elapsed time, model output, tool calls, and textual content do not change its weight or create more events.
9. **App-local content, redacted external lineage.** After the exact preview, consent, and completion confirmation, the selected final-answer content is normalized into `normalized_text` and persisted only in Memoryling's app-local SQLite database. It never crosses frontend IPC, appears in UI or logs, or enters pet DTOs. External lineage retains only provider kind, adapter/mapping versions, scope hash, consent revision, content-free hashed thread/turn references needed for recomputation, and exclusion reasons; it contains no Codex text, title, path, repository identity, model reasoning, command output, or tool payloads.
10. **Strict read-only behavior.** The adapter must not call turn start/resume, fork, inject, rename, pin, archive, delete, rollback, goal, metadata-write, shell, or process methods. It makes no model call, runtime network call, telemetry event, or write into Codex. WebSocket is excluded. Using stdio narrows exposure but does not promote the experimental App Server command into a production-supported interface.
11. **Forget only Memoryling's copy.** Forgetting removes or recomputes the imported event and all local downstream signals, effects, explanations, render state, caches, and lineage from the remaining eligible data. It never deletes, edits, archives, or claims secure erasure of the original Codex thread.
12. **One-source invariant.** Until a later accepted ADR changes the model, Memoryling allows one approved source scope total. Choosing another thread requires forgetting the current source and completing a fresh list, selection, preview, consent, and import flow.

## Consequences

### Positive

- The first real-source experiment uses documented methods rather than an inferred private file format while keeping its host maturity visible.
- Product copy remains truthful: a completed work thread is evidence explicitly confirmed by the user, not a claimed reading of Codex memory or personality.
- User-triggered listing, one-source scope, redacted lineage, and local-only forgetting keep the privacy surface narrow and testable.
- The adapter can reuse the existing preview, consent, lineage, deterministic effect, and transactional forgetting architecture.

### Costs and limits

- App Server is experimental and unsupported for production; availability, protocol compatibility, pinned-version policy, response validation, process lifecycle, cancellation, and failure recovery need platform tests.
- Thread history may contain more sensitive content than a derived memory summary, so minimization and no-output test fixtures are mandatory.
- The pilot represents one user-confirmed completed-work event; it does not yet support content-derived evolution, automatic sync, multiple threads, or multiple sources.
- Real private-data proof cannot be automated or recorded in repository fixtures and requires separate user authorization.

## Rejected alternatives

- **Parse `~/.codex/memories/`:** rejected because official documentation calls it generated state and publishes no stable third-party schema or export API.
- **Call the source “Codex memories”:** rejected because thread history and durable memories are different official concepts.
- **Read latest/all threads automatically:** rejected because it removes meaningful selection and widens exposure.
- **Claim stdio App Server is production-stable:** rejected because the methods are documented without an `experimentalApi` requirement, but OpenAI still marks the overall command／transport experimental and unsupported for production workloads.
- **Use WebSocket App Server:** rejected because it widens the experimental transport surface without helping the local pilot.
- **Use thread text to classify task outcome or personality:** rejected because it is unnecessary, privacy-expanding, and incompatible with deterministic user-confirmed completion.
- **Delete the original thread when forgetting:** rejected because the connector is read-only and forgetting governs Memoryling's local copy only.

## Privacy impact

The adapter opens a narrow local boundary to one explicitly selected Codex thread. Catalog data is transient and redacted. Before exact consent, selected final-answer text is backend-transient only. After exact consent and completion confirmation, that selected content is normalized into `normalized_text` and retained only in app-local SQLite; it is never exposed through frontend IPC, UI, logs, pet DTOs, or external lineage. Persisted external source metadata and lineage are content-free or hashed. User prompts, commentary, reasoning, paths, repository identity, and tool output are neither rendered nor retained.

No model, cloud, telemetry, network, background-monitoring, or direct tool-home access is authorized by this ADR. This Proposed pilot does not close Phase 1 or authorize production distribution. Any future wider data category, automatic synchronization, additional source, content-derived mapping, memory-format integration, or production-supported claim requires a new preview, privacy review, and ADR revision or successor.

## Acceptance and private UAT gate

This ADR may become Accepted only after all of the following are proved:

- synthetic App Server contract fixtures cover pagination, redaction, unknown fields, malformed responses, active-thread rejection, cancellation, and process failure;
- the adapter proves and enforces one exact Codex CLI version pin and fails closed on missing or any non-matching App Server build; it does not claim a supported version range;
- an allowlist test proves the adapter issues only stdio `thread/list` and `thread/read` and makes no model, network, telemetry, mutation, or filesystem-scan request;
- catalog handles expire; no source content persists before exact consent; afterward only the selected final answer persists as app-local SQLite `normalized_text`; and no raw identifiers, titles, paths, previews, or content reach logs, snapshots, errors, frontend persistence, frontend IPC, external lineage, or pet DTOs;
- one-source and one-thread invariants fail closed before any extra read;
- preview, consent, scope-hash, user-confirmed completion, lineage, restart persistence, explanation, correction boundary, and transactional forgetting pass automated tests;
- forgetting removes all local downstream effects while the original Codex thread remains untouched;
- packaged Windows behavior passes with synthetic App Server fixtures;
- the user separately authorizes access to one deliberately selected completed private thread for UAT;
- private UAT records only pass/fail and content-free counts: no thread content, title, path, identifier, screenshot, database content, or tool output enters git, logs, chat evidence, or test fixtures;
- private UAT proves explicit listing, one selection, redacted preview, consent, one deterministic effect, restart, explanation, forget, and no mutation of Codex; this validates only the pinned experimental pilot and does not close Phase 1;
- user-facing English and Traditional Chinese copy consistently says Codex work/thread history and never claims durable-memory access.
