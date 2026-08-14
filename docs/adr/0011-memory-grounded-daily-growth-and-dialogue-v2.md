# ADR-0011: Memory-grounded daily growth and rolling dialogue protocol v2

- Status: Accepted
- Date: 2026-08-14
- Supersedes: decisions 1, 3, 6, and 7 of [ADR-0008](0008-agent-operated-memoryling-protocol.md)
- Extends: [ADR-0010](0010-installed-app-teaches-agent-activation.md)

## Context

The first Agent-operated slice proved the local handoff, but its small replace-all dialogue set, fixed ambient cadence, and operation-shaped appearance did not yet feel like a pet that keeps living with the user. The activation wording also needed to remain unmistakably different from requests to wake the project or Agent.

The App must remain deterministic and local-first. Semantic interpretation stays in the already-authorized Agent; the App must not scan Agent memory, call an AI API, or retain obsolete derived conversation history.

## Decision

1. The exact Traditional Chinese activation phrase is `醒來吧我的寵物`; English remains `Memoryling, wake up`. General requests to wake the project, Agent, self, or `AI-WAKEUP.md` do not activate the pet.
2. Before reading memory for this workflow, the Agent runs the helper's readiness-only check. A compatible Memoryling 0.7.0-or-newer process must already be open. The Agent never launches the App.
3. Protocol v2 contains one activity profile, one journey state, 1–12 opaque evidence references, one appearance plan, and exactly 48 bilingual dialogue cards: 8 opening, 20 interaction, 16 ambient, and 4 appearance.
4. Dialogue is semi-specific: it may reflect recognizable work themes, progress, friction, rest, or return, but it must not expose project names, paths, source wording, secrets, diagnoses, or unsupported facts. Each localized line is at most 160 characters.
5. Stable dialogue IDs have rolling semantics. If a later operation retains the same ID and the same bilingual text, its use count and last-used time survive. Changed or absent cards are retired, and new cards start unused. The App retains no history of retired dialogue content.
6. Appearance plans are `hold`, `change`, or `reset`. A `change` needs either one explicit milestone or at least two consistent signals, must cite package-local evidence, and must name a visible supported activity and journey target. Weak plans hold.
7. At most one persistent visible appearance change is applied per local calendar day. A second qualified same-day change becomes one pending plan and may apply on the next local day. Current and pending appearance keep only opaque lineage. `reset` requires explicit source-removal evidence.
8. Ambient dialogue uses a local 35–70 minute randomized opportunity, remains blocked from 22:00 through 09:00, waits at least 10 minutes after any displayed line, and is capped at seven lines per local day. Click dialogue does not consume that budget and has a two-second anti-stack interval.
9. The pet presents operation dialogue in a styled speech bubble. It is dismissible, clamps to three visual lines, and automatically hides after seven seconds. Presentation creates no new semantic content.

## Consequences

- Each activation prepares enough varied material for continued use without app-side generation.
- Familiar useful lines can keep their local usage history while stale derived content disappears.
- Appearance can express evidence-backed continuity without changing repeatedly in one day.
- The App speaks more often than protocol v1 while quiet hours, daily limits, and user dismissal remain enforceable.
- Protocol v1 packages are not accepted by the v2 inbox; the Agent skill, helper, schema, example, and App must move together.

## Privacy and deletion

Readiness checking is process-metadata-only and occurs before any pet-workflow memory read. Packages remain lossy derived artifacts. Raw memory, prompts, reasoning, paths, names, credentials, and source text are forbidden. Clear deletes the current operation, rolling counters, current and pending appearance, and their local lineage without writing to Agent-owned memory.

## Rejected alternatives

- **Let the App generate more dialogue through an AI API:** expands cost, network, credential, and provider-retention boundaries.
- **Replace every counter on every operation:** makes retained dialogue repeat as if it were new.
- **Change appearance on every activation:** rewards invocation frequency rather than evidence and creates visual instability.
- **Use a short generic activation phrase:** risks colliding with project／Agent wake-up requests.

## Rollback

Stop accepting protocol v2, clear v2 operation and appearance tables, and restore the v1 schema／inbox only through a new accepted decision. Agent-owned context remains unchanged.
