# Memoryling Agent operation protocol v1

Submit exactly one JSON object. The app validates it again before persistence.

- `operationId`: unique lowercase opaque ID, max 64 characters.
- `generatedAt`, `observedAt`, `notBefore`, `expiresAt`: RFC 3339.
- `agent.family`: `codex`, `claude`, or `other`.
- `sourceDigest`: SHA-256 over the sorted opaque reference hashes plus the profile and dialogue IDs. Never hash secrets merely to smuggle them into the package.
- `profile.dominantActivity` and optional `secondaryActivity`: `building`, `research`, `design`, `planning`, `debugging`, `writing`, `coordination`, or `shipping`.
- `profile.journeyState`: `steady`, `exploring`, `milestone`, or `recovering`.
- `evidence[].kind`: `durable-memory`, `recent-work`, `repo-ssot`, or `current-thread`.
- `evidence[].referenceHash`: lowercase SHA-256 of a stable, non-secret source pointer. The raw pointer never enters the package.
- `dialogues[].trigger`: `on-open`, `on-interact`, or `ambient`.
- `priority`: 0-3. `cooldownMinutes`: 0-10080. `maxUses`: 1-20.
- Every dialogue must cite at least one evidence ID from this package.

The package is a lossy pet-state artifact, not a memory export. See the root JSON Schema for exact fields and bounds.
