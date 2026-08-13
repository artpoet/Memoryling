# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-13 (Asia/Taipei)

## Current milestone

The v0.4.0 source tree adds the optional BYOK **Daily Memory Scout** to the version-bound experimental Codex work-record pilot and verified v0.2.0 pet-first baseline. When explicitly enabled, it sends only a visible allowlisted coarse work summary to fixed OpenAI Responses／Web Search, reserves at most one attempt per local date, and returns a short cited pet message. The ordinary pet remains API-free and durable-memory access remains off. No real API key, paid request, private-record UAT, or packaged v0.4.0 acceptance is claimed.

Extended pet-shell environment acceptance and the WebView2-missing branch remain required before public distribution, but they do not justify rebuilding the completed shell. Do not begin the Codex for Open Source application before the recorded product, release, adoption, and maintainer-loop gates.

## Overall state

- Stage: v0.4.0 source-only Daily Memory Scout on the experimental work-record pilot; v0.2.0 remains the last installed-UAT artifact
- Product surface: bilingual transparent pet plus on-demand detail surface; browser stays an honest detail-only preview
- Synthetic Codex-shaped source pipeline: implemented end to end
- Codex work-record pilot: implemented in source for explicit one-record selection; private-content UAT not performed
- Daily Memory Scout: implemented in source, off by default; synthetic provider／citation／once-per-day proof passes, real paid smoke not authorized or performed
- Codex durable-memory access: not implemented; no supported export API or stable third-party file contract is available
- Local store: SQLite schema v3 under Tauri app-local data, including import consent plus Daily Scout settings, attempt ledger, cited insight, and source lineage; API key is separate in Windows Credential Manager
- GitHub repository: public at https://github.com/artpoet/Memoryling
- Default branch: main
- CI: historical first-memory／packaging, v0.2.0 pet-first, v0.3.0 work-record, and v0.4.0 Daily Scout bundles pass GitHub Actions
- Release: unsigned v0.2.0 Windows x64 current-user installer passed installed UAT; v0.4.0 has no packaged artifact or public release
- Codex for Open Source readiness: product direction is relevant and the repository meets the basic public／maintainer form, but there is no public release, real-source proof, external adoption, or maintainer loop yet; do not submit until the recorded readiness gates are met
- Creature visual baseline: the accepted violet egg-shaped `seed`／`memory-seed-egg-v1` body now uses the `procedural-svg-v2` layered runtime renderer, with the accepted concept's juvenile face proportions, original-size synchronized eyes with restored breathing room, prominent memory core, and cheek-hugging side plates; the required access-off truth remains quiet inline text, while AI concept art and PNG icons are reference／packaging material only and later stages remain reference-only
- First-run creation: a bilingual native first-launch flow now asks only for language and whether to keep the local pet or securely prepare an OpenAI key for Daily Scout; local-only is the recommended default, no content is transmitted, and Daily Scout remains disabled until its later context review and purpose-specific consent
- Creature growth design: user-confirmed biological／organic plus restrained sacred-premium direction, bounded variants, scoped automatic derivation, and long-term-versus-recent growth rules recorded; concept forms are reference vocabulary rather than a fixed roster, while classifiers, path profile, morphology-recipe compiler, later-stage renderer, genome, stages, and growth journal remain proposed and unimplemented
- Desktop presence: transparent pet-only default with on-demand detail, native menu／tray／single-instance recovery, content-free position settings, and a Rust-owned two-window lifecycle implemented; extended multi-DPI／accessibility／shutdown acceptance remains open

## Completed vertical slices

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
- preserved the restrictive CSP, telemetry-free design, and lack of arbitrary filesystem permission
- recorded the user-confirmed pet-first desktop direction, a detailed implementation contract, and proposed ADR-0003 without presenting the then-current one-window app as complete
- recorded the user-confirmed high-level visual and many-variant direction, plus a proposed weighted-profile model and ADR-0004, without presenting the concept forms as fixed routes, production assets, or live personalization
- completed a five-round PM interview for Agent-memory-linked variation: `AgentActivityPattern` has priority over accumulated journey and collaboration style; content domain is a secondary visual layer; a future real source uses one scoped source／data-category／purpose consent followed by automatic in-scope local derivation; recent use stays ephemeral until multiple independent, outcome-qualified evidence groups pass a versioned durable-growth gate
- completed historical v0.1.0 current-user native installer UAT through fixture approval, restart persistence, lineage, forgetting, and restart without ghost state
- verified both historical v0.1.0 uninstall data choices: unchecked retained the pre-existing app-data tree; checked in the second same-artifact cycle removed it completely
- recorded the unavailable WebView2-missing environment and fixed v0.1.0's distribution decision as unsigned, local-test-only, and not a public release
- implemented pre-created `pet`／`main` windows, transparent tight pet bounds, bilingual one-time onboarding, native right-click／keyboard menu, tray recovery, single-instance behavior, and explicit Quit
- made Rust own close／minimize／restore／hide transitions with tested rollback, TrayOnly handling, and content-free atomic shell settings／backup
- added normalized work-area position persistence, bounds clamping, move debounce, topology polling, and automated 100–200%／negative-monitor geometry coverage
- replaced shared default capabilities with exact `main`／`pet` capabilities, generated app-command permissions, and independent caller-label guards
- proved both security layers by invoking all eight sensitive memory commands from `pet`: production ACL and caller defense each reject before handler entry; `main` has a positive invoke path
- added the whitelisted `CreatureRenderState`, opaque deterministic revisions, approve／forget refresh events, race-safe subscription, and a pet code split that does not load full-memory APIs
- fixed concurrent first-open SQLite migration by serializing version check, migration, and commit under one immediate transaction
- completed raw-native fixture approve／restart／lineage／forget UAT plus a normal Explorer-launched v0.2.0 current-user install, installed-shortcut cold／resident single-instance recovery, pet↔detail smoke, explicit Quit, and retain-data uninstall
- documented that Codex durable memories have no stable public export API／schema and selected a separately labeled, version-bound thread-history pilot in proposed ADR-0005
- implemented a Rust-owned local App Server client pinned to `codex-cli 0.134.0`, one shared 10-second operation deadline, bounded fail-closed cleanup, and no-console／bounded-output process handling
- added user-triggered content-minimized `thread/list`, Rust-only raw IDs, atomic one-catalog consumption, and one explicitly selected `thread/read` that accepts only the last completed turn's final answer
- kept selected content out of frontend preview／logs, bound approval to a canonical consent-scope hash, enforced one active source, stored external consent in SQLite schema v2, and kept external lineage content-free
- guarded late async results, concurrent source approval／forgetting, catalog／preview expiry, and detail reset; extended dual-layer pet denial tests from six to eight sensitive commands
- recorded proposed ADR-0006 and a visible purpose-specific consent contract for optional daily OpenAI transmission without changing the API-free ordinary pet
- added a deterministic allowlist compiler that reads only an approved work record and emits coarse work domains, public tools／models, generic goals, non-sensitive constraints, dates, and fixed categories—never source prose, paths, IDs, credentials, or synthetic fixtures
- stored BYOK only through Windows Credential Manager; Rust fixes the OpenAI endpoint, pinned `gpt-5.6-terra`, `store: false`, required Web Search, limits, timeout, explicit current-update／fallback-tip classification, and annotation-only HTTPS citation validation
- added SQLite migration 0003, transactional one-attempt-per-local-date reservation, app-running schedule, honest no-retry failure, source-linked insight deletion, and separate disable／clear-history／delete-key／reset controls
- added a compact bilingual Daily Scout setup／insight panel, official API links, immediate key-field clearing, and a neutral pet-only ready state; all ten new commands are main-only under both ACL and caller-label guards

## Product truth

The fixture pipeline remains real and persistent. The v0.4.0 source retains the experimental work-record path: only after the user asks to browse does Rust list neutral, content-minimized candidates; only one explicitly selected completed record may be read; its content remains Rust-only until redacted preview and exact consent; and only Memoryling's local approved copy is forgotten. The connector does not scan tool-home／arbitrary files, parse `~/.codex/memories`, write to Codex, or call a model.

Daily Memory Scout is the one separately labeled network exception. It is off by default and unavailable to the synthetic fixture. After key save, outbound-context review, cost／retention disclosure, and explicit consent, Rust may automatically make one OpenAI Web Search attempt per local date while the app runs. It sends no source prose; only the exact coarse summary displayed in settings leaves the device. The WebView cannot control endpoint, key, model, tools, or instructions. Full insight and citations stay in the detail surface; the pet sees only a neutral ready bit. Browser mode makes no request. Durable／real-memory access remains visibly off.

The user-confirmed future growth direction allows many bounded variants within one living family, rather than a fixed pre-authored pet roster. The current concept forms only test family grammar, silhouette range, material language, and adjacent transitions. The information priority is observable Agent activity first, accumulated journey／outcomes second, and collaboration style third: activity alone controls the main silhouette and motion; journey gates permanent reshaping and controls maturity／milestones without selecting a silhouette; collaboration can add only a bounded local rhythm. Coarse content domain is a second layer limited to material, surface, or pattern accents and cannot select a species or sensitive personality label. A future real connector uses one narrow `SourceConsentScope` for one exact source, data categories, purposes, adapter version, and rule versions; new in-scope records may then be derived locally without per-record prompts. Another source needs another scope, while a category／purpose expansion needs a new consent revision. Recent use affects only expiring render hints. Permanent reshaping requires multiple deduplicated, independent, outcome-qualified evidence groups; calendar time, record volume, tokens, Agent count, and app-open time are not XP. The resulting lineage-backed profile compiles with identity, stage, and a finite module catalog into a deterministic `MorphologyRecipe`; correction, source disablement, or forgetting recomputes it. No Agent monitoring exists. Distant forms may differ greatly, but every adjacent change retains a versioned evolution bridge. The exact scope schema, evidence mapping, thresholds, module catalog, renderer, and visual limits remain proposed and unimplemented; this is not a claim about the current CSS creature, completion-star fixture, or exploratory ImageGen concepts.

The desktop is now pet-first: normal presence is one transparent floating creature; right-click or the keyboard context-menu path opens the standard detail window. Rust owns the pre-created `pet`／`main` lifecycle, native menu／tray, installed-shortcut single-instance recovery, saved bounds, and explicit Quit. The pet is capability-separated and receives only a content-minimized render DTO plus opaque revision events. Browser mode cannot claim native labels or persistence, so it intentionally renders the honest detail preview. Extended live DPI／monitor／hitbox／assistive-tech／shutdown checks remain acceptance work; ADR-0003 therefore stays Proposed.

## Windows x64 test-build status

- the supported tester entry is the current-user NSIS installer `Memoryling_0.2.0_x64-setup.exe`
- the local artifact exists at `src-tauri/target/release/bundle/nsis/Memoryling_0.2.0_x64-setup.exe`; build output remains untracked
- `npm run build:windows` is the documented developer build command
- the installer is bilingual and is configured to download Microsoft's WebView2 bootstrapper when WebView2 is missing; that conditional path is not yet UAT-verified and is separate from the network-free fixture memory path
- the raw `src-tauri/target/release/memoryling.exe` depends on `src-tauri/target/release/fixtures/codex-first-memory-v1.json` and is not a portable distribution
- app state is under `%LOCALAPPDATA%\app.memoryling.desktop`; uninstall can retain it unless the delete-app-data option is explicitly selected
- the new icon and in-app brand asset were generated with Codex's built-in ImageGen; alpha-channel and transparent-pixel checks passed for the source and generated PNG path
- raw-native UAT passed pet onboarding／menu／lifecycle, fixture preview／approve／restart／lineage／forget, cross-surface refresh, single-instance recovery, explicit Quit, and a 100% second-monitor smoke
- normal Explorer-launched current-user installation created the real LocalAppData program, HKCU registration, Start Menu and desktop shortcuts; installed cold and resident launches both recovered one process and the pet-first lifecycle
- unchecked v0.2.0 uninstall removed the process, program, registration, and shortcuts while retaining `%LOCALAPPDATA%\app.memoryling.desktop`; only metadata was inspected and no database content was read
- a direct packaged-agent installer launch was rejected as evidence because Windows redirected it into the agent package's virtualized LocalAppData; its virtualized program／shortcut were removed before the normal run
- v0.1.0 remains a no-redo historical baseline for the full fixture tour and both uninstall data choices; the v0.2.0 run did not repeat checked-delete
- the installer is unsigned and may show Unknown publisher or SmartScreen warnings; v0.2.0 is local-test-only and not public release-ready
- the finalized v0.2.0 installer is 2,875,965 bytes with SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`; regenerate the checksum after any rebuild
- historical v0.1.0 packaging/icon commit `2aead61` passed Windows CI run `31394540587`

## Verification evidence

- PASS — 32 frontend tests cover native-label routing, browser honesty, fixture／work-record flows, Daily Scout consent gating, immediate key-field clearing, citation-only links, neutral pet readiness, reset races, and bilingual boundaries
- PASS — 45 Rust tests cover fixture／work-record parsing, process bounds, consent migration, one-source enforcement, persistence／recomputation, coarse-context minimization, explicit low-value fallback labeling, OpenAI response／citation rejection, one-attempt success／failure idempotency, source invalidation, ACL／caller guards, and pet-shell lifecycle／geometry; one private-read live smoke stays explicitly ignored
- PASS — production capabilities and the independent caller-label guard deny the full sensitive manifest from `pet`, including all ten Daily Scout settings／credential／network／external-link／full-insight commands; `main` retains a positive invoke path
- PASS — `npm run check`, `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, and `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
- PASS — Windows CI for source v0.4.0 implementation commit `c3b59f8`: https://github.com/artpoet/Memoryling/actions/runs/31664739429
- PASS — the pet entry bundle excludes full-memory client identifiers and detail UI; browser mode renders only the honest detail preview
- PASS — restrictive CSP remains; broad `core:default` was replaced by exact `main`／`pet` capabilities, and the pet has no cross-window mutation permission
- PASS — exact local `codex-cli 0.134.0` content-minimized live `thread/list` smoke returned only neutral candidates and left no Codex child process; it did not invoke `thread/read` or output IDs, titles, paths, summaries, or content
- PASS — source v0.4.0 native desktop smoke observed the transparent pet, native Open／close／Quit lifecycle, English／Traditional Chinese Daily Scout value copy, OFF／ordinary-pet-no-API state, key／official links, synthetic-ineligible context, daily timing, consent／cost／retention copy, and disabled enable control; no key was entered, no source was browsed, and no request was made
- PENDING — one explicitly authorized paid Daily Scout smoke and packaged v0.4.0 native acceptance; synthetic tests do not substitute for provider-account or paid behavior
- PENDING — one exact user-authorized private work-record preview／consent／approve／restart／redacted-lineage／forget UAT; no private record has been selected or read
- PASS — `npm run build:windows` produced the v0.2.0 NSIS installer, release executable, and synthetic fixture resource
- PASS — raw release UAT observed transparent pet-only launch, 360 × 430 Traditional Chinese onboarding → 320 × 320 dismissed bounds, native right-click and Shift+F10 menus, pet↔detail close／minimize lifecycle, explicit native Quit, and one-process relaunch
- PASS — the raw fixture tour observed preview／consent／approve, persisted completion star after restart, all four lineage stages, forget, restart without ghost state, and approve／forget synchronization across surfaces
- PASS — raw pet movement and a second-monitor 100% smoke were observed; precise drag bounds and the remaining live DPI／topology matrix stay open
- PASS — normal Explorer-launched v0.2.0 current-user install created the real program, registration, and shortcuts; installed Start Menu cold and resident launches both recovered one pet-first process
- PASS — installed right-click Open, main close → pet, explicit Quit, and process-absence checks
- PASS — unchecked v0.2.0 uninstall removed process, program directory, registration, and both shortcuts while preserving the app-data tree; only metadata was inspected and no database content was read
- PASS — rejected virtualized harness attempt and its virtual program／shortcut were removed; it is not counted as product installation evidence
- HISTORICAL PASS — checksum-matched v0.1.0 UAT covered the full fixture tour plus unchecked-retain and checked-delete uninstall choices; it remains a no-redo baseline, not substituted v0.2.0 evidence
- PENDING — live 125／150／175／200% and mixed-DPI, monitor hot-unplug／taskbar relocation, adjacent-desktop hitbox, Win+B tray keyboard access, Narrator／NVDA, and sign-out／shutdown acceptance
- DEFERRED — WebView2-missing bootstrapper UAT requires a disposable clean Windows x64 environment; this host has WebView2 151.0.4129.78 and no Windows Sandbox／Hyper-V test environment, so the host runtime was not removed

The v0.2.0 installed UAT used the real normally installed NSIS artifact and uninstaller, not a browser mock or the rejected virtualized attempt. No database contents, private memories, or UAT screenshots were collected or added to the repository.

Remote evidence:

- repository visibility is PUBLIC and default branch is main
- GitHub recognizes the MIT license
- discovery topics include agent-memory, desktop-pet, local-first, Tauri, React, and TypeScript
- private vulnerability reporting is enabled
- implementation commit: [`979bf7e`](https://github.com/artpoet/Memoryling/commit/979bf7eb19f31b1b3931b4c8824df1d94689408d)
- PASS — Windows CI for that implementation commit: https://github.com/artpoet/Memoryling/actions/runs/31380474307
- packaging/icon commit: [`2aead61`](https://github.com/artpoet/Memoryling/commit/2aead6133d31578239ea49e04c9a95509c05911a)
- PASS — Windows CI for the packaging/icon source bundle: https://github.com/artpoet/Memoryling/actions/runs/31394540587
- pet-first implementation commit: [`f48ec6f`](https://github.com/artpoet/Memoryling/commit/f48ec6fab2fd8ca26ab369d5972f4ae6d68b4075)
- PASS — Windows CI for the v0.2.0 implementation bundle: https://github.com/artpoet/Memoryling/actions/runs/31603804040
- work-record implementation commit: [`1d1d918`](https://github.com/artpoet/Memoryling/commit/1d1d9180a518f27c6340b2280370dec7f9226014)
- PASS — Windows CI for the v0.3.0 source bundle: https://github.com/artpoet/Memoryling/actions/runs/31612662676
- Daily Scout implementation commit: [`c3b59f8`](https://github.com/artpoet/Memoryling/commit/c3b59f851ab6c6aca3da9045dbea931b8fdca8f5)
- PASS — Windows CI for the v0.4.0 source bundle: https://github.com/artpoet/Memoryling/actions/runs/31664739429

## Codex for Open Source readiness

Memoryling can truthfully apply as a public MIT project maintained by `artpoet`, and its source-traceable／reversible architecture is relevant to the Codex ecosystem. It is not yet a competitive maintainer-program application: the repository remains young, has no public Release or demonstrated external maintainer loop, and the new work-record pilot has no authorized private-data UAT or packaged beta.

The internal decision is **do not submit yet**. With the pet shell complete, prove one real-source sequence, close release-environment checks, publish an honest beta and demo, then demonstrate genuine external testing plus at least one public feedback／issue → fix → follow-up release maintainer loop. The complete program facts, dynamic evidence warning, readiness gates, application drafts, and API-credit privacy boundary live in [the Codex for Open Source readiness plan](docs/research/2026-08-12_codex-for-open-source-readiness.md). Suggested tester counts are internal credibility targets, not official OpenAI thresholds.

## Known gaps

- the scaffold icons were replaced with generated test artwork, but the new artwork has not received public-release signoff
- accessibility requires a dedicated keyboard and screen-reader audit
- no stable Codex durable-memory API／schema is available; the work-record pilot is version-bound to exact CLI `0.134.0`, is not a production connector, and must fail closed on drift
- private-record UAT remains pending exact source／scope authorization; source v0.4.0 has not been packaged, installed, signed, or publicly released
- Daily Memory Scout has no real paid request or provider-account proof; an explicit non-private live-smoke authorization and packaged native UAT remain separate gates
- Daily Scout relevance／low-value fallback has bounded unit fixtures, not the planned broad synthetic quality evaluation or external tester evidence
- conversation model strategy remains intentionally open
- the evolving-creature system has product drafts and proposed ADR-0004, but its `SourceConsentScope`, in-scope automatic classifier, recent-hint TTL, outcome-qualified durable-growth gates, final visual asset, path-profile mapping, lineage-bearing bounded `MorphologyRecipe` catalog and compatibility matrix, genome, multi-source growth graph, renderer, privacy mode, and accessibility acceptance remain unimplemented
- no process, session, or Agent-presence monitoring exists; any future ephemeral presence adapter requires a separate product／privacy decision and explicit consent
- extended pet-shell acceptance remains open for live 125–200%／mixed DPI, monitor hot-unplug and taskbar relocation, adjacent-desktop click-through, precise drag bounds, Win+B, Narrator／NVDA, and sign-out／shutdown
- compact pet and onboarding envelopes are implemented; wide／tall／long growth envelopes remain future renderer work
- WebView2-missing bootstrapper behavior still needs UAT in a disposable clean Windows x64 environment; this host has WebView2 151.0.4129.78 and no Windows Sandbox／Hyper-V environment, and its runtime must not be removed for testing
- code signing and public distribution remain incomplete; v0.2.0 is unsigned and local-test-only, and the recorded checksum changes after any rebuild

## Next bundle

First unfinished feature gate: run one real Daily Scout request only after the user explicitly authorizes a paid smoke and provides／enters the intended API key through the product UI. Use only the visible coarse context; never retrieve a key from environment files or print it. Then verify one cited result, restart no-rerun, read state, disable, local clear, key deletion, and source-forget invalidation without capturing private content. Until that authorization exists, synthetic evidence is the honest boundary.

The v0.4.0 pilot's private-data UAT remains separate and may run only after the user names one exact Codex work record and authorizes the recorded read／local-storage／derivation scope. Do not list or read private records under a general continuation instruction.

If a fresh conversation has no exact private-source authorization, it should continue autonomously with the safe current-host shell-UAT fallback: native tray Show／Hide／Open／Always-on-top／Quit, `Win+B` keyboard recovery, both current 100% monitors' drag／restart／clamp behavior, adjacent-desktop hitbox, and built-in UIA／Narrator basics. Record only content-free evidence. Do not change display scaling, disconnect a monitor, relocate the taskbar through registry hacks, install NVDA, sign out／shut down, or remove WebView2; leave those items PENDING／DEFERRED for a suitable environment or explicit coordination.

The future Phase 2 bounded-variant growth direction is recorded in `docs/drafts/deep-interview-evolving-creature-system-2026-08-11.md`, `docs/drafts/deep-interview-agent-memory-variation-rules-2026-08-12.md`, and proposed ADR-0004; it does not supersede the installer, pet-shell, and real-source gates above.

After one authorized source slice is proven, prepare the public-beta and maintainer-evidence track: package and repeat installer UAT for that exact artifact, create a verified GitHub Release and checksums, recruit real external testers, record honest adoption, and complete at least one feedback／issue → fix → follow-up release loop. Refresh official terms and live evidence only when those gates are ready. Do not submit the application from the current pre-release state.

## Fresh-chat handoff

Paste-ready instruction:

> 請依專案 `AI-WAKEUP.md` 指定順序喚醒，先核對 `PROJECT_STATUS.md`、`main`／`origin/main` 與禁止重做邊界。v0.4.0 Daily Memory Scout source 已完成；若我沒有明確授權付費 API smoke，不得找現成 key 或發出真實請求。若我沒有明確指定一筆私人 Codex 工作紀錄及 read／local-storage／derivation scope，也不得 Browse 或呼叫 `thread/read`。請改做仍安全可執行的 synthetic quality／packaging preparation 或 current-host shell-UAT bundle，並完成內容最小化證據、SSOT、commit、push、CI／remote 核對與 Final Gate。不得重做 v0.1／v0.2 artifact gates、移除 WebView2、改 DPI／螢幕／taskbar、安裝 NVDA、sign out／shutdown，或提交 Codex for Open Source 申請表。

Expected wakeup outcome: the next agent either executes one explicitly authorized paid API smoke or private-record UAT, or—under a generic continuation request—finishes a non-private synthetic／current-host bundle without reopening completed implementation／installer work. Any environment-incompatible check stays honestly PENDING／DEFERRED.

## Do not redo

- do not replace the Tauri + React foundation without new evidence
- do not rebuild the completed fixture → pending preview → SQLite → lineage → recompute path
- do not distribute the raw release executable as a portable app without its generated fixture sidecar
- do not describe the unsigned NSIS test artifact or generated test icon as public release-ready
- do not recommend bypassing SmartScreen or weakening Windows security controls
- do not add open-ended AI chat before the memory lineage path exists
- do not describe the fixture pilot as access to the user's real Codex memories
- do not call thread-history work records “Codex memories,” parse generated memory state, widen the exact CLI pin, or revive arbitrary file access without a new supported-format decision
- do not rerun private `thread/read` without exact source authorization or claim that content-minimized catalog smoke is private-data UAT
- do not widen Daily Scout beyond its consented coarse-context, fixed-OpenAI, once-per-date, Web-Search-only boundary; do not treat generic continuation as paid-request authorization
- do not add cloud sync, telemetry, or any other remote memory processing by assumption
- do not implement the floating pet as a second independent creature state, expose full memory text to its surface, or treat a browser mock as native two-window verification
- do not collapse growth into one fixed evolution line, classify raw text into a personality, or let runtime AI／unsaved randomness choose a permanent route
- do not turn the reference forms into a fixed sprite roster or let live Agent presence silently accumulate into permanent morphology
- do not treat one-time consent as permission to scan new locations or add new data categories／purposes; do not promote ephemeral hints or raw usage volume into permanent growth
- do not describe Codex for Open Source as a contest, guaranteed `$1,200`, cash award, or automatic benefit for any public repository
- do not invent or inflate stars, downloads, testers, issues, pull requests, releases, testimonials, adoption, or maintainer work
- do not upload private source prose or broaden runtime cloud AI merely to strengthen the application or request API credits
- do not submit the application before the readiness gates are met unless the user explicitly changes that decision after reviewing current evidence
