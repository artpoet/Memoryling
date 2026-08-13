# Changelog

All notable changes to Memoryling will be documented here.

The project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and intends to use semantic versioning after its first public release.

## [Unreleased]

### Added

- conversation-first wake-only phrases that show the existing pet without reading memory or creating an operation
- `Start-Memoryling.ps1` with exact installed-App resolution, minimum-version checks, bounded launch, and optional inbox-consumption confirmation

### Changed

- normal Agent operation now validates the installed App before submission, opens or recalls the pet automatically, and reports completion in the same conversation
- cold launch goes directly to the pet with OS-locale selection; the blocking first-run setup screen and its Tauri commands were removed
- single-instance relaunch returns to the existing pet instead of opening the detail window

### Security and privacy

- wake-only grants no memory access; launcher discovery is limited to an explicit path, exact running process, current-user uninstall registration, and exact current-user install candidates
- missing or stale App fails before an operation package is written; helpers never print the executable path or package content

### Validated

- full frontend／Rust checks, Rust formatting, Clippy, JSON parsing, and official skill validation pass
- missing App fails before write, a mismatched binary is rejected, and isolated `-SkipLaunch` submission remains available for test harnesses
- a freshly built v0.6.0 release binary consumed the synthetic operation and displayed the native pet; wake-only relaunch preserved one process and one pet window
- English／Traditional Chinese browser inspection confirms conversation-first copy, honest native boundary, no horizontal overflow, and no warning／error logs

## [0.6.0] - 2026-08-13

> Source-only Agent-operated milestone. The v0.2.0 installer remains the last installed-UAT baseline; no packaged v0.6.0 or private-memory acceptance is claimed.

### Added

- project trigger phrases `運作 Memoryling`, `執行 Memoryling`, and `Run Memoryling`
- validated `memoryling-operation` Agent skill with explicit authorization and minimization rules
- strict protocol-v1 JSON Schema, human reference, and committed synthetic example
- PowerShell validation and atomic app-local inbox submission without package-content output
- exact-file Rust inbox worker with regular-file／symlink／UTF-8／64 KiB／strict-schema guards
- SQLite migration 0005／schema v5 for Agent operation profile, hashed evidence, bilingual dialogue, lineage, runtime counters, and ambient daily budget
- render-state schema v6 with Agent operation state, coarse activity accent, milestone mark, and current dialogue
- on-open, on-interact, and ambient dialogue with time bounds, cooldown, max uses, 22:00–09:00 quiet hours, and two ambient lines per local day
- local **Clear this pet update** control
- accepted ADR-0008 and an Agent-operated architecture／privacy／user-guide SSOT chain

### Changed

- split responsibilities explicitly: the Agent understands and compiles; the app persists, renders, times, and clears
- made every new operation an authoritative transactional replacement so obsolete derived dialogue is not retained as history
- removed API-key and direct memory-connector setup from first run and the primary detail surface
- stopped automatic direct Codex-memory sync and Daily Scout scheduling; retained their code as labeled compatibility experiments only
- revised product copy around one slogan, local rule-driven life, and no app-side AI API
- bumped source version to 0.6.0 and store／render schemas to 5／6

### Security and privacy

- operation packages forbid raw memories, prompts, reasoning, paths, names, secrets, credentials, and tool output
- hashed source references never cross the render DTO
- duplicate operation ID＋digest is idempotent; reused ID with another digest fails closed
- synthetic-only protocol, persistence, replacement, clear, rendering, and submit-helper tests avoid user memory

### Validated

- official `skill-creator` validator passes for the project skill
- submit helper passes against the committed synthetic package in an isolated app-data directory
- targeted Rust Agent-operation and frontend Agent-operated surfaces pass
- full repository checks, browser smoke, CI, and Final Gate are recorded in `PROJECT_STATUS.md`

## [0.5.0] - 2026-08-13

> Source v0.5.0 development milestone. The primary path is now a one-time approved local Codex Agent-memory source with read-only automatic sync. No private-memory UAT or packaged v0.5.0 acceptance is claimed; v0.2.0 remains the installed no-redo baseline.

### Added

- `codex-local-memory-store` v1 connector for the exact configured Codex `memories` root, restricted to top-level `memory_summary.md` and `MEMORY.md`, with symlink／type／UTF-8／size／root-hash fail-closed guards
- Schema-v2 source-level consent for local derivation and future automatic in-scope sync without per-document selection; previews are content-redacted and expose only logical IDs, timestamps, character counts, and hashes
- Startup, 15-minute, and manual sync with SQLite migration 0004／schema v4 `source_sync_state`, transactional event replacement, missing-source withdrawal／recovery, last-valid-state preservation, and complete local disconnect
- Aggregate `agent-memory-continuity` derivation, visible memory halo, render-safe `codex-local` state, bilingual connection／sync／forget UX, and primary-source documentation in accepted ADR-0007
- Synthetic temporary-file, full-scope consent, sync, recovery, redaction, pet-safe DTO, ACL, and frontend vertical-slice coverage; no private memory was read

### Changed

- Codex Agent memory replaces manual work-record selection as the recommended primary source; the exact-version App Server work-record pilot remains a supplementary compatibility path
- Daily Scout remains restricted to a separately approved work record and explicitly excludes Agent-memory documents from outbound context
- Refined the deterministic programmatic seed renderer through v8: broader teardrop／rounded-base proportions, synchronized eye geometry, layered biological facets, bright side plates that protrude beyond the body, and a restrained soft outer shadow; AI concept art remains reference-only rather than a runtime asset
- Added a plain-language fresh-session handoff that makes the next agent summarize the current state, present three bounded next-step choices, label private／paid consequences, and wait for the user's selection before acting

### Validated

- 41 frontend tests pass, including complete Agent-memory source approval, content-redacted preview, manual sync, missing-source honesty, halo rendering, browser-off behavior, and English／Traditional Chinese parity
- 50 Rust tests pass with one explicitly ignored private-source live smoke, including two-file allowlisting, size／UTF-8／symlink guards, schema-v4 migration, consent binding, automatic-sync state, missing／recovery semantics, forgetting, render-safe DTOs, and ACL／caller denial
- Production TypeScript／Vite build, Rust compile, rustfmt, and Clippy with warnings denied pass
- Local browser smoke observed the primary Agent-memory copy, honest memory-off desktop-runtime boundary, responsive full-page layout, Traditional Chinese switch, local-only／Daily Scout exclusions, and zero browser warnings or errors; it did not read a source or imitate native sync

### Planned

- Separately authorized exact-source private Agent-memory UAT, plus optional private-thread UAT for the supplementary work-record pilot
- Packaged v0.5.0 synthetic compatibility UAT; the verified v0.2.0 installer remains the historical no-redo artifact
- Explicitly authorized paid Daily Scout smoke using only reviewed coarse context, followed by packaged native acceptance
- Monitor official Codex memory storage guidance; any filename, scope, or semantic drift requires adapter versioning and fresh privacy review
- Native reminder delivery with quiet hours and daily budgets
- Remaining packaged pet-shell acceptance for accessibility, 125–200% and mixed DPI, monitor／taskbar topology changes, tray keyboard recovery, and Windows session shutdown

## [0.4.0] - 2026-08-13

> Unreleased, source-only development milestone. It has no accepted v0.4.0 installer, private-thread UAT, or real paid API smoke. Daily Memory Scout is optional and off by default; the verified fixture-only v0.2.0 installer remains the packaged no-redo baseline.

### Added

- Optional bilingual Daily Memory Scout panel that explains its value, keeps the ordinary pet API-free, previews the exact coarse outbound work context, links to official OpenAI key／quickstart pages, and exposes enable, disable, connection test, key deletion, local-history deletion, and full-reset controls
- Proposed ADR-0006 plus a purpose-specific consent contract fixing provider, model, source, allowed categories, context compiler, 3,000-character cap, and automatic once-daily send behavior
- Deterministic Rust context compiler for one approved Codex work record; only allowlisted work domains, public tool／model names, generic goals, non-sensitive constraints, dates, and fixed insight categories can leave the device
- Windows Credential Manager BYOK storage through a Rust-only credential abstraction; the saved key never returns to the WebView or enters SQLite／JSON／localStorage
- Fixed OpenAI Responses API client using pinned `gpt-5.6-terra`, required Web Search, `store: false`, bounded output, timeout handling, provider-error classes, explicit current-update／fallback-tip classification, and annotation-only HTTPS citations
- SQLite migration 0003 and schema v3 settings, daily attempt ledger, cited insight, source-lineage, read, clear, and source-forget invalidation records
- App-running scheduler and transactional one-attempt-per-local-date guard; failures do not automatically retry and missed dates are not replayed
- Neutral `off`／`waiting`／`ready` pet DTO so private insight text and citations stay on the full detail surface

### Security

- Kept synthetic fixtures, raw prompt／answer text, paths, repository URLs, thread and record IDs, arbitrary private phrases, credentials, provider error bodies, and model-written plaintext URLs out of the outbound context and clickable result surface
- Fixed endpoint, Authorization header, model, tool, and instructions in Rust; external webpages are treated as untrusted content and no shell, local file, remote MCP, account login, or external write tool is available
- Extended exact main-only Tauri capabilities and the independent caller-label guard to all ten Daily Scout commands; the pet surface can receive only a neutral readiness state
- Routed official and citation links through a Rust-validated Tauri opener: official URLs are fixed, while a result link must already exist in the local citation table before the system browser can open it
- Preserved the daily attempt ledger when local history or supporting data is removed, preventing deletion from enabling a second paid attempt on the same date

### Validated locally

- 32 frontend tests pass, including consent gating, immediate key-field clearing, browser-off behavior, bilingual context／result controls, annotation-derived links, and neutral pet readiness
- 45 Rust tests pass with one explicitly ignored live catalog test, including context minimization, explicit low-value fallback labeling, Web Search／citation validation, schema migration, daily success and failure idempotency, source invalidation, and exact capability separation
- Production TypeScript／Vite build and Rust compile pass. No real OpenAI key was read, stored, printed, or called during acceptance

## [0.3.0] - 2026-08-12

> Unreleased, source-only development milestone. It has no accepted v0.3.0 installer and no private-thread UAT. The connector is an experimental, exact-version Codex work／thread-history pilot—not durable-memory access or a production connector. The verified v0.2.0 installer remains unchanged and does not need to be rebuilt or re-tested for its already accepted scope.

### Added

- Official-source evaluation recording that OpenAI publishes no stable Codex durable-memory export API or compatibility-guaranteed memory-file schema; direct Codex tool-home parsing is outside the product contract
- Proposed ADR-0005 for an honestly labeled, version-bound Codex work／thread-history pilot through documented App Server `thread/list` and `thread/read` method names over local stdio, while preserving the overall host／transport's experimental and unsupported-for-production status
- Rust-only fixed standard Codex Desktop executable resolution with an exact `codex-cli 0.134.0` fail-closed pin; neither the WebView nor a caller can supply an executable, path, method, or transport
- User-triggered, content-minimized thread listing with short-lived opaque handles and Rust-only raw identifiers, followed by one explicit selected-thread read of the last completed turn's `final_answer`
- Redacted preview that exposes only bounded count／time／exclusion／character-count／hash metadata, explicit completion confirmation, and approval bound to a canonical consent-scope hash
- SQLite migration 0002 and schema v2 `source_consent_scopes`, including deterministic backfill of the known fixture-only v1 store, one-active-source enforcement, and content-free external lineage for the thread pilot
- Transactional local-only forgetting for the selected source, consent scope, event, signal, effect, explanation, render state, cache, and lineage without editing or deleting the original Codex thread
- Synthetic adapter, malformed-response, version-pin, timeout, migration, scope-hash, one-source, redaction, lineage, restart, and forgetting coverage

### Security

- Extended both exact Tauri production capabilities and independent Rust caller-label guards from six to all eight sensitive commands, adding Codex list／preview to the fixture list／preview, cancel, full-state, approve, and forget boundary
- Shared one 10-second deadline across CLI-version verification and each App Server operation, capped stdout／line／final-answer sizes, hidden stderr, and bounded child-process cleanup after timeout or failure
- Kept raw thread IDs, titles, paths, prompts, commentary, reasoning, tool output, and selected final-answer text out of frontend IPC, UI, logs, pet DTOs, native labels, notifications, external lineage, and repository fixtures
- Added no telemetry, cloud sync, remote model call, WebSocket, runtime network request, background monitoring, direct Codex-home scan, or source mutation

### Validated locally

- Synthetic tests exercise the experimental adapter contract, SQLite v1 → v2 migration, canonical consent binding, one-source invariant, redacted boundaries, deterministic completion effect, restart state, lineage, and transactional local forgetting
- A content-free live `thread/list` compatibility smoke passed against the exact pinned local CLI. It did not select any candidate, call `thread/read`, expose thread metadata or content, or constitute private-data UAT
- A native source-tree smoke preserved the pet-first default and opened the detail UI with the experimental work-record boundary labeled separately while durable-memory access remained visibly off
- No private thread was read. Private UAT remains blocked on separate authorization naming the exact completed thread and scope; even a future pass would validate only this pinned experimental pilot

## [0.2.0] - 2026-08-12

> Local unsigned Windows test version; not a signed or public release-ready package.

### Added

- Fixture-only first-memory flow with source selection, exact scope disclosure, preview, record selection, explicit consent, and cancel
- Versioned normalized `completion` event schema and app-local SQLite schema v1 with migration 0001
- Deterministic completion-star derivation with persisted source → event → signal → effect lineage
- “Why did this happen?” inspector plus transactional local-source forgetting and recomputation
- Bilingual desktop UI and automated Rust and React coverage for the supported synthetic path
- Current-user Windows x64 NSIS test-build configuration and `npm run build:windows` command
- English and Traditional Chinese Windows test guides covering installation, fixture use, local data, uninstall limits, and raw-executable sidecars
- Test-build Memoryling icon and in-app brand artwork generated with Codex's built-in ImageGen; PNG transparency was checked for the test-build assets
- Product and technical design draft for deterministic, lineage-aware creature growth with large-form evolution, adjacent-stage EvolutionBridges, reversible time states, local rendering, and no runtime image-generation dependency
- Pet-first two-surface shell with one transparent floating `pet`, one pre-created hidden `main` detail window, and browser routing that does not imitate native pet behavior
- Rust-owned native pet menu, tray recovery, single-instance relaunch, explicit Quit, and close／minimize／restore lifecycle
- Per-window command permissions plus caller-label defense, a content-minimized `CreatureRenderState`, content-free revision events, and pet-denial tests for sensitive memory commands
- Bilingual one-time onboarding, access-off badge, reduced-motion／forced-colors behavior, always-on-top control, and local pet-position recovery state
- User-confirmed pet-first desktop-shell design, proposed ADR-0003, and a staged native acceptance plan
- User-confirmed shared biological／organic and restrained sacred-premium creature language, plus proposed ADR-0004 for deterministic, lineage-aware bounded-variant evolution and recomputation after forgetting
- Five-round PM specification for Agent-memory-linked variants: activity > accumulated journey／outcomes > collaboration, coarse content-domain accents, scoped one-time consent with automatic in-scope local derivation, and outcome-gated durable reshaping after ephemeral recent-use hints
- Codex for Open Source readiness assessment with official-program truth boundaries, a public-beta／maintainer-evidence track, submission gates, and privacy-safe application drafts

### Changed

- Creature marks now depend on persisted Rust state; remaining event and reminder examples are explicitly labeled as concept or planned UI
- Real-memory access remains visibly off during the fixture pilot and in browser preview
- Windows bundling now targets an unsigned bilingual NSIS test installer and can download Microsoft's WebView2 bootstrapper when the prerequisite is missing
- Future creature growth now treats concept forms as visual-vocabulary references, not a fixed route roster: approved durable Agent-activity evidence may deterministically compile into many bounded `MorphologyRecipe` variants, while live Agent presence remains reversible presentation state
- Future permanent morphology may not level from time, tokens, record volume, or one-off recent use; it requires deduplicated, independent, outcome-qualified evidence and must recompute after correction, source disablement, or forgetting
- Package and application versions now identify the pet-first local test line as v0.2.0; real-memory access remains visibly off and no new memory-content network boundary was added

### Validated locally

- The exact 2,875,965-byte `Memoryling_0.2.0_x64-setup.exe` (`SHA-256 BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`) passed an Explorer-launched current-user install into the real per-user LocalAppData location, normal Start menu／desktop shortcut creation, and HKCU uninstall registration reporting version 0.2.0
- Installed-shortcut cold and resident launches remained pet-first and single-instance; right-click → Open showed `main`, closing `main` returned to `pet`, and explicit Quit left no process
- v0.2.0 uninstall with **Delete the application data** clear removed the program, registration, shortcuts, and process while retaining `%LOCALAPPDATA%\app.memoryling.desktop`; only metadata was inspected and no database content was read
- An earlier agent-direct installer launch affected by Windows virtualization was cleaned up and excluded from product evidence
- Historical v0.1.0 two-cycle UAT verified both retained-data and selected-delete-data uninstall behavior; the older selected-delete result is not presented as v0.2.0 evidence
- WebView2-missing UAT remains deferred to a safe disposable Windows environment; code signing, full accessibility／DPI／topology acceptance, real-source access, and public distribution remain incomplete

## [0.1.0] - 2026-08-10

### Added

- Bilingual English and Traditional Chinese concept shell
- Interactive CSS Memoryling creature
- Continuing-event, memory-signal, and bounded-initiative panels
- Explicit prototype and local-first privacy states
- Initial architecture, privacy, roadmap, and contributor documentation
