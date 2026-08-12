# ADR-0004: Deterministic content-derived evolution paths

- Status: Proposed
- Date: 2026-08-11
- Updated: 2026-08-12
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0002](0002-sqlite-v1-fixture-first-memory.md), [ADR-0003](0003-pet-first-two-window-desktop-shell.md)
- Design detail: [Evolving creature system](../drafts/deep-interview-evolving-creature-system-2026-08-11.md)

## Context

Memoryling's first growth draft allowed large-form evolution connected by adjacent `EvolutionBridge` records, but it did not define whether every creature followed one fixed morphology path. The user has confirmed a different direction: all forms should share a living, organic identity and a restrained sacred-premium material language, while approved Agent-activity evidence may lead the same life toward many different morphology variants. The current ImageGen concepts are reference material for family grammar, silhouette range, materials, and adjacent transitions; they are not a fixed pre-authored pet roster or production sprite set.

This must not become open-ended raw-text classification, personality diagnosis, or runtime image generation. The implemented app still supports only one synthetic `completion` event and one completion-star effect; no content-derived profile or recipe compiler exists today.

## Proposed decision

Memoryling will add a finite, versioned `PathContribution` projection in parallel with other structural `GrowthContribution` and `WorldEffect` projections. Path contributions aggregate into a weighted `EvolutionPathProfile`; the profile, genome, identity, stage, and a finite local module catalog then compile into a deterministic `MorphologyRecipe`:

1. **One visual family, many bounded variants.** Every recipe follows a shared visual grammar: living gaze or its bridged successor, organic motion, a transformable memory-seed motif, and restrained lilac／mint／indigo material ancestry. These are not immutable anatomy fields; adjacent bridges may preserve, grow, split, merge, relocate, or retire them when a readable successor carries the family relationship. Sacred-premium character comes from proportion, nacreous or porcelain folds, inner light, and precise edges—not religious symbols, rank, or prophecy language. Reference concepts show vocabulary extrema and transition ideas only; they do not define a route count or map one image to one activity axis.
2. **Approved durable evidence only.** Permanent weights come only from versioned `DerivedSignal` records backed by explicitly selected, previewed, and approved sources／records／activity categories with machine-readable lineage. Connector-declared tags are valid only inside that approved scope; out-of-scope tags contribute `0`. Raw memory text, source count, record length, Agent names, project names, paths, token counts, open time, runtime model output, and unsaved randomness are not permanent inputs. No live Agent monitoring exists. Any future `LiveAgentPresence` feature requires separate consent and a source-specific read-only adapter; its allowlisted content-free enum stays in memory with a TTL, defaults to neutral, clears when disabled or unavailable, and never enters SQLite, logs, telemetry, contributions, or permanent growth. If live usage is ever meant to affect permanent form, it must first become a previewed, approved, versioned event with lineage.
3. **Activities, not personalities.** Initial safe axes may describe observable approved activities such as making, inquiry, stewardship, or exchange. They must never infer medical, psychological, political, religious, moral, relationship-quality, sentiment, or personality labels. The exact taxonomy and signal mapping remain proposed.
4. **Weighted profile, not branch selection.** Proposed `PathMappingV1` uses integer support buckets: user-confirmed tags yield `1000`; connector-declared tags yield `750／500／250` at confidence ranges `9000–10000／7500–8999／6000–7499` bps; missing or lower confidence yields `0`. Identical supports in one canonical group take the highest bucket without multiplying lineage. Each group caps at `1000`, each axis at `3000`, and morphology activates only at `growing` with at least three eligible groups and a top score of `1500`. Three or more axes at `1500` with a spread no greater than `500` produce balanced confluence; otherwise two axes at `1500` with a difference below `750` produce a hybrid; remaining profiles retain quantized influence weights, with at most one non-mark material／motion accent when the second axis reaches `1000`. Equal IDs are ordered by a documented stable axis order. These fixture axes influence a recipe; they do not select one fixed character or correspond one-to-one with a visual body. The exact table remains proposed until synthetic acceptance.
5. **Canonical grouping.** A versioned hash of event schema, event kind, and canonical content hash identifies one group. The group contributes once while retaining all supporting signal／event lineage. Conflicting valid tags fail closed for morphology until resolved; partial-source forgetting updates lineage, while removal of the last support deletes the contribution.
6. **Finite deterministic recipe compiler.** A versioned allowlisted module catalog, compatibility matrix, visual-slot caps, and quantized geometry／material／motion parameters compile the profile and genome into a `MorphologyRecipe`. Stable module-instance and parameter-bucket keys map through a separate `RecipeLineageMap` to supporting contribution／signal IDs, derivation version, and explanation keys; that map never enters pet render state. The space may contain many variants but remains bounded and enumerable for tests. The same canonical approved-event set, identity seed, derivation version, path-mapping version, genome version, recipe-schema／compiler version, module-catalog version, bridge-rule version, and explicit local override must produce the same profile and recipe across import order, source enumeration, restart, and locale. An unknown version, module, or incompatible combination fails the transaction and leaves the last valid canonical recipe untouched. Only when no valid recipe has ever existed may the UI display a transient reviewed baseline plus an error; it cannot persist that fallback or report successful derivation.
7. **Recipe changes are recomputation.** Approving or forgetting a source recomputes contributions, path profile, genome, recipe decisions, bridges, marks, journal projections, explanations, and render caches in the same local transaction boundary. A recipe change uses an adjacent `EvolutionBridge`; the UI must not instantly replace the creature or describe recomputation as injury, death, failure, or personality change.
8. **Bridges compare recipes without delaying truth.** Every adjacent stage or recipe change compares before／after `MorphologyRecipe` values and records preserved, grown, split, merged, relocated, or retired-with-successor modules. Each adjacent pair must retain at least one life／sensory continuity and one structural continuity. The canonical `afterRecipe` is committed atomically and never time-stepped. If a transition exceeds a versioned complexity cap, the presentation layer may deterministically derive content-free `BridgeFrameRecipe[]`; those frames never enter the genome, stage, contribution graph, next compilation input, or persistent store, and reduced-motion may skip them completely.
9. **Render-safe boundary.** Profile axes, weights, activity labels, and `RecipeLineageMap` remain inside Rust and the authorized aggregate explanation boundary. Agent or source identity is not a profile／recipe input; any retained source metadata may appear only behind a separately verified source-detail unlock gate. The pet renderer may receive only final visual-module IDs, bounded quantized parameters, stage, revision, and mark IDs. It must not receive raw source text, paths, names, content summaries, lineage maps, profile semantics, or sensitive classifications.
10. **No runtime AI dependency.** Image generation and design skills may help during art exploration. Shipping recipe compilation and rendering use local deterministic rules and bundled reviewed assets, with zero image or language model calls.

This ADR remains **Proposed** until synthetic profiles prove deterministic recipe compilation, visual distinction, adjacent continuity, privacy, forgetting, and packaged desktop behavior.

## Consequences

### Positive

- Different approved Agent-activity patterns can visibly shape many variants without turning each launch into a random skin.
- A shared visual DNA keeps distant variants emotionally recognizable as one life while allowing substantial morphology differences.
- Weighted profiles support mixed histories without permanently assigning the user a class.
- Full lineage and recomputation preserve the existing explanation and forgetting contract.

### Costs and limits

- The project needs an evidence schema, mapping version, capped aggregation rules, lineage-bearing recipe compiler, versioned module catalog, compatibility matrix, synthetic profile matrix, and recomputation tests.
- Every supported module family and parameter level expands small-size silhouette, transparent-window bounds, animation, accessibility, and bilingual explanation acceptance.
- The current concept image is exploratory reference only; it is not a production asset, final SVG, or proof that adjacent transitions work.
- A real connector must expose or obtain safe structured activity signals before content can influence morphology. Unknown or untrusted tags fail closed and may still create a bounded non-morphology mark.

## Rejected alternatives

- **One fixed five-stage line for every user:** rejected because it cannot let approved content meaningfully shape permanent morphology.
- **One exclusive personality class:** rejected because it overstates inference, handles mixed histories poorly, and can turn source deletion into an identity judgment.
- **Direct raw-text-to-visual classification:** rejected because it widens privacy and explainability risk and is not deterministic enough for forgetting.
- **Runtime image generation after every memory:** rejected because it adds a network／model dependency, weakens reproducibility, and cannot guarantee lineage-safe rerendering.
- **Unlimited mixing of every morphology module:** rejected because it creates visual clutter, combinatorial test cost, and oversized transparent hit regions.

## Privacy impact

The evolution profile is a local derived projection, not a stored personality assessment. Explanations describe only supported activity-signal categories and source counts that the detail surface is allowed to show. Live Agent presence is content-minimized, expires, and cannot be promoted into durable evidence without preview and approval. Pet IPC, DOM attributes, logs, screenshots, tests, and window or tray labels must remain content-minimized.

Forgetting the final supporting source must remove or recompute its profile contribution and every downstream visual, journal, explanation, and cache projection. This never modifies the read-only source and does not make a physical secure-erasure promise.

## Acceptance gate

Before this ADR can be marked Accepted, the project must prove with synthetic data that:

- a synthetic profile matrix produces many distinct but bounded recipes at the same identity and maturity stage, without a fixed pre-authored pet roster or one-to-one axis-to-body mapping;
- mixed profiles deterministically compile to allowlisted module combinations and quantized parameters;
- every recipe follows the shared living／organic and restrained premium grammar, while any transformed or retired eye／seed signature has an understandable adjacent successor;
- every adjacent stage or recipe change exposes an `EvolutionBridge` with at least one life／sensory continuity and one structural continuity;
- duplicate and 1,000-record stress fixtures respect contribution, profile, stage, recipe, and visual-slot caps;
- unknown tags and future mapping versions fail closed;
- mapping-boundary, duplicate-lineage, conflicting-tag, partial-source, and last-source fixtures match the proposed fixed-point table;
- forgetting and transaction rollback leave no ghost module, journal entry, or false UI success;
- profile and recipe output are stable across import order, source order, restart, locale, and reduced-motion mode;
- any future live-presence adapter creates no database row, contribution, or journal entry; cold start is neutral, TTL expiry and clock skew cannot accumulate state, and active／idle／session changes do not mutate the persisted profile, genome, recipe, or stage;
- live-presence logs, render DTOs, and pet commands contain no Agent, project, path, or session identity;
- render DTOs, logs, snapshots, and pet commands contain no raw memory content or sensitive labels;
- the packaged Windows pet remains usable across the accepted recipe matrix, DPI settings, and transparent-window bounds with no runtime network request.
