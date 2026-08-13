# Codex source format evaluation

> Update (2026-08-13): current official Codex documentation now describes local memory files under the Codex home. The primary-source conclusion here is superseded by [the updated research](2026-08-13_codex-local-memory-source-update.md) and [ADR-0007](../adr/0007-codex-agent-memory-auto-sync.md). This file remains historical evidence for the supplementary thread-history pilot.

AS_OF: 2026-08-12 (Asia/Taipei)

## Decision summary

OpenAI does **not** currently publish a stable export API, memory API, or compatibility-guaranteed file schema for local Codex durable memories. Memoryling must not treat files below `~/.codex/memories/` as a supported product integration contract.

The best bounded alternative for the first real-source experiment is **Codex work/thread history** through the documented App Server stdio methods that do not require the `experimentalApi` capability:

- `thread/list` to build a user-triggered, content-minimized selection catalog;
- `thread/read` with `includeTurns: true` only after the user explicitly selects one thread;
- no `turn/start`, model invocation, thread mutation, direct tool-home scan, WebSocket transport, telemetry, or runtime cloud service.

This source must be described as Codex work or thread history, never as Codex memory. The overall App Server command／transport is still documented as experimental and unsupported for production workloads, so this remains a version-bound experimental pilot rather than a supported production connector.

## Scope and method

This evaluation used only current official OpenAI documentation. It did not inspect Codex home, local memories, rollout logs, user chats, databases, credentials, prompts, or any other private data. No private-data sample informed the decision.

Official sources:

- [Memories](https://learn.chatgpt.com/docs/customization/memories)
- [Codex SDK](https://learn.chatgpt.com/docs/codex-sdk)
- [Codex App Server](https://learn.chatgpt.com/docs/app-server)
- [Configuration Reference](https://learn.chatgpt.com/docs/config-file/config-reference)
- [Feature Maturity](https://learn.chatgpt.com/docs/feature-maturity)

## Findings

### 1. Durable memories exist locally, but their storage is generated state

The Memories documentation distinguishes ChatGPT web memory from the separate local memory store used by local Codex clients. It says the main files live below `~/.codex/memories/` and may include summaries, durable entries, recent inputs, and supporting evidence.

The same page says to treat those files as **generated state**. They may be inspected for troubleshooting or before sharing a Codex home directory, but manual file editing is not the primary control surface. The documentation does not provide:

- a versioned memory export schema;
- a third-party memory enumeration or read API;
- forward/backward compatibility rules for those files;
- a deprecation policy for individual memory file fields;
- a consent-scoped connector contract.

The Configuration Reference documents memory-generation and memory-use settings. Those settings control Codex behavior; they do not define an external data format or expose memories to another application.

**Implementation result:** direct parsing of the local memory directory is technically discoverable state, not a supported Memoryling connector.

### 2. Codex SDK is a thread-control SDK, not a memory-export SDK

The Codex SDK documentation supports starting, continuing, and resuming local Codex threads. It does not document durable-memory listing, reading, exporting, or change notifications.

**Implementation result:** the SDK is not evidence that Memoryling may access Codex durable memories. It may be appropriate for agent execution, which this local read-only source does not need.

### 3. App Server exposes documented thread history operations

The App Server documentation describes a local JSON-RPC interface for rich clients. Its default stdio transport uses newline-delimited JSON. The documented methods that do not require the `experimentalApi` capability include:

- `thread/list`, which pages stored thread logs;
- `thread/read`, which reads one stored thread without resuming it and optionally includes turns.

The protocol separately identifies methods or fields that require the `experimentalApi` capability. Memoryling does not need those opt-in operations for this pilot. This distinction applies only to the methods: the same official page says the App Server command／transport is experimental and unsupported for production workloads. Local Codex CLI help must be checked and pinned during implementation because the pilot remains version-bound.

The page explicitly describes WebSocket transport as experimental and unsupported for production workloads. Memoryling therefore must use local stdio only and must not open a WebSocket listener.

**Implementation result:** a narrow, experimental thread-history adapter can use documented local stdio methods without parsing Codex-owned files. It is a version-bound evaluation source, not a durable-memory source or production-supported connector.

### 4. Official maturity language supports this distinction

OpenAI defines a stable feature as fully supported, documented, ready for broad use, and consistent over time. No documented durable-memory schema or memory read/export endpoint meets that bar. App Server documents the required thread methods and identifies opt-in experimental method fields, but explicitly keeps the overall command／transport experimental and unsupported for production. The pilot therefore cannot close the real-source Phase 1 gate or support a public production claim.

This does not make every internal thread field a permanent product contract. The adapter must consume only the documented response fields it needs, pin an adapter version, fail closed on unknown shapes, and retain synthetic contract fixtures.

## Allowed experimental pilot boundary

Memoryling may implement one local read-only, version-bound experimental Codex work/thread-history pilot with all of these constraints:

1. Listing starts only from an explicit user action.
2. The catalog is content-minimized and uses opaque selection handles rather than displaying raw thread identifiers or paths.
3. The user explicitly selects exactly one thread.
4. The thread is not actively running, and the user confirms that the selected work is complete.
5. Default preview reveals only bounded, redacted metadata needed for consent.
6. The adapter calls only local stdio `thread/list` and `thread/read`; it does not start or resume a turn, and must fail closed unless the Codex CLI version exactly matches the single version proved by the implementation's compatibility test. This is an exact pin, not a claimed compatible range.
7. No model call, external network call, telemetry, background monitoring, or direct Codex-home scan is introduced.
8. The selected final-answer content is the only Codex text that may persist: after exact preview, consent, and completion confirmation, it is normalized into `normalized_text` and stored only in Memoryling's app-local SQLite database. It is never returned through frontend IPC, displayed, logged, or emitted in pet DTOs. External lineage stores only content-free/redacted references and a content-free scope hash.
9. Forgetting deletes or recomputes Memoryling's local projections only; it never archives, deletes, edits, or otherwise mutates the Codex thread.
10. A separately authorized private-data UAT is required before claiming the pilot works with one deliberately selected real thread; this does not make it production-supported or complete Phase 1.

The detailed product decision is recorded in [ADR-0005](../adr/0005-codex-thread-history-source-pilot.md).

## Unsupported claims and implementations

Do not claim or implement any of the following on this evidence:

- “Read your Codex memories” or equivalent UI copy;
- automatic discovery or scanning of `~/.codex/memories/`;
- reliance on current local memory filenames or field layouts;
- WebSocket App Server integration for the production pilot;
- import of every thread, automatic background synchronization, or monitoring;
- deletion or correction of data inside Codex;
- inference that a thread proves task success without explicit user confirmation;
- use of raw prompts, assistant responses, paths, titles, or tool output as pet-render data.

## Re-evaluation trigger

Re-evaluate this decision if OpenAI publishes a stable, versioned memory export/API contract or promotes and supports the relevant App Server command／transport for production. Until then, visible durable-memory access remains off; Codex work/thread history is an opt-in experimental pilot only, and the Phase 1 real-source gate remains open.
