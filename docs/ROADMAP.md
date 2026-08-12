# Roadmap

This roadmap describes intent, not a delivery promise.

## Phase 0 — Honest concept shell

- [x] Tauri 2 + React + TypeScript desktop foundation
- [x] English and Traditional Chinese concept experience
- [x] visible “memory access is off” state
- [x] initial open-source governance and CI
- [x] replace scaffold icons with generated Memoryling test artwork; public-release visual signoff remains separate
- [x] generate a local current-user Windows x64 NSIS fixture-only test artifact and bilingual user guide
- [x] complete current-host current-user install／launch／fixture／restart／uninstall UAT, including retention and explicit delete-app-data choices
- [ ] exercise the WebView2-missing bootstrapper in a safe disposable Windows x64 environment
- [x] record the finalized local test-artifact checksum and fresh Windows CI evidence
- [x] decide that v0.1.0 remains unsigned, local-test-only, and not a public release
- [x] build and hash unsigned `Memoryling_0.2.0_x64-setup.exe`: version 0.2.0, 2,875,965 bytes, SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`, `NotSigned`
- [ ] complete code signing and public-distribution readiness before any public release
- [ ] keyboard and screen-reader audit

## Phase 0.5 — Pet-first desktop presence

The 0.2.0 vertical slice now implements the user-confirmed “two surfaces, one life” direction from the [pet-first desktop shell draft](drafts/pet-first-desktop-shell-2026-08-11.md) and proposed [ADR-0003](adr/0003-pet-first-two-window-desktop-shell.md). Normal presence is one transparent floating pet with one-time onboarding and visible memory-off honesty; right-click or a focused-pet keyboard action opens the native menu, while tray, Start Menu, and installed-shortcut relaunch recover the one detail surface. ADR-0003 remains Proposed because the remaining live Windows acceptance matrix is not complete.

- [x] confirm the pet-only default surface and on-demand detail direction
- [x] document native lifecycle, privacy, accessibility, DPI, and recovery requirements
- [x] add pre-created `pet` and hidden `main` surfaces with Rust-owned show／hide／focus and compensating close／minimize／restore lifecycle
- [x] add native pet context menu, focused keyboard equivalent, tray Show／Hide／Open／Quit, safe caller-bound dragging, and single-instance relaunch
- [x] generate exact per-window app-command permissions and prove all six sensitive memory commands fail closed from `pet` independently at the production ACL and caller-label layers
- [x] add a content-minimized `CreatureRenderState` boundary and content-free revision synchronization to pet and detail
- [x] persist and clamp pet position across restart with monitor／work-area context, move／scale settle, topology polling, atomic shell settings, anchor-preserving onboarding resize, and pure 100–200% geometry tests
- [x] keep the visible real-memory-off state, transparent pet, one-time bilingual onboarding, reduced-motion behavior, and browser truth boundary
- [x] prove raw fixture approve／restart／lineage／forget consistency through native and packaged desktop smoke
- [x] pass a normal Explorer-launched current-user NSIS install, actual installed Start shortcut cold and resident single-instance relaunch, core pet／main lifecycle, explicit Quit, and retained-data uninstall UAT
- [x] pass 23 frontend tests and 29 Rust tests, including first-open SQLite concurrency and lifecycle rollback
- [ ] live-test 125／150／200% and mixed-DPI movement, monitor hot-unplug, taskbar relocation, and adjacent-desktop pet hitbox
- [ ] complete `Win+B`, Narrator／NVDA, keyboard-only, sign-out／shutdown, and remaining accessibility UAT
- [ ] prove compact／wide／tall／long growth envelopes; only the compact baseline exists, and real-source／growth implementation has not started

Exit condition: launching the packaged app shows exactly one recoverable floating pet; every supported entry opens exactly one detail window; pet attempts to invoke sensitive commands fail closed; both surfaces stay privacy-safe and state-consistent; browser mode remains honest; and no network boundary is added.

**Exit status: not met, with the core 0.2.0 slice implemented.** Automated tests and current-host core native／packaged flows pass, but live DPI／mixed-monitor／taskbar／hitbox, `Win+B`, Narrator／NVDA, sign-out／shutdown, and non-compact envelope gates remain open. The WebView2-missing bootstrapper stays deferred. An early agent-direct launch that triggered Windows virtualization is an invalid harness artifact, not a product failure; only normal Explorer and installed-shortcut runs count as packaged evidence.

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

## Cross-phase track — Public beta and OSS maintainer evidence

This track supports a future [Codex for Open Source application](research/2026-08-12_codex-for-open-source-readiness.md). It is not an OpenAI-official checklist and does not replace the Phase 0／0.5／1 product gates.

- [ ] close the deferred WebView2 clean-environment check plus signing and public-distribution readiness before beta release
- [ ] add a strong README hero and a 60–90 second synthetic-data demo with honest `Working now / Next / Vision` labels
- [ ] publish a verifiable GitHub beta Release with checksums, release notes, installation boundaries, privacy, and security guidance
- [ ] recruit genuine external testers and record consented feedback plus reproducible release／download evidence
- [ ] complete at least one public feedback／issue → maintainer response → fix → follow-up release loop
- [ ] publish the reusable adapter／event／lineage／forgetting contract and synthetic conformance fixtures at a contributor-ready level
- [ ] refresh official program terms, GitHub profile, live repository metrics, and all application claims before submission
- [ ] submit only after the readiness evidence is real and mutually consistent

Exit condition: a public beta proves the core promise through one user-selected real source; external people have tested it; at least one visible maintainer loop is complete; and every application metric can be reproduced from public or consented evidence. There is no invented star, download, or tester threshold.

**Exit status: not met.** Memoryling currently has no public Release, real-source connector, external-user evidence, or issue／fix／follow-up-release loop.

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
