# Security Policy

Memoryling is pre-release software and must not yet be trusted with production-sensitive memory sources.

## Reporting a vulnerability

Please do not open a public issue for a suspected security or privacy vulnerability. Use GitHub's private vulnerability reporting feature under the repository's Security tab. Include reproduction steps, impact, affected versions, and any suggested mitigation.

Do not attach real agent memories, credentials, tokens, private prompts, or personal databases. Create the smallest synthetic reproduction possible.

## Current security boundary

- The v0.2.0 pet-first fixture shell does not access real agent memories.
- No telemetry or cloud upload is implemented.
- The pet WebView receives a whitelisted render DTO and is denied all six sensitive memory commands by both exact capabilities and Rust caller-label guards.
- Future connectors must be source-specific and read-only by default.
- Future derived records must retain source lineage and support deletion or recomputation.
- Secrets and local memory databases are ignored by Git and must never enter fixtures.

Only the latest released version will receive security fixes once packaged releases begin.
