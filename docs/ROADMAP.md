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

Design foundation recorded: the user confirmed the biological／organic plus restrained sacred-premium direction and the goal of content-responsive routes. The [evolving creature system draft](drafts/deep-interview-evolving-creature-system-2026-08-11.md) and proposed [ADR-0004](adr/0004-deterministic-content-derived-evolution-paths.md) propose a deterministic, lineage-aware branch-and-blend implementation. Distant stages and routes may diverge, while each adjacent stage or reroute retains a versioned, understandable evolution bridge. Permanent growth comes only from approved memory-derived data; time supplies reversible presentation state. This is design evidence, not implemented Phase 2 functionality or an accepted route schema.

- [ ] persistent creature traits and visual marks
- [ ] versioned identity core, growth contributions, recomputable creature genome, and evolution stages
- [ ] versioned, lineage-aware `EvolutionPathProfile` with multiple content-derived routes, bounded blending, and deterministic rerouting
- [ ] deterministic EvolutionBridge records and adjacent-stage continuity acceptance
- [ ] layered local renderer with no runtime image-generation dependency
- [ ] multi-day story event state machine
- [ ] habitat changes linked to completions and recurring themes
- [ ] contradiction events across approved sources
- [ ] event history and manual correction controls
- [ ] reduced-motion, keyboard, screen-reader, high-contrast, and 200% zoom acceptance for automatic evolution
- [ ] screenshot／streaming privacy mode and neutral growth-summary behavior before public testing
- [ ] a local source-detail unlock gate for Growth Journal explanations before exposing private record details

Exit condition: the creature demonstrates continuity across restarts, explains every persistent change, and produces different but understandable routes from different synthetic approved-memory profiles; forgetting support recomputes or removes the route without ghost modules or runtime model calls.

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
