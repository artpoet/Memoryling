# Privacy Principles

Memoryling's emotional value depends on access to sensitive context. Privacy is therefore a product behavior, not a legal footer.

## Commitments

### 1. Local-first by default

Raw agent memories and derived state stay on the user's device by default. No telemetry, cloud sync, or remote model request is currently implemented.

### 2. Explicit, narrow consent

Memoryling must explain which source it wants to read, which path or account is involved, and what categories of records will be imported. Consent to one source does not authorize another.

For the proposed growth system, one consent scope covers one exact source, its source-specific adapter version, allowed data categories, local derivation purposes, and consent／mapping versions. New records inside that exact scope may then update abstract local growth without per-record prompts. Another source requires another scope; a new category, purpose, or materially changed mapping requires a new consent revision. This never authorizes scanning another location or silently expanding use. The user must be able to inspect aggregate reasons, correct classification, disable derivation, revoke a scope, and forget its local downstream effects.

### 3. Read-only connectors

Source adapters may read approved durable-memory records but must not alter, delete, reorganize, or “repair” the source agent's files.

### 4. Import preview

Before the initial real-source scope is activated, users should see the proposed source, adapter version, data categories, purposes, initial record count／time range, and representative redacted samples. They can exclude initial records or cancel. Once that exact scope is active, later in-scope records may persist and derive automatically without another per-record preview; users instead retain aggregate inspection, correction, scope disable／revoke, and forgetting controls. Any scope expansion returns to preview and consent.

The current pilot shows fictional fixture text. Redaction behavior for a real source must be designed and verified before private-data UAT.

### 5. Source lineage

Every derived signal and world effect keeps machine-readable references to its source records and derivation version. “Why did this happen?” is a required control.

### 6. Complete forgetting

Deleting an imported source or record invalidates its dependent signals. Effects must be deleted or recomputed from the remaining sources.

“Complete” means complete within Memoryling's imported records and supported downstream graph. It does not authorize changing the source agent's files or promise physically irrecoverable erasure from storage media, backups, or operating-system snapshots.

### 7. Bounded reminders

Reminder candidates do not become notifications automatically. Policy must enforce quiet hours, daily budgets, urgency thresholds, snooze state, and a global off switch.

### 8. No secrets in development artifacts

Real memories, tokens, credentials, private prompts, local databases, and identifiable logs must not appear in this repository, issues, CI, screenshots, or fixtures.

## Current fixture-pilot evidence

The implemented first-memory pipeline has a deliberately narrower boundary than a real connector:

- Its only source is one fictional Codex-shaped JSON resource bundled with the app. It cannot scan arbitrary paths or read a user's Codex tool-home.
- Source preview state is held in Rust process memory and bound to a pending token. Previewing or canceling does not persist the fixture's source content; the desktop may still initialize an empty local schema.
- Explicit approval stores the selected normalized text, timestamps, hashes, adapter metadata, and machine-readable lineage in `memoryling.sqlite3` under Tauri's app-local data directory.
- Derivation is deterministic and local: the supported completion record creates one completion signal and one completion-star effect. No network client, telemetry, cloud sync, or remote model call is part of this path.
- Forgetting runs in a local transaction: it removes the imported source and cascading normalized event, clears derived state, and recomputes effects from supported records that remain. The bundled fixture itself is read-only and is not changed or deleted.
- SQLite foreign keys and `secure_delete` are enabled, but this is an application-level deletion control, not a cryptographic secure-erasure guarantee.

No user-owned agent memory has been imported or used for real-data UAT. The visible real-memory access state must remain off until a real connector and consent flow are verified.

The scoped automatic-derivation model described above is future design, not behavior of the current fixture pilot. It does not weaken the current import preview or explicit fixture approval flow, and it must pass a dedicated privacy review before real-source implementation.

## Proposed pet-surface display boundary

The user-confirmed floating-pet shell is not implemented, but its privacy contract is already fixed:

- The resident pet surface receives a dedicated render-safe state, not the full memory or lineage DTO. It must not receive normalized memory text, source paths or locators, private explanation content, or arbitrary record payloads.
- A safe DTO is not sufficient by itself. Per-window app-command permissions and caller-label checks must deny list／preview／cancel／full-state／approve／forget commands from the pet surface, with negative invoke tests.
- Pet reactions, native menu items, tray labels, window titles, onboarding, and operating-system surfaces must remain neutral; they cannot reveal names, projects, traits, or source summaries.
- Cross-window events carry only opaque revisions or non-sensitive counts. Each surface refetches a typed state limited to its purpose.
- Closing the detail window, hiding the pet, quitting the app, and forgetting a source are distinct operations and must never be described as equivalent deletion.
- The visible real-memory-off status remains on the pet surface until a real connector is verified. Browser mode must not imitate native persistence or resident-window behavior.
- A screenshot／screen-sharing privacy mode and neutral growth-summary review are required before public testing with real memory-derived state.

## Before any network feature

Any feature that transmits memory-derived content must receive:

- a dedicated architecture decision record;
- a visible data-flow explanation;
- purpose-specific opt-in;
- a reviewable payload preview where practical;
- retention and deletion behavior;
- an offline or local-only path.

Until those conditions are met, the product must fail closed.
