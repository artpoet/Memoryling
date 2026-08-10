# ADR-0001: Local-first, lineage-aware derived memory

- Status: Accepted
- Date: 2026-08-10

## Context

Memoryling needs sensitive agent memory to create meaningful continuity. Sending all raw memory to a hosted service would weaken user trust, make deletion difficult to verify, and turn the central product advantage into a privacy risk.

The creature also needs derived state—traits, story events, visual marks, dialogue facts, and reminder candidates. If these effects lose their source relationship, the user cannot understand or fully delete them.

## Decision

Memoryling will use a local-first graph of source records, derived signals, and world effects.

- Connectors are source-specific and read-only by default.
- Import requires a preview and explicit approval.
- Raw and derived records remain local unless a later ADR authorizes a narrow network feature.
- Every derived record stores its source lineage and derivation version.
- Deleting a source invalidates dependent signals and deletes or recomputes world effects.
- Repository fixtures use synthetic data only.

## Consequences

### Positive

- Users can inspect why a change occurred.
- Forgetting can be implemented as a verifiable chain.
- Core evolution can work offline.
- Connectors have a narrow, testable security boundary.

### Costs

- The local schema and migration system become critical infrastructure.
- Derivation must be deterministic enough to recompute or explicitly version model-assisted outputs.
- Optional remote conversation features require careful context selection and a separate consent model.
- Cross-device continuity cannot be treated as a trivial sync feature.

## Rejected alternatives

- **Cloud-first memory lake:** rejected because it expands the highest-risk trust boundary before product value is proven.
- **Store only final pet state:** rejected because it breaks explanation, correction, and complete forgetting.
- **Write annotations back to source agent memory:** rejected because Memoryling must not mutate another tool's memory system.
