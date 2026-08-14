# Memoryling Agent Operation Protocol v2

Protocol v2 is a privacy-minimized, local handoff from the current Agent to an already-open Memoryling 0.7.0-or-newer App. The machine contract is `schemas/agent-operation-v2.schema.json`.

## Package

- `schemaVersion`: exactly `2`.
- `operationId`: a new stable lowercase ID for this snapshot. Never reuse one ID with another digest.
- `generatedAt`: RFC 3339 timestamp.
- `agent.family`: `codex`, `claude`, or `other`.
- `sourceDigest`: SHA-256 over sorted opaque reference hashes plus profile, appearance decision, and dialogue IDs. Never hash a secret merely to include it.
- `profile`: dominant activity, optional secondary activity, and journey state.
- `appearancePlan`: `hold`, evidence-qualified `change`, or explicit-source-removal `reset`.
- `evidence`: 1–12 opaque records. Allowed kinds are `durable-memory`, `recent-work`, `repo-ssot`, and `current-thread`.
- `dialogues`: exactly 48 bilingual cards.

## Appearance qualification

- `hold` uses `insufficient-evidence`, has no target, and may cite the context that was evaluated.
- `change` uses `explicit-milestone` with at least one evidence ID or `consistent-signals` with at least two distinct evidence IDs. It supplies a target activity and journey state.
- `reset` uses `source-removed`, has no target, and is valid only when removal or retraction is explicit in authorized context.

The App owns the local-date gate. A qualified second change on the same day becomes the one pending change and can apply on the next local day while that operation remains authoritative.

## Dialogue deck

The exact distribution is:

- 8 `opening` cards with trigger `on-open`;
- 20 `interaction` cards with trigger `on-interact`;
- 16 `ambient` cards with trigger `ambient`;
- 4 `appearance` cards with trigger `on-open`.

Every card includes a stable ID, `themeId`, `semanticGroup`, one-line English and Traditional Chinese text, priority, cooldown, max uses, and at least one package-local evidence ID. Each language is at most 160 characters.

Stable IDs are the rolling-update contract. If an ID and both localized lines are unchanged, the App preserves its use count and last-used time. Missing IDs retire; new IDs start unused. The local selector prefers least-used content and avoids repeating the current semantic group or theme when another eligible card exists.

## Runtime policy

- Click dialogue has a two-second anti-stack interval.
- Ambient dialogue is randomly scheduled 35–70 minutes apart, blocked from 22:00 through 09:00, delayed at least ten minutes after any shown line, and capped at seven per local day.
- Only the generated line reaches the pet DTO. Theme IDs, semantic groups, hashes, evidence, and full deck state stay outside the pet surface.

## Forbidden content

Never include raw memory, source text, prompts, reasoning, tool output, paths, URLs, names, emails, customer data, secrets, credentials, arbitrary attachments, or executable instructions. The dialogue should gently abstract authorized context, not quote or dump it.
