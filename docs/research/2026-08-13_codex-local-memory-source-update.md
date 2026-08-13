# Codex local-memory source update

- `AS_OF`: 2026-08-13
- Scope: official OpenAI documentation only
- Product decision: [ADR-0007](../adr/0007-codex-agent-memory-auto-sync.md)

## Factual anchors

OpenAI's current [Codex Memories documentation](https://learn.chatgpt.com/docs/customization/memories) states that local Codex clients use a separate local memory system. When enabled, eligible chats can produce local memory files updated in the background. It identifies the Codex home (normally `~/.codex`) and `~/.codex/memories/` as the current storage area, including summaries, durable entries, recent inputs, and supporting evidence. It also describes these files as generated state that may be inspected but should not be hand-edited.

OpenAI's current [Codex import documentation](https://learn.chatgpt.com/docs/import) says Codex can import supported Agent setup and recent-work context, and that Codex Desktop can update imports automatically. This supports treating Agent memory as accumulated context rather than requiring a user to nominate each work record.

The current [Codex App Server documentation](https://learn.chatgpt.com/docs/app-server) documents thread and history integration but does not document a memory-export RPC. A bounded local-file connector is therefore the narrow current route; it is not evidence of a stable public memory schema.

## Product interpretation

- Use one explicit source-level consent, followed by bounded automatic local sync.
- Treat the current filenames as adapter-v1 allowlist details, not a compatibility promise from OpenAI.
- Never enumerate the broader memory evidence or rollout directories.
- Keep raw content backend-only and exclude this source from Daily Scout or any external model call.
- Require a new adapter version and privacy review if official storage guidance or observed structure changes.
