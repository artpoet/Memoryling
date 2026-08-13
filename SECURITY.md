# Security Policy

Memoryling is pre-release software. Source v0.6.0 must not yet be trusted with production-sensitive Agent context, and it has no accepted v0.6.0 installer or private-memory UAT.

## Reporting a vulnerability

Do not open a public issue for a suspected security or privacy vulnerability. Use GitHub private vulnerability reporting and include a synthetic reproduction, impact, and affected version.

Never attach real Agent memories, prompts, credentials, tokens, operation packages, local databases, or identifiable screenshots.

## Current v0.6.0 core boundary

- The user invokes one bounded update through `運作 Memoryling` or `Run Memoryling` in an Agent project.
- The current Agent may use only context its environment already authorizes. The slogan grants no new source, account, connector, external-service, or write permission.
- The app does not discover or scan Agent tool homes. It polls one exact app-local inbox file only.
- The protocol package forbids raw memory, prompts, reasoning, paths, names, secrets, credentials, and tool output.
- The submit helper and Rust independently validate size, file type, symlink state, schema, unknown fields, IDs, timestamps, enums, counts, and dialogue bounds.
- SQLite retains only the newest authoritative operation. New apply transactionally replaces the old graph; the detail clear control removes it immediately.
- The pet WebView receives a whitelisted render DTO without source hashes, operation digest, paths, or full state.
- Exact Tauri capabilities and independent Rust caller-label guards protect sensitive main commands from the pet surface.
- The core Agent-operated loop has no telemetry, cloud sync, API key, model request, or source write.
- Automated and browser verification uses synthetic data only.

The app cannot observe an Agent-memory deletion because it deliberately does not scan Agent storage. The next successful operation must be compiled from current context and replaces prior derived state. Immediate local removal uses Clear. Neither path edits Agent-owned memory.

## Legacy compatibility code

Older fixture import, exact Codex work-record, direct Codex-memory, and BYOK Daily Scout implementations remain in source for compatibility research. Direct memory and Daily Scout schedulers are disabled, and their UI is not part of the v0.6.0 core.

Daily Scout compatibility code can make an OpenAI API request only after its old, separate manual key and consent flow. Do not present that as the ordinary pet, reactivate it automatically, or reuse Agent-operation data for it. Any continued network feature needs a fresh product and privacy review before public distribution.

## Release boundary

The unsigned v0.2.0 current-user installer is the last installed-UAT artifact. It is a historical local-test baseline and does not prove v0.6.0. Code signing, packaged upgrade and migration, WebView2-missing behavior, mixed-DPI／monitor recovery, assistive technology, sign-out／shutdown, and screenshot privacy remain open.

Only a packaged release explicitly listed as supported will receive a release security window. Until then, report issues against the source version and commit.
