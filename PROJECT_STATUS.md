# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-10 (Asia/Taipei)

## Current milestone

Complete the fixture-only “First real memory” vertical slice without touching private agent memory.

## Overall state

- Stage: v0.1.0 with a complete fixture-only first-memory pipeline
- Product surface: bilingual desktop and browser-safe experience implemented locally
- Synthetic Codex-shaped source pipeline: implemented end to end
- User-owned Codex memory access: not implemented
- Local store: SQLite schema v1 under Tauri app-local data
- GitHub repository: public at https://github.com/artpoet/Memoryling
- Default branch: main
- CI: local checks and this bundle's GitHub Actions run are passing
- Release: no packaged release

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

The Windows desktop-control helper could read the native UI but failed at input injection after recovery. Native click-through is therefore supported by Rust product-path tests plus frontend interaction tests, not claimed as completed human UAT.

Remote evidence:

- repository visibility is PUBLIC and default branch is main
- GitHub recognizes the MIT license
- discovery topics include agent-memory, desktop-pet, local-first, Tauri, React, and TypeScript
- private vulnerability reporting is enabled
- implementation commit: [`979bf7e`](https://github.com/artpoet/Memoryling/commit/979bf7eb19f31b1b3931b4c8824df1d94689408d)
- PASS — Windows CI for that implementation commit: https://github.com/artpoet/Memoryling/actions/runs/31380474307

## Known gaps

- scaffold icons have not been replaced with final Memoryling artwork
- accessibility requires a dedicated keyboard and screen-reader audit
- the adapter supports only the bundled synthetic v1 fixture; no user-owned Codex memory format is accepted
- a Rust-owned native picker, strict external-file validation, and preview redaction remain future work
- conversation model strategy remains intentionally open
- there are no packaged installers or releases

## Next bundle

Prepare a first user-selected Codex-source pilot as described in AI-WAKEUP.md. Do not treat private `MEMORY.md` files as a public format specification or begin real-data UAT without explicit source selection and authorization.

## Do not redo

- do not replace the Tauri + React foundation without new evidence
- do not rebuild the completed fixture → pending preview → SQLite → lineage → recompute path
- do not add open-ended AI chat before the memory lineage path exists
- do not describe the fixture pilot as access to the user's real Codex memories
- do not add cloud sync, telemetry, or remote memory processing by assumption
