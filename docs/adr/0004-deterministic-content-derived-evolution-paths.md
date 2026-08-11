# ADR-0004: Deterministic content-derived evolution paths

- Status: Proposed
- Date: 2026-08-11
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0002](0002-sqlite-v1-fixture-first-memory.md), [ADR-0003](0003-pet-first-two-window-desktop-shell.md)
- Design detail: [Evolving creature system](../drafts/deep-interview-evolving-creature-system-2026-08-11.md)

## Context

Memoryling's first growth draft allowed large-form evolution connected by adjacent `EvolutionBridge` records, but it did not define whether every creature followed one fixed morphology path. The user has confirmed a different direction: all forms should share a living, organic identity and a restrained sacred-premium material language, while approved memory content may lead the same creature through more than one evolution route.

This must not become open-ended raw-text classification, personality diagnosis, or runtime image generation. The implemented app still supports only one synthetic `completion` event and one completion-star effect; no content-derived route exists today.

## Proposed decision

Memoryling will add a finite, versioned `PathContribution` projection in parallel with other structural `GrowthContribution` and `WorldEffect` projections. Path contributions aggregate into an `EvolutionPathProfile`, which is then one input to the recomputable creature genome:

1. **One visual family, multiple routes.** Every route follows a shared visual grammar: living gaze or its bridged successor, organic motion, a transformable memory-seed motif, and restrained lilac／mint／indigo material ancestry. These are not immutable anatomy fields; adjacent bridges may preserve, grow, split, merge, relocate, or retire them when a readable successor carries the family relationship. Sacred-premium character comes from proportion, nacreous or porcelain folds, inner light, and precise edges—not religious symbols, rank, or prophecy language.
2. **Approved signals only.** Route weights come only from versioned `DerivedSignal` records backed by approved sources and machine-readable lineage. Raw memory text, source count, record length, names, runtime model output, and unsaved randomness are not route inputs.
3. **Activities, not personalities.** Initial safe axes may describe observable approved activities such as making, inquiry, stewardship, or exchange. They must never infer medical, psychological, political, religious, moral, relationship-quality, sentiment, or personality labels. The exact taxonomy and signal mapping remain proposed.
4. **Branch and blend.** Proposed `PathMappingV1` uses integer support buckets: user-confirmed tags yield `1000`; connector-declared tags yield `750／500／250` at confidence ranges `9000–10000／7500–8999／6000–7499` bps; missing or lower confidence yields `0`. Identical supports in one canonical group take the highest bucket without multiplying lineage. Each group caps at `1000`, each axis at `3000`, and morphology activates only at `growing` with at least three eligible groups and a top score of `1500`. Three or more axes at `1500` with a spread no greater than `500` produce balanced confluence; otherwise two axes at `1500` with a difference below `750` produce a hybrid; remaining active profiles use the top axis as dominant, with at most one non-mark material／motion accent when the second axis reaches `1000`. Equal IDs are ordered by a documented stable axis order, not used to force one winner. The exact table remains proposed until synthetic acceptance.
5. **Canonical grouping.** A versioned hash of event schema, event kind, and canonical content hash identifies one group. The group contributes once while retaining all supporting signal／event lineage. Conflicting valid tags fail closed for morphology until resolved; partial-source forgetting updates lineage, while removal of the last support deletes the contribution.
6. **Deterministic and versioned.** The same canonical approved-event set, identity seed, derivation version, path-mapping version, genome version, and explicit local override must produce the same quantized path profile and render IDs across import order, source enumeration, restart, and locale.
7. **Rerouting is recomputation.** Approving or forgetting a source recomputes contributions, path profile, genome, marks, journal projections, and explanations in the same local transaction boundary. A route change uses an adjacent `EvolutionBridge`; the UI must not instantly replace the creature or describe rerouting as injury, death, failure, or personality change.
8. **Render-safe boundary.** Route IDs, weights, and activity labels remain inside Rust and the authorized detail explanation boundary. The pet renderer may receive only final visual-module IDs, bounded geometry／motion parameters, stage, revision, and mark IDs. It must not receive raw source text, paths, names, content summaries, profile semantics, or sensitive classifications.
9. **No runtime AI dependency.** Image generation and design skills may help during art exploration. Shipping route selection and rendering use local deterministic rules and bundled reviewed assets, with zero image or language model calls.

This ADR remains **Proposed** until synthetic profiles prove deterministic routing, visual distinction, adjacent continuity, privacy, forgetting, and packaged desktop behavior.

## Consequences

### Positive

- Different approved memory patterns can visibly shape the creature without turning each launch into a random skin.
- A shared visual DNA keeps distant branches emotionally recognizable as one life while allowing substantial morphology differences.
- Weighted routes support mixed histories without permanently assigning the user a class.
- Full lineage and recomputation preserve the existing explanation and forgetting contract.

### Costs and limits

- The project needs a path schema, mapping version, capped aggregation rules, branch-aware renderer modules, synthetic fixtures, and rerouting tests.
- Every supported route expands small-size silhouette, transparent-window bounds, animation, accessibility, and bilingual explanation acceptance.
- The current concept image is exploratory reference only; it is not a production asset, final SVG, or proof that adjacent transitions work.
- A real connector must expose or obtain safe structured activity signals before content can influence morphology. Unknown or untrusted tags fail closed and may still create a bounded non-morphology mark.

## Rejected alternatives

- **One fixed five-stage line for every user:** rejected because it cannot let approved content meaningfully shape permanent morphology.
- **One exclusive personality class:** rejected because it overstates inference, handles mixed histories poorly, and can turn source deletion into an identity judgment.
- **Direct raw-text-to-visual classification:** rejected because it widens privacy and explainability risk and is not deterministic enough for forgetting.
- **Runtime image generation after every memory:** rejected because it adds a network／model dependency, weakens reproducibility, and cannot guarantee lineage-safe rerendering.
- **Unlimited mixing of every route module:** rejected because it creates visual clutter, combinatorial test cost, and oversized transparent hit regions.

## Privacy impact

The route profile is a local derived projection, not a stored personality assessment. Explanations describe only supported activity-signal categories and source counts that the detail surface is allowed to show. Pet IPC, DOM attributes, logs, screenshots, tests, and window or tray labels must remain content-minimized.

Forgetting the final supporting source must remove or recompute its route contribution and every downstream visual, journal, explanation, and cache projection. This never modifies the read-only source and does not make a physical secure-erasure promise.

## Acceptance gate

Before this ADR can be marked Accepted, the project must prove with synthetic data that:

- several single-axis profiles produce distinct silhouettes at the same identity and maturity stage;
- mixed profiles produce deterministic dominant-plus-secondary or balanced forms rather than one fixed route;
- every route follows the shared living／organic and restrained premium grammar, while any transformed or retired eye／seed signature has an understandable adjacent successor;
- every adjacent stage or reroute exposes at least one understandable `EvolutionBridge` transformation;
- duplicate and 1,000-record stress fixtures respect contribution, route, stage, and visual-slot caps;
- unknown tags and future mapping versions fail closed;
- mapping-boundary, duplicate-lineage, conflicting-tag, partial-source, and last-source fixtures match the proposed fixed-point table;
- forgetting and transaction rollback leave no ghost module, journal entry, or false UI success;
- route output is stable across import order, source order, restart, locale, and reduced-motion mode;
- render DTOs, logs, snapshots, and pet commands contain no raw memory content or sensitive labels;
- the packaged Windows pet remains usable across supported routes, DPI settings, and transparent-window bounds with no runtime network request.
