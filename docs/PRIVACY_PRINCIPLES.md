# Privacy Principles

Memoryling's emotional value depends on access to sensitive context. Privacy is therefore a product behavior, not a legal footer.

## Commitments

### 1. Local-first by default

Raw agent memories and derived state stay on the user's device by default. No telemetry, cloud sync, or remote model request is currently implemented.

### 2. Explicit, narrow consent

Memoryling must explain which source it wants to read, which path or account is involved, and what categories of records will be imported. Consent to one source does not authorize another.

### 3. Read-only connectors

Source adapters may read approved durable-memory records but must not alter, delete, reorganize, or “repair” the source agent's files.

### 4. Import preview

Before persistence, users should see the proposed scope, record count, time range, and representative redacted samples. They can exclude records or cancel.

### 5. Source lineage

Every derived signal and world effect keeps machine-readable references to its source records and derivation version. “Why did this happen?” is a required control.

### 6. Complete forgetting

Deleting an imported source or record invalidates its dependent signals. Effects must be deleted or recomputed from the remaining sources.

### 7. Bounded reminders

Reminder candidates do not become notifications automatically. Policy must enforce quiet hours, daily budgets, urgency thresholds, snooze state, and a global off switch.

### 8. No secrets in development artifacts

Real memories, tokens, credentials, private prompts, local databases, and identifiable logs must not appear in this repository, issues, CI, screenshots, or fixtures.

## Before any network feature

Any feature that transmits memory-derived content must receive:

- a dedicated architecture decision record;
- a visible data-flow explanation;
- purpose-specific opt-in;
- a reviewable payload preview where practical;
- retention and deletion behavior;
- an offline or local-only path.

Until those conditions are met, the product must fail closed.
