# Roadmap

This roadmap describes intent, not a delivery promise.

## Phase 0 — Honest concept shell

- [x] Tauri 2 + React + TypeScript desktop foundation
- [x] English and Traditional Chinese concept experience
- [x] visible “memory access is off” state
- [x] initial open-source governance and CI
- [x] replace scaffold icons with generated Memoryling test artwork; public-release visual signoff remains separate
- [x] generate a local current-user Windows x64 NSIS fixture-only test artifact and bilingual user guide
- [ ] complete human installer, WebView2 prerequisite, uninstaller, and delete-app-data click-through UAT
- [x] record the finalized local test-artifact checksum and fresh Windows CI evidence
- [ ] complete code signing and public-distribution review
- [ ] keyboard and screen-reader audit

## Phase 0.5 — Pet-first desktop presence

Design foundation recorded: the user-confirmed [pet-first desktop shell draft](drafts/pet-first-desktop-shell-2026-08-11.md) and proposed [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md) define “two surfaces, one life.” Normal presence is one transparent floating pet; right-click is the primary detail entry, while `Win+B` tray, Start Menu, and packaged installed-shortcut paths provide recovery. Focused-pet keyboard actions open the same menu. This is design evidence, not implemented desktop behavior.

- [x] confirm the pet-only default surface and on-demand detail direction
- [x] document native lifecycle, privacy, accessibility, DPI, and recovery requirements
- [ ] add pre-created `pet` and hidden `main` surfaces with Rust-owned show／hide／focus and close／minimize／restore lifecycle
- [ ] add native pet context menu, focused keyboard equivalent, tray Show／Hide／Open／Quit, and single-instance relaunch
- [ ] generate per-window app-command permissions, deny all sensitive memory commands from `pet`, and test caller-label defense
- [ ] add a content-minimized `CreatureRenderState` boundary and cross-surface revision synchronization
- [ ] persist and clamp pet position across restart, work-area changes, mixed DPI, and monitor removal
- [ ] keep the visible real-memory-off state, one-time onboarding, reduced-motion behavior, and bilingual parity
- [ ] prove approve／restart／explain／forget consistency through packaged native desktop smoke
- [ ] pass keyboard, Narrator／NVDA, 100–200% DPI, multi-monitor, shortcut, tray, and NSIS lifecycle UAT

Exit condition: launching the packaged app shows exactly one recoverable floating pet; every supported entry opens exactly one detail window; pet attempts to invoke sensitive commands fail closed; both surfaces stay privacy-safe and state-consistent; browser mode remains honest; and no network boundary is added.

**Exit status: not met.** The product direction and implementation contract are recorded, but the current app still opens one standard window and has no pet window, tray, or two-window lifecycle.

## Phase 1 — First real memory

Fixture foundation completed:

- [x] versioned memory-event schema v1 for one synthetic `completion` record
- [x] fixed-path, read-only adapter for one bundled fictional Codex-shaped fixture
- [x] fixture selection, scope explanation, record preview, explicit consent, and cancel flow
- [x] local SQLite schema v1 with migration 0001
- [x] deterministic completion star, source lineage, and “Why did this happen?” inspector
- [x] application-level deletion and deterministic recomputation for the supported fixture path

Real-source work required for the phase exit:

- [ ] validate and document a supported user-owned Codex durable-memory format
- [ ] read only an exact external source explicitly selected by the user, without tool-home scanning
- [ ] add real-source scope disclosure, redacted preview behavior, consent, and invalid-input coverage
- [ ] complete explicitly authorized private-data UAT without exposing source content in git, logs, CI, or screenshots

Exit condition: one approved Codex memory can create one explainable, removable creature change without any network request.

**Exit status: not met.** The bundled synthetic fixture proves the local pipeline, but it is not a user-owned Codex memory or a production connector.

## Phase 2 — A life that continues

Design foundation recorded: the user confirmed the biological／organic plus restrained sacred-premium direction and a content-responsive space with many variants. The current concept forms are vocabulary and adjacent-bridge references, not a fixed pre-authored pet roster. The [future creature-growth boundary](ARCHITECTURE.md#future-creature-growth-boundary), [evolving creature system draft](drafts/deep-interview-evolving-creature-system-2026-08-11.md), [Agent-memory variation rules](drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md), and proposed [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md) describe a deterministic, lineage-aware weighted profile compiled into bounded `MorphologyRecipe` variants. The proposed PM split uses authorized Agent-use behavior for primary morphology, authorized history outcomes for maturity and marks, collaboration mode for local motion, and content domain only for a secondary material accent. This is design evidence, not implemented Phase 2 functionality, an accepted schema, or an extension of the current fixture consent.

- [ ] persistent creature traits and visual marks
- [ ] versioned identity core, growth contributions, recomputable creature genome, and evolution stages
- [ ] add a versioned `SourceConsentScope` over one selected source and adapter version, allowed data categories, and named derivation purposes; derive automatically only within that scope, use a separate scope for another source, and require a new revision preview／consent before category／purpose／mapping expansion
- [ ] implement the A／C／B evidence contract: A Agent-use behavior alone shapes primary morphology, C outcome-qualified history gates reshaping／advances maturity／creates lineage marks without selecting a silhouette, and B collaboration mode affects only bounded local movement
- [ ] restrict approved content-domain influence to a secondary material／surface accent rather than stage, main form, or personality inference
- [ ] derive `EphemeralActivityHint` from newly normalized in-scope records as a content-minimized, memory-only TTL state that cannot enter SQLite, contributions, maturity, or permanent recipes
- [ ] keep any optional live-presence adapter as a distinct, separately consented, content-free, TTL-bound source that cannot mutate permanent growth
- [ ] require multiple deduplicated independent outcome-qualified canonical groups before permanent structural reshaping; prove time, record volume, duplicate sources, tokens, session length, and open hours add no XP
- [ ] versioned, lineage-aware `EvolutionPathProfile` as a weighted influence vector, with deterministic recipe recomputation
- [ ] finite versioned module catalog, compatibility matrix, visual-slot caps, and quantized `MorphologyRecipe` compiler for many bounded variants
- [ ] deterministic EvolutionBridge records and adjacent-stage continuity acceptance
- [ ] atomically rederive profile, maturity, recipe, bridges, marks, journal, explanations, and caches after forget, approved-evidence correction, or consent-scope disable, with no ghost modules or stale lineage
- [ ] layered local renderer with no runtime image-generation dependency
- [ ] multi-day story event state machine
- [ ] habitat changes linked to completions and recurring themes
- [ ] contradiction events across approved sources
- [ ] event history and manual correction controls
- [ ] reduced-motion, keyboard, screen-reader, high-contrast, and 200% zoom acceptance for automatic evolution
- [ ] screenshot／streaming privacy mode and neutral growth-summary behavior before public testing
- [ ] a local source-detail unlock gate for Growth Journal explanations before exposing private record details

Exit condition: synthetic fixtures prove that `SourceConsentScope` blocks unapproved source／category／purpose expansion; A／C／B and secondary-domain evidence stay within their assigned influence limits; multiple deduplicated outcome-qualified groups—not time or usage volume—gate permanent reshape; forget, correction, and scope disable deterministically rederive all downstream state; every persistent recipe change has an understandable `EvolutionBridge`; and rendering makes no runtime model call.

**Exit status: not met.** The consent scope, A／B／C evidence lanes, outcome gate, ephemeral hint, recipe compiler, bridge records, and correction／disable recomputation are proposed only and do not exist in the current fixture runtime.

## Phase 3 — Useful initiative

- [ ] reminder-candidate derivation
- [ ] quiet hours, daily budget, urgency, snooze, and global off
- [ ] native desktop notifications
- [ ] feedback loop for “helpful,” “not now,” and “never remind me of this”

Exit condition: reminders remain within hard limits and all notifications trace to approved source memories.

## Phase 4 — Conversation

- [ ] local-context assembly with redaction controls
- [ ] conversation-provider decision and ADR
- [ ] optional local-model path
- [ ] bilingual grounded dialogue
- [ ] explanation and correction for remembered claims

Exit condition: users can see what context supported a response and prevent future use of a memory.

## Later possibilities

- additional agent adapters
- opt-in encrypted multi-device continuity
- creature and habitat accessibility themes
- extension API for community-authored story systems

These are deliberately deferred until the single-device trust model is proven.
