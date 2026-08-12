# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-12 (Asia/Taipei)

## Current milestone

Deliver the user-confirmed pet-first direction through the proposed two-window shell as a complete synthetic-data vertical slice before widening the product to a real memory source.

For a fresh session, the first executable bundle is the pet-first shell below. The current-host Windows installer gate is closed; the WebView2-missing branch has a concrete unavailable-environment dependency and remains required before public distribution. Do not begin the Codex for Open Source application or real-source work before their preceding gates.

## Overall state

- Stage: v0.1.0 with a complete fixture-only first-memory pipeline and a local Windows x64 NSIS test artifact
- Product surface: bilingual desktop and browser-safe experience implemented locally
- Synthetic Codex-shaped source pipeline: implemented end to end
- User-owned Codex memory access: not implemented
- Local store: SQLite schema v1 under Tauri app-local data
- GitHub repository: public at https://github.com/artpoet/Memoryling
- Default branch: main
- CI: the first-memory implementation and current packaging／icon source bundle both pass local checks and GitHub Actions
- Release: unsigned local Windows x64 test installer produced and current-host native UAT completed; no signed or public packaged release
- Codex for Open Source readiness: product direction is relevant and the repository meets the basic public／maintainer form, but there is no public release, real-source proof, external adoption, or maintainer loop yet; do not submit until the recorded readiness gates are met
- Creature growth design: user-confirmed biological／organic plus restrained sacred-premium direction, bounded variants, scoped automatic derivation, and long-term-versus-recent growth rules recorded; concept forms are reference vocabulary rather than a fixed roster, while the consent scope, classifiers, path profile, morphology-recipe compiler, genome, stages, renderer, and growth journal remain proposed and unimplemented
- Desktop presence design: pet-only default surface with on-demand detail, native recovery paths, and a Rust-owned two-window lifecycle recorded; none of that shell is implemented

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
- recorded the user-confirmed pet-first desktop direction, a detailed implementation contract, and proposed ADR-0003 without presenting the current one-window app as complete
- recorded the user-confirmed high-level visual and many-variant direction, plus a proposed weighted-profile model and ADR-0004, without presenting the concept forms as fixed routes, production assets, or live personalization
- completed a five-round PM interview for Agent-memory-linked variation: `AgentActivityPattern` has priority over accumulated journey and collaboration style; content domain is a secondary visual layer; a future real source uses one scoped source／data-category／purpose consent followed by automatic in-scope local derivation; recent use stays ephemeral until multiple independent, outcome-qualified evidence groups pass a versioned durable-growth gate
- completed current-user native installer UAT through fixture approval, restart persistence, lineage, forgetting, and restart without ghost state
- verified both uninstall data choices: unchecked retained the pre-existing app-data tree; checked in the second same-artifact cycle removed it completely
- recorded the unavailable WebView2-missing environment and fixed v0.1.0's distribution decision as unsigned, local-test-only, and not a public release

## Product truth

The local pipeline is real and persistent, but its only permitted input is a fictional resource bundled with the app. Memoryling does not scan or read the user's Codex tool-home, arbitrary files, credentials, prompts, or private memory. Browser preview keeps memory access off; after fixture approval the desktop still says real memory access is off.

The user-confirmed future growth direction allows many bounded variants within one living family, rather than a fixed pre-authored pet roster. The current concept forms only test family grammar, silhouette range, material language, and adjacent transitions. The information priority is observable Agent activity first, accumulated journey／outcomes second, and collaboration style third: activity alone controls the main silhouette and motion; journey gates permanent reshaping and controls maturity／milestones without selecting a silhouette; collaboration can add only a bounded local rhythm. Coarse content domain is a second layer limited to material, surface, or pattern accents and cannot select a species or sensitive personality label. A future real connector uses one narrow `SourceConsentScope` for one exact source, data categories, purposes, adapter version, and rule versions; new in-scope records may then be derived locally without per-record prompts. Another source needs another scope, while a category／purpose expansion needs a new consent revision. Recent use affects only expiring render hints. Permanent reshaping requires multiple deduplicated, independent, outcome-qualified evidence groups; calendar time, record volume, tokens, Agent count, and app-open time are not XP. The resulting lineage-backed profile compiles with identity, stage, and a finite module catalog into a deterministic `MorphologyRecipe`; correction, source disablement, or forgetting recomputes it. No Agent monitoring exists. Distant forms may differ greatly, but every adjacent change retains a versioned evolution bridge. The exact scope schema, evidence mapping, thresholds, module catalog, renderer, and visual limits remain proposed and unimplemented; this is not a claim about the current CSS creature, completion-star fixture, or exploratory ImageGen concepts.

The user-confirmed desktop direction is now pet-first: normal presence is one transparent floating creature; right-click is the primary entry to an on-demand standard detail window, with `Win+B` tray, Start Menu, and packaged installed-shortcut recovery. The proposed implementation uses pre-created `pet`／`main` windows, Rust-owned lifecycle, per-window app-command permissions, and a content-minimized render-state boundary. The current packaged app still opens one 1180 × 780 standard window and has no floating pet, tray, or single-instance two-window lifecycle.

## Windows x64 test-build status

- the supported tester entry is the current-user NSIS installer `Memoryling_0.1.0_x64-setup.exe`
- the local artifact exists at `src-tauri/target/release/bundle/nsis/Memoryling_0.1.0_x64-setup.exe`; build output remains untracked
- `npm run build:windows` is the documented developer build command
- the installer is bilingual and is configured to download Microsoft's WebView2 bootstrapper when WebView2 is missing; that conditional path is not yet UAT-verified and is separate from the network-free fixture memory path
- the raw `src-tauri/target/release/memoryling.exe` depends on `src-tauri/target/release/fixtures/codex-first-memory-v1.json` and is not a portable distribution
- app state is under `%LOCALAPPDATA%\app.memoryling.desktop`; uninstall can retain it unless the delete-app-data option is explicitly selected
- the new icon and in-app brand asset were generated with Codex's built-in ImageGen; alpha-channel and transparent-pixel checks passed for the source and generated PNG path
- current-host native UAT passed install → Start Menu launch → fixture preview／approve → restart persistence → lineage → forget → restart without ghost state, followed by both uninstall choices
- the installer is unsigned and may show Unknown publisher or SmartScreen warnings; v0.1.0 is fixed as a local test artifact and is not public release-ready
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
- PASS — checksum-matched packaged app completed the native fixture tour, restored its persisted mark after Start Menu relaunch, exposed all four lineage stages, forgot the supported graph, and relaunched with no ghost mark or lineage
- PASS — retention uninstall removed the process, install directory, shortcut, and registration while preserving the pre-existing app-data tree and its two direct children; no database content was read
- PASS — the same artifact reinstalled and opened in the expected real-memory-off／no-approved-source state, then explicit delete-app-data uninstall removed the process, install directory, both shortcuts, registration, and entire app-data tree; post-check completed at 2026-08-12T17:17:31+08:00
- DEFERRED — WebView2-missing bootstrapper UAT requires a disposable clean Windows x64 environment; this host has WebView2 151.0.4129.78 and no Windows Sandbox／Hyper-V test environment, so the installed host runtime was not removed

The UAT used the packaged installed app and real NSIS uninstaller, not a browser mock. The user completed the initial installer confirmation and the final explicit-delete action; agent-assisted native UI covered the fixture, restart, lineage, forget, retention, and second-install path. No database contents or UAT screenshots were collected or added to the repository.

Remote evidence:

- repository visibility is PUBLIC and default branch is main
- GitHub recognizes the MIT license
- discovery topics include agent-memory, desktop-pet, local-first, Tauri, React, and TypeScript
- private vulnerability reporting is enabled
- implementation commit: [`979bf7e`](https://github.com/artpoet/Memoryling/commit/979bf7eb19f31b1b3931b4c8824df1d94689408d)
- PASS — Windows CI for that implementation commit: https://github.com/artpoet/Memoryling/actions/runs/31380474307
- packaging/icon commit: [`2aead61`](https://github.com/artpoet/Memoryling/commit/2aead6133d31578239ea49e04c9a95509c05911a)
- PASS — Windows CI for the packaging/icon source bundle: https://github.com/artpoet/Memoryling/actions/runs/31394540587

## Codex for Open Source readiness

Memoryling can truthfully apply as a public MIT project maintained by `artpoet`, and its source-traceable／reversible Agent-memory architecture is relevant to the Codex ecosystem. It is not yet a competitive maintainer-program application: the public repository was created on 2026-08-10 and remains a young one-contributor project with no public Release or demonstrated external maintainer loop. The implemented product remains synthetic fixture-only.

The internal decision is **do not submit yet**. Finish the pet shell → one real source sequence, close the deferred release-environment checks, publish an honest beta and demo, then demonstrate genuine external testing plus at least one public feedback／issue → fix → follow-up release maintainer loop. The complete program facts, dynamic evidence warning, readiness gates, application drafts, and API-credit privacy boundary live in [the Codex for Open Source readiness plan](docs/research/2026-08-12_codex-for-open-source-readiness.md). Suggested tester counts are internal credibility targets, not official OpenAI thresholds.

## Known gaps

- the scaffold icons were replaced with generated test artwork, but the new artwork has not received public-release signoff
- accessibility requires a dedicated keyboard and screen-reader audit
- the adapter supports only the bundled synthetic v1 fixture; no user-owned Codex memory format is accepted
- a Rust-owned native picker, strict external-file validation, and preview redaction remain future work
- conversation model strategy remains intentionally open
- the evolving-creature system has product drafts and proposed ADR-0004, but its `SourceConsentScope`, in-scope automatic classifier, recent-hint TTL, outcome-qualified durable-growth gates, final visual asset, path-profile mapping, lineage-bearing bounded `MorphologyRecipe` catalog and compatibility matrix, genome, multi-source growth graph, renderer, privacy mode, and accessibility acceptance remain unimplemented
- no process, session, or Agent-presence monitoring exists; any future ephemeral presence adapter requires a separate product／privacy decision and explicit consent
- the pet-first shell is design-only: transparent pet window, hidden detail window, native context menu, tray, single-instance recovery, safe position restore, and narrow render DTO remain unimplemented
- current custom memory commands are not yet permission-separated by window; a future pet surface must be denied list／preview／cancel／full-state／approve／forget commands, not merely avoid calling them in normal UI
- WebView2-missing bootstrapper behavior still needs UAT in a disposable clean Windows x64 environment; this host has WebView2 151.0.4129.78 and no Windows Sandbox／Hyper-V environment, and its runtime must not be removed for testing
- code signing and public distribution remain incomplete; v0.1.0 is intentionally unsigned and local-test-only, and the recorded checksum changes after any rebuild

## Next bundle

Implement the proposed pet-first shell against synthetic state as one vertical slice: `pet`／`main` surfaces, native menu and tray, single-instance recovery, generated per-window app-command permissions plus pet-denial tests, close／minimize／restore behavior, position／DPI handling, narrow render DTO, approve／forget synchronization, bilingual accessibility, and packaged desktop smoke. The current v0.1.0 standard window is the old fixture shell, not the confirmed pet-first experience. Only after the pet shell is proven should the first user-selected Codex-source pilot resume; do not treat private `MEMORY.md` files as a public format specification.

The future Phase 2 bounded-variant growth direction is recorded in `docs/drafts/deep-interview-evolving-creature-system-2026-08-11.md`, `docs/drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md`, and proposed ADR-0004; it does not supersede the installer, pet-shell, and real-source gates above.

After one real-source slice is proven, prepare the public-beta and maintainer-evidence track: README hero and short synthetic demo, verified GitHub Release and checksums, real external testers, honest adoption metrics, and at least one feedback／issue → fix → follow-up release loop. Refresh official terms and live GitHub evidence only when those gates are ready, then complete the application from the recorded drafts. Do not submit the current pre-release fixture-only state.

## Do not redo

- do not replace the Tauri + React foundation without new evidence
- do not rebuild the completed fixture → pending preview → SQLite → lineage → recompute path
- do not distribute the raw release executable as a portable app without its generated fixture sidecar
- do not describe the unsigned NSIS test artifact or generated test icon as public release-ready
- do not recommend bypassing SmartScreen or weakening Windows security controls
- do not add open-ended AI chat before the memory lineage path exists
- do not describe the fixture pilot as access to the user's real Codex memories
- do not add cloud sync, telemetry, or remote memory processing by assumption
- do not implement the floating pet as a second independent creature state, expose full memory text to its surface, or treat a browser mock as native two-window verification
- do not collapse growth into one fixed evolution line, classify raw text into a personality, or let runtime AI／unsaved randomness choose a permanent route
- do not turn the reference forms into a fixed sprite roster or let live Agent presence silently accumulate into permanent morphology
- do not treat one-time consent as permission to scan new locations or add new data categories／purposes; do not promote ephemeral hints or raw usage volume into permanent growth
- do not describe Codex for Open Source as a contest, guaranteed `$1,200`, cash award, or automatic benefit for any public repository
- do not invent or inflate stars, downloads, testers, issues, pull requests, releases, testimonials, adoption, or maintainer work
- do not add runtime cloud AI or upload private memories merely to strengthen the application or request API credits
- do not submit the application before the readiness gates are met unless the user explicitly changes that decision after reviewing current evidence
