# ADR-0004: Deterministic content-derived evolution paths

- Status: Proposed
- Date: 2026-08-11
- Updated: 2026-08-12
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0002](0002-sqlite-v1-fixture-first-memory.md), [ADR-0003](0003-pet-first-two-window-desktop-shell.md)
- Design detail: [Evolving creature system](../drafts/deep-interview-evolving-creature-system-2026-08-11.md); [Agent-memory variation rules](../drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md)

## Context

Memoryling's first growth draft allowed large-form evolution connected by adjacent `EvolutionBridge` records, but it did not define whether every creature followed one fixed morphology path. The user has confirmed a different direction: all forms should share a living, organic identity and a restrained sacred-premium material language, while approved Agent-activity evidence may lead the same life toward many different morphology variants. The current ImageGen concepts are reference material for family grammar, silhouette range, materials, and adjacent transitions; they are not a fixed pre-authored pet roster or production sprite set.

This must not become open-ended raw-text classification, personality diagnosis, or runtime image generation. The implemented app still supports only one synthetic `completion` event and one completion-star effect; no content-derived profile, consent-scope adapter, activity hint, or recipe compiler exists today. This ADR records a proposed future contract only: runtime remains fixture-only.

## Proposed decision

Memoryling will add a finite, versioned `PathContribution` projection in parallel with other structural `GrowthContribution` and `WorldEffect` projections. Path contributions aggregate into a weighted `EvolutionPathProfile`; the profile, genome, identity, stage, and a finite local module catalog then compile into a deterministic `MorphologyRecipe`:

1. **One visual family, many bounded variants.** Every recipe follows a shared visual grammar: living gaze or its bridged successor, organic motion, a transformable memory-seed motif, and restrained lilac／mint／indigo material ancestry. These are not immutable anatomy fields; adjacent bridges may preserve, grow, split, merge, relocate, or retire them when a readable successor carries the family relationship. Sacred-premium character comes from proportion, nacreous or porcelain folds, inner light, and precise edges—not religious symbols, rank, or prophecy language. Reference concepts show vocabulary extrema and transition ideas only; they do not define a route count or map one image to one activity axis.
2. **One source per consent scope, then local automatic derivation.** A versioned `SourceConsentScope` identifies one exact selected source／read-only adapter and adapter version, allowed data-category enums, permitted purposes, mapping version, consent-schema version, and consent revision. The user previews and consents to that scope once. Records already inside it, and later records that remain inside it, may then be normalized and derived locally without per-record or per-visual-change approval. Another source requires another scope; adding a data category, purpose, or materially different mapping semantics creates a new revision and requires a new preview and consent. Out-of-scope records contribute `0`. Each accepted event and every downstream projection retain the scope ID and machine-readable lineage. This does not authorize background monitoring or writes to the source.
3. **Evidence priority is A usage behavior > C journey outcome > B collaboration mode.** Only observable, allowlisted evidence categories are valid. Eligible A evidence alone controls primary structural tendency. C evidence gates permanent reshaping, advances maturity, and creates outcome marks, but cannot select or override the main silhouette when A is absent. B evidence cannot override A or C; it is limited to a documented posture, motion, or secondary rhythm channel. Separately, content domain may produce only a bounded `ContentDomainContribution` for surface／material detail; topic labels never select silhouette, maturity, stage, or a fixed character. None of these categories may infer medical, psychological, political, religious, moral, relationship-quality, sentiment, or personality labels.
4. **Recent activity is ephemeral; permanent reshaping requires sustained outcome-bearing evidence.** Recent in-scope activity may derive a content-minimized, lineage-bearing `EphemeralActivityHint` with a fixed TTL. It stays in memory, defaults to neutral, and may change only expression, pose, breathing, movement, or light presentation. It never becomes XP, a contribution, stage, genome, recipe, journal entry, or persistence row. Permanent reshaping is recomputed only from durable events that pass a versioned gate requiring multiple deduplicated canonical groups, independent support, sustained evidence, and explicit outcome semantics. A raw count of records or sources, elapsed time, token count, app／Agent open hours, session duration, or data volume never supplies support units or XP. A temporary hint is never “promoted”; the durable graph is derived afresh when its gate is met. Until the production gate's thresholds and independence rules are accepted and versioned, permanent reshaping fails closed.
5. **`PathMappingV1` is a legacy synthetic artifact, not the real-source contract.** Its `user-confirmed = 1000` import-preview bucket, connector confidence buckets `750／500／250`, `craft／inquiry／stewardship／exchange` axes, and balanced／hybrid thresholds remain only as deterministic regression fixtures for the bundled synthetic prototype. They do not implement the A > C > B priority, `SourceConsentScope`, recent-versus-durable split, or outcome-bearing permanent gate. A real connector must not feed `PathMappingV1`; before any real-source rollout, it must be replaced by a newly identified mapping version with fixed A／C／B taxonomy, independence rules, promotion thresholds, and content-domain channel. The new mapping and consent versions are pinned by `SourceConsentScope`; unknown or V1-only real-source inputs fail closed rather than being silently migrated.
6. **Canonical grouping and independence are explicit.** A versioned hash of event schema, event kind, and canonical content hash identifies one group. The group contributes once while retaining all supporting signal／event／consent-scope lineage. Cross-source copies do not become independent evidence merely because there are more sources; independence and outcome semantics must come from allowlisted, versioned fields. Conflicting valid tags fail closed for morphology until resolved; partial-source forgetting updates lineage, while removal of the last support deletes the contribution.
7. **Finite deterministic recipe compiler.** A versioned allowlisted module catalog, compatibility matrix, visual-slot caps, and quantized geometry／material／motion parameters compile the profile and genome into a `MorphologyRecipe`. Stable module-instance and parameter-bucket keys map through a separate `RecipeLineageMap` to supporting contribution／signal IDs, derivation version, consent-scope ID, and explanation keys; that map never enters pet render state. The space may contain many variants but remains bounded and enumerable for tests. The same canonical in-scope event set, identity seed, consent scope, derivation version, activity-mapping version, genome version, recipe-schema／compiler version, module-catalog version, bridge-rule version, and explicit local override must produce the same profile and recipe across import order, source enumeration, restart, and locale. An unknown version, module, or incompatible combination fails the transaction and leaves the last valid canonical recipe untouched. Only when no valid recipe has ever existed may the UI display a transient reviewed baseline plus an error; it cannot persist that fallback or report successful derivation.
8. **Correction, disable, and forgetting are full recomputation.** Correcting an event or category, narrowing／disabling a consent scope, disabling an adapter, or forgetting a source／record atomically clears affected ephemeral hints and recomputes contributions, path profile, genome, recipe decisions, bridges, marks, journal projections, explanations, lineage maps, and render caches from the remaining eligible events. A recipe change uses an adjacent `EvolutionBridge`; the UI must not instantly replace the creature or describe recomputation as injury, death, failure, or personality change.
9. **Bridges compare recipes without delaying truth.** Every adjacent stage or recipe change compares before／after `MorphologyRecipe` values and records preserved, grown, split, merged, relocated, or retired-with-successor modules. Each adjacent pair must retain at least one life／sensory continuity and one structural continuity. The canonical `afterRecipe` is committed atomically and never time-stepped. If a transition exceeds a versioned complexity cap, the presentation layer may deterministically derive content-free `BridgeFrameRecipe[]`; those frames never enter the genome, stage, contribution graph, next compilation input, or persistent store, and reduced-motion may skip them completely.
10. **Render-safe boundary.** Profile axes, weights, evidence-class labels, content-domain labels, and `RecipeLineageMap` remain inside Rust and the authorized aggregate explanation boundary. Agent or source identity is not a profile／recipe input; any retained source metadata may appear only behind a separately verified source-detail unlock gate. The pet renderer may receive only final visual-module IDs, bounded quantized parameters, stage, revision, mark IDs, and ephemeral presentation parameters. It must not receive raw source text, paths, names, content summaries, lineage maps, profile semantics, consent details, or sensitive classifications.
11. **No runtime AI dependency.** Image generation and design skills may help during art exploration. Shipping recipe compilation and rendering use local deterministic rules and bundled reviewed assets, with zero image or language model calls.

This ADR remains **Proposed** until synthetic profiles prove deterministic recipe compilation, visual distinction, adjacent continuity, privacy, forgetting, and packaged desktop behavior.

## Consequences

### Positive

- Different authorized Agent-activity patterns can visibly shape many variants without turning each launch into a random skin.
- A shared visual DNA keeps distant variants emotionally recognizable as one life while allowing substantial morphology differences.
- The A > C > B hierarchy keeps the dominant tendency readable while allowing mixed histories without permanently assigning the user a class; content domain remains a secondary material detail.
- Scope-level consent avoids repetitive per-record prompts while keeping source, category, purpose, and version boundaries explicit.
- Full lineage and recomputation preserve the existing explanation and forgetting contract.

### Costs and limits

- The project needs a `SourceConsentScope` schema, A／C／B evidence taxonomy, `EphemeralActivityHint`, versioned durable-reshape gate, successor mapping version, capped aggregation rules, lineage-bearing recipe compiler, versioned module catalog, compatibility matrix, synthetic profile matrix, and recomputation tests.
- Every supported module family and parameter level expands small-size silhouette, transparent-window bounds, animation, accessibility, and bilingual explanation acceptance.
- The current concept image is exploratory reference only; it is not a production asset, final SVG, or proof that adjacent transitions work.
- A real connector must expose or obtain safe structured activity signals before content can influence morphology. Unknown or untrusted tags fail closed and may still create a bounded non-morphology mark.

## Rejected alternatives

- **One fixed five-stage line for every user:** rejected because it cannot let approved content meaningfully shape permanent morphology.
- **One exclusive personality class:** rejected because it overstates inference, handles mixed histories poorly, and can turn source deletion into an identity judgment.
- **Direct raw-text-to-visual classification:** rejected because it widens privacy and explainability risk and is not deterministic enough for forgetting.
- **Runtime image generation after every memory:** rejected because it adds a network／model dependency, weakens reproducibility, and cannot guarantee lineage-safe rerendering.
- **Unlimited mixing of every morphology module:** rejected because it creates visual clutter, combinatorial test cost, and oversized transparent hit regions.
- **Per-record approval or recency-as-XP:** rejected because repeated prompts undermine the agreed source-and-purpose consent model, while time, volume, and passive presence are poor and privacy-expanding proxies for meaningful growth.

## Privacy impact

The evolution profile is a local derived projection, not a stored personality assessment. `SourceConsentScope` is local and purpose-bound; automatic processing is limited to its selected source and data categories. Explanations describe only supported evidence classes and aggregate counts that the detail surface is allowed to show. `EphemeralActivityHint` is content-minimized, expires, and cannot be promoted into durable evidence. Any future live-presence adapter may only feed that ephemeral contract after separate product and privacy approval. Pet IPC, DOM attributes, logs, screenshots, tests, and window or tray labels must remain content-minimized.

Forgetting the final supporting source must remove or recompute its profile contribution and every downstream visual, journal, explanation, and cache projection. This never modifies the read-only source and does not make a physical secure-erasure promise.

## Acceptance gate

Before this ADR can be marked Accepted, the project must prove with synthetic data that:

- a synthetic profile matrix produces many distinct but bounded recipes at the same identity and maturity stage, without a fixed pre-authored pet roster or one-to-one axis-to-body mapping;
- A／C／B collision fixtures prove that eligible A alone determines the primary form, C only gates maturity／reshape and supplies marks, and B never overrides either lane;
- content-domain changes alter only allowlisted surface／material buckets and never silhouette, maturity, stage, or primary structural tendency;
- consenting once to a `SourceConsentScope` allows automatic local derivation of later in-scope records without per-record prompts, while every source／category／purpose expansion requires a new consent revision;
- correcting an event, narrowing or disabling a scope, disabling an adapter, and forgetting a source all clear affected hints and atomically rederive every downstream persistent projection;
- recent-only evidence produces only an expiring `EphemeralActivityHint`; time, volume, tokens, open hours, duplicate records, and duplicate sources cannot cross the permanent gate;
- permanent reshaping occurs only when versioned fixtures satisfy sustained, multiple, deduplicated, independent, outcome-semantic evidence, and fails closed when any required dimension is absent;
- mixed profiles deterministically compile to allowlisted module combinations and quantized parameters;
- every recipe follows the shared living／organic and restrained premium grammar, while any transformed or retired eye／seed signature has an understandable adjacent successor;
- every adjacent stage or recipe change exposes an `EvolutionBridge` with at least one life／sensory continuity and one structural continuity;
- duplicate and 1,000-record stress fixtures respect contribution, profile, stage, recipe, and visual-slot caps;
- unknown tags and future mapping versions fail closed;
- the legacy `PathMappingV1` boundary fixtures still reproduce their synthetic-only fixed-point table, while any real-source attempt to use its user-confirmed／connector buckets fails closed;
- forgetting and transaction rollback leave no ghost module, journal entry, or false UI success;
- profile and recipe output are stable across import order, source order, restart, locale, and reduced-motion mode;
- any future recent-activity or live-presence adapter creates no database row, contribution, or journal entry; cold start is neutral, TTL expiry and clock skew cannot accumulate state, and active／idle／session changes do not mutate the persisted profile, genome, recipe, or stage;
- live-presence logs, render DTOs, and pet commands contain no Agent, project, path, or session identity;
- render DTOs, logs, snapshots, and pet commands contain no raw memory content or sensitive labels;
- the packaged Windows pet remains usable across the accepted recipe matrix, DPI settings, and transparent-window bounds with no runtime network request.
