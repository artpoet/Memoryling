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
- [x] generate exact per-window app-command permissions and prove all eight current sensitive memory commands fail closed from `pet` independently at the production ACL and caller-label layers
- [x] add a content-minimized `CreatureRenderState` boundary and content-free revision synchronization to pet and detail
- [x] persist and clamp pet position across restart with monitor／work-area context, move／scale settle, topology polling, atomic shell settings, anchor-preserving onboarding resize, and pure 100–200% geometry tests
- [x] keep the visible real-memory-off state, transparent pet, one-time bilingual onboarding, reduced-motion behavior, and browser truth boundary
- [x] prove raw fixture approve／restart／lineage／forget consistency through native and packaged desktop smoke
- [x] pass a normal Explorer-launched current-user NSIS install, actual installed Start shortcut cold and resident single-instance relaunch, core pet／main lifecycle, explicit Quit, and retained-data uninstall UAT
- [x] pass 23 frontend tests and 29 Rust tests, including first-open SQLite concurrency and lifecycle rollback
- [ ] live-test 125／150／200% and mixed-DPI movement, monitor hot-unplug, taskbar relocation, and adjacent-desktop pet hitbox
- [ ] complete `Win+B`, Narrator／NVDA, keyboard-only, sign-out／shutdown, and remaining accessibility UAT
- [ ] prove compact／wide／tall／long growth envelopes; only the compact baseline exists, and production-supported real-memory／growth implementation has not started

Exit condition: launching the packaged app shows exactly one recoverable floating pet; every supported entry opens exactly one detail window; pet attempts to invoke sensitive commands fail closed; both surfaces stay privacy-safe and state-consistent; browser mode remains honest; and no network boundary is added.

**Exit status: not met, with the core 0.2.0 packaged slice implemented and preserved as a no-redo baseline.** Automated tests and current-host core native／packaged flows pass, but live DPI／mixed-monitor／taskbar／hitbox, `Win+B`, Narrator／NVDA, sign-out／shutdown, and non-compact envelope gates remain open. The WebView2-missing bootstrapper stays deferred. Source v0.3.0 adds an experimental work-record pilot but has no packaged installer UAT. An early agent-direct launch that triggered Windows virtualization is an invalid harness artifact, not a product failure; only normal Explorer and installed-shortcut runs count as packaged evidence.

## Phase 1 — First real memory

Fixture and local-store foundation completed:

- [x] versioned memory-event schema v1 for one synthetic `completion` record
- [x] fixed-path, read-only adapter for one bundled fictional Codex-shaped fixture
- [x] fixture selection, scope explanation, record preview, explicit consent, and cancel flow
- [x] local SQLite schema v2: migration 0001 for the fixture graph plus migration 0002 for canonical source-consent scopes and deterministic legacy-fixture backfill
- [x] deterministic completion star, source lineage, and “Why did this happen?” inspector
- [x] application-level deletion and deterministic recomputation for the supported fixture path

Primary Agent-memory source completed in the source v0.5.0 tree:

- [x] refresh official-source research and accept [ADR-0007](adr/0007-codex-agent-memory-auto-sync.md), superseding the work-record pilot as the primary source
- [x] implement one-time schema-v2 source consent for the exact Codex `memories` root, two top-level allowlisted generated files, local derivation, and automatic read-only sync
- [x] keep raw Agent-memory text Rust／SQLite-only and expose only redacted preview metadata, hashes, sync status, and content-minimized pet state
- [x] add startup, 15-minute, and manual synchronization with transactional replacement／recomputation, missing-source withdrawal／recovery, and last-valid-state preservation for unsafe input
- [x] add SQLite schema v4 sync state, aggregate memory-continuity signal, visible memory halo, complete local forgetting, and explicit exclusion from Daily Scout
- [x] pass synthetic temporary-file adapter, full-scope consent, sync, missing／recovery, redaction, render-safe, ACL, and frontend flow coverage without reading private Agent memory

Experimental work-record pilot completed in the source v0.3.0 tree:

- [x] record the historical 2026-08-12 conclusion; current official memory-location evidence and the new bounded connector supersede its primary-source recommendation while retaining the no-stable-third-party-schema warning
- [x] document the distinction between durable memory and a version-bound Codex work／thread-history pilot in the [source-format evaluation](research/2026-08-12_codex-source-format-evaluation.md) and proposed [ADR-0005](adr/0005-codex-thread-history-source-pilot.md)
- [x] implement a Rust-only fixed local Codex Desktop executable boundary, exact `codex-cli 0.134.0` fail-closed pin, and local App Server stdio calls limited to documented `thread/list` and `thread/read`
- [x] implement user-triggered content-minimized listing, Rust-only raw identifiers, exactly one selected completed thread, last-completed `final_answer` extraction, content-free preview, explicit completion confirmation, and canonical consent hash
- [x] enforce one active source, persist consent scope／normalized record in SQLite schema v2, keep external lineage content-free, and forget only Memoryling's local copy and downstream effects
- [x] extend independent production-ACL and caller-label denial coverage to all eight sensitive commands; bound the shared operation deadline, output, and child-process cleanup
- [x] pass synthetic adapter／migration／consent／lineage／forgetting coverage and a content-free live `thread/list` smoke on the exact pinned CLI without selecting a thread, calling `thread/read`, or exposing private content

Remaining gates for this phase:

- [ ] obtain separate authorization for one exact private Agent-memory source and its read／local-storage／derivation／automatic-sync UAT scope
- [ ] complete content-free private UAT for preview → consent → halo → restart → automatic change sync → missing-source withdrawal／recovery → forget, without recording memory text
- [ ] obtain separate authorization naming one exact completed private thread and the UAT scope before any private `thread/read`
- [ ] complete private UAT for list → one selection → redacted preview → consent → one effect → restart → explanation → forget, recording only content-free pass／fail evidence and proving the original thread remains unchanged
- [ ] complete packaged v0.5.0 synthetic compatibility UAT; the verified v0.2.0 installer remains the historical no-redo artifact and does not prove the new connector path
- [ ] monitor official Codex memory storage guidance; any filename, scope, or semantic drift requires a new adapter version and privacy review

Exit condition: one explicitly approved Codex Agent-memory source can create, update, withdraw, recover, explain, and remove a creature change locally without any network request, with private and packaged acceptance recorded.

**Exit status: not met.** The v0.5.0 source implements the bounded Agent-memory vertical slice and synthetic proof, but no private Agent-memory UAT or v0.5.0 installer acceptance has run. Generated Codex memory files also remain a versioned integration surface rather than a stable third-party schema.

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

**Exit status: not met.** Memoryling currently has no public Release, production-supported real-memory connector, private-thread acceptance, external-user evidence, or issue／fix／follow-up-release loop. The source-only experimental pilot does not satisfy those gates and does not authorize submitting an application.

## Phase 2 — A life that continues

Design foundation recorded: the user confirmed the biological／organic plus restrained sacred-premium direction and a content-responsive space with many variants. The current concept forms are vocabulary and adjacent-bridge references, not a fixed pre-authored pet roster. The [future creature-growth boundary](ARCHITECTURE.md#future-creature-growth-boundary), [evolving creature system draft](drafts/deep-interview-evolving-creature-system-2026-08-11.md), [Agent-memory variation rules](drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md), and proposed [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md) describe a deterministic, lineage-aware weighted profile compiled into bounded `MorphologyRecipe` variants. The proposed PM split uses authorized Agent-use behavior for primary morphology, authorized history outcomes for maturity and marks, collaboration mode for local motion, and content domain only for a secondary material accent. This is design evidence, not implemented Phase 2 functionality, an accepted schema, or an extension of the current fixture consent.

- [ ] persistent creature traits and visual marks
- [ ] versioned identity core, growth contributions, recomputable creature genome, and evolution stages
- [ ] extend the current one-source Agent-memory auto-sync scope with disable／re-enable and correction controls; reuse future records only within unchanged boundaries, and require a new revision preview／consent before category／purpose／mapping expansion
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

**Exit status: not met.** Source v0.4.0 persists only the narrow import consent scope; reusable ongoing scope behavior, A／B／C evidence lanes, outcome gate, ephemeral hint, recipe compiler, bridge records, and correction／disable recomputation remain proposed and do not exist in the current runtime.

## Phase 3 — Useful initiative

Daily Memory Scout source slice:

- [x] optional OpenAI BYOK setup with an official key link, Windows Credential Manager, and a clearly unchanged API-free ordinary pet
- [x] visible allowlisted coarse-context preview, purpose-specific consent hash, Rust-only fixed endpoint／model／Web Search, `store: false`, and annotation-derived citations
- [x] one attempt per local date while the app runs, honest failure without same-day automatic retry, and no missed-day replay
- [x] compact bilingual pet message／source card plus neutral pet ready state, turn-off, clear-history, delete-key, reset, and source-forget invalidation
- [x] synthetic frontend／Rust coverage for outbound minimization, citation validation, daily idempotency, ACL separation, and deletion behavior
- [ ] explicitly authorized paid smoke using reviewed synthetic／coarse context; never infer consent from a generic continuation request
- [ ] packaged v0.4.0 native UAT for enable／restart-no-rerun／result／disable／key-delete without capturing the key or private content
- [ ] synthetic quality evaluation broad enough to judge relevance and low-value fallback before beta claims

- [ ] reminder-candidate derivation
- [ ] quiet hours, daily budget, urgency, snooze, and global off
- [ ] native desktop notifications
- [ ] feedback loop for “helpful,” “not now,” and “never remind me of this”

Exit condition: useful initiative remains within hard limits, every message traces to approved source context and cited evidence, and reminders／notifications satisfy their separate quiet-hour and budget controls.

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
