# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-11 (Asia/Taipei)

## Current milestone

Provide an honest Windows x64 current-user test installer for the completed fixture-only memory slice, then close its remaining installer UAT and artifact-evidence gaps.

## Overall state

- Stage: v0.1.0 with a complete fixture-only first-memory pipeline and a local Windows x64 NSIS test artifact
- Product surface: bilingual desktop and browser-safe experience implemented locally
- Synthetic Codex-shaped source pipeline: implemented end to end
- User-owned Codex memory access: not implemented
- Local store: SQLite schema v1 under Tauri app-local data
- GitHub repository: public at https://github.com/artpoet/Memoryling
- Default branch: main
- CI: the first-memory implementation and current packaging／icon source bundle both pass local checks and GitHub Actions
- Release: unsigned local Windows x64 test installer produced; no signed or public packaged release
- Creature growth design: deterministic large-form evolution direction recorded; genome, stages, renderer, and growth journal are not implemented

## Completed in this bundle

- added a strict Codex-shaped adapter v1 for one fictional bundled JSON fixture
- added explicit fixture selection, exact scope disclosure, preview, record selection, cancel, and consent
- bound approval to an in-memory pending-preview token instead of trusting content from the WebView
- added normalized memory-event schema v1 and SQLite migration 0001
- stored adapter version, source and record hashes, timestamps, normalized text, and machine-readable lineage
- added one deterministic completion signal and one persisted completion-star world effect
- added a “Why did this happen?” chain from source record through event and signal to mark
- implemented transactional local-source forgetting, derived-state cleanup, and deterministic recomputation
- made the creature mark conditional on persisted Rust state; removed fake always-visible marks
- relabeled the remaining sample event and initiative UI as CONCEPT／PLANNED in both languages
- added Vitest／Testing Library UI coverage and Rust adapter／migration／persistence／forgetting tests
- kept Tauri capabilities and CSP unchanged; added no telemetry, network client, or arbitrary filesystem permission

## Product truth

The local pipeline is real and persistent, but its only permitted input is a fictional resource bundled with the app. Memoryling does not scan or read the user's Codex tool-home, arbitrary files, credentials, prompts, or private memory. Browser preview keeps memory access off; after fixture approval the desktop still says real memory access is off.

The user-confirmed future growth direction allows the creature to evolve substantially. Distant stages do not have to remain immediately recognizable as the same form, but every adjacent transition must retain an understandable, versioned evolution bridge. Permanent growth will come only from approved memory-derived data; calendar time will affect reversible presentation state only. Changes will apply automatically after source approval but remain lineage-explainable and recomputable. The detailed stages, bridge grammar, schema, renderer, and visual-slot limits remain proposed. This is documented design intent, not a claim about the current CSS creature or completion-star fixture.

## Windows x64 test-build status

- the supported tester entry is the current-user NSIS installer `Memoryling_0.1.0_x64-setup.exe`
- the local artifact exists at `src-tauri/target/release/bundle/nsis/Memoryling_0.1.0_x64-setup.exe`; build output remains untracked
- `npm run build:windows` is the documented developer build command
- the installer is bilingual and can download Microsoft's WebView2 bootstrapper when WebView2 is missing; that prerequisite download is separate from the network-free fixture memory path
- the raw `src-tauri/target/release/memoryling.exe` depends on `src-tauri/target/release/fixtures/codex-first-memory-v1.json` and is not a portable distribution
- app state is under `%LOCALAPPDATA%\app.memoryling.desktop`; uninstall can retain it unless the delete-app-data option is explicitly selected
- the new icon and in-app brand asset were generated with Codex's built-in ImageGen; alpha-channel and transparent-pixel checks passed for the source and generated PNG path
- installer generation and configuration inspection passed, but full human install → open → fixture flow → uninstall click-through, including the delete-app-data option, is not yet claimed as passed UAT
- the installer is unsigned and may show Unknown publisher or SmartScreen warnings; it is not public release-ready
- the finalized local installer is 2,759,655 bytes with SHA-256 `62FE4E5D87E4F221174F120F84A94303345C3694CA57090353438037F271D79B`; regenerate the checksum after any rebuild
- packaging/icon source commit `2aead61` passed fresh Windows CI run `31394540587`

## Verification evidence

- PASS — 5 frontend interaction tests covering off → preview → approve → mark → lineage → forget, restart restore, bilingual browser boundary, invalid-timestamp rendering defense, and failed-forget state integrity
- PASS — 7 Rust tests covering strict parsing and RFC 3339 validation, unknown-version fail-closed, preview zero-persistence, pending-token binding and failed-approval retry, migration and future-schema rejection, restart persistence, idempotent approval, lineage, fixture immutability, and complete supported-chain forgetting
- PASS — npm run check (frontend tests, TypeScript, Vite production build, Rust tests, and cargo check)
- PASS — cargo fmt --manifest-path src-tauri/Cargo.toml --check
- PASS — cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
- PASS — npm audit reported 0 vulnerabilities
- PASS — Tauri desktop launched and exposed the bundled source, exact access scope, and empty app-local store through its native accessibility tree
- PASS — app-local SQLite was created outside the repository
- PASS — the bundled runtime resource exists and its SHA-256 matches the committed synthetic fixture
- PASS — English and Traditional Chinese browser smoke at desktop width plus a 390 × 844 viewport, with no horizontal overflow
- PASS — browser console contained no errors or warnings and the page linked no external runtime resources
- PASS — restrictive CSP and `core:default` capability remain unchanged
- PASS — `npm run build:windows` produced the Windows x64 current-user NSIS installer and bundled `memoryling.exe` plus the synthetic fixture resource
- PASS — raw release-app smoke opened the bilingual Tauri window; both the Windows title bar and in-app header showed the generated Memoryling icon
- PASS — installer-launch smoke reached the Traditional Chinese `Memoryling 安裝` window with the generated icon, then closed without installing
- PASS — generated ICO contains 16／24／32／48／64／256 px layers; the 1,254 × 1,254 RGBA source has transparent pixels

The Windows desktop-control helper could read the native UI but failed at input injection after recovery. Native click-through is therefore supported by Rust product-path tests plus frontend interaction tests, not claimed as completed human UAT.

Remote evidence:

- repository visibility is PUBLIC and default branch is main
- GitHub recognizes the MIT license
- discovery topics include agent-memory, desktop-pet, local-first, Tauri, React, and TypeScript
- private vulnerability reporting is enabled
- implementation commit: [`979bf7e`](https://github.com/artpoet/Memoryling/commit/979bf7eb19f31b1b3931b4c8824df1d94689408d)
- PASS — Windows CI for that implementation commit: https://github.com/artpoet/Memoryling/actions/runs/31380474307
- packaging/icon commit: [`2aead61`](https://github.com/artpoet/Memoryling/commit/2aead6133d31578239ea49e04c9a95509c05911a)
- PASS — Windows CI for the packaging/icon source bundle: https://github.com/artpoet/Memoryling/actions/runs/31394540587

## Known gaps

- the scaffold icons were replaced with generated test artwork, but the new artwork has not received public-release signoff
- accessibility requires a dedicated keyboard and screen-reader audit
- the adapter supports only the bundled synthetic v1 fixture; no user-owned Codex memory format is accepted
- a Rust-owned native picker, strict external-file validation, and preview redaction remain future work
- conversation model strategy remains intentionally open
- the evolving-creature system has a product and architecture draft, but its identity core, genome schema, multi-source growth graph, evolution renderer, privacy mode, and accessibility acceptance remain unimplemented
- Windows installer and uninstaller click-through, WebView2-missing behavior, and app-data deletion still need human UAT
- code signing and public distribution remain incomplete; the recorded local checksum changes after any rebuild

## Next bundle

Close the Windows x64 test-build gate before returning to a real-source pilot: complete current-user install, launch, fixture tour, uninstall, and app-data deletion UAT; recheck the recorded installer checksum after any rebuild; then decide signing and distribution readiness. After that gate, prepare the first user-selected Codex-source pilot described in AI-WAKEUP.md without treating private `MEMORY.md` files as a public format specification.

The future Phase 2 growth direction is recorded in `docs/drafts/deep-interview-evolving-creature-system-2026-08-11.md`; it does not supersede the installer and real-source gates above.

## Do not redo

- do not replace the Tauri + React foundation without new evidence
- do not rebuild the completed fixture → pending preview → SQLite → lineage → recompute path
- do not distribute the raw release executable as a portable app without its generated fixture sidecar
- do not describe the unsigned NSIS test artifact or generated test icon as public release-ready
- do not recommend bypassing SmartScreen or weakening Windows security controls
- do not add open-ended AI chat before the memory lineage path exists
- do not describe the fixture pilot as access to the user's real Codex memories
- do not add cloud sync, telemetry, or remote memory processing by assumption
