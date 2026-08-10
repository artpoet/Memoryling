# PROJECT_STATUS — Memoryling

AS_OF: 2026-08-10 (Asia/Taipei)

## Current milestone

Establish a public, bilingual, runnable open-source concept repository for Memoryling.

## Overall state

- Stage: v0.1.0 concept shell
- Product surface: implemented locally
- Real memory access: not implemented
- GitHub repository: public at https://github.com/artpoet/Memoryling
- Default branch: main
- CI: passing on GitHub Actions
- Release: no packaged release

## Completed in this bundle

- scaffolded Tauri 2 + React 19 + TypeScript + Vite
- replaced the starter UI with a responsive English／Traditional Chinese concept experience
- added a CSS Memoryling creature, continuing-event panel, sample signals, explanation state, and bounded initiative
- removed unnecessary runtime network dependencies
- minimized Tauri capabilities and added a restrictive CSP
- documented product vision, architecture, privacy principles, roadmap, and initial ADR
- added open-source contribution, conduct, security, and agent-entry documents
- added Windows CI definition and GitHub templates

## Product truth

The current interface uses sample content only. It does not locate, read, normalize, store, transmit, or derive anything from real agent memories. The UI must continue to say this until a real connector and consent gate are verified.

## Verification evidence

- PASS — npm run check (TypeScript, Vite production build, and cargo check)
- PASS — cargo fmt --manifest-path src-tauri/Cargo.toml --check
- PASS — npm audit reported 0 vulnerabilities
- PASS — high-confidence secret scan found no hits
- PASS — runtime source scan found no Google Fonts or remote CSS imports
- PASS — local Vite smoke returned HTTP 200
- PASS — English and Traditional Chinese 1400 × 1100 screenshots were visually inspected
- PASS — GitHub CI run 31373607683 completed successfully on commit fe9f4e1

Remote evidence:

- repository visibility is PUBLIC and default branch is main
- GitHub recognizes the MIT license
- discovery topics include agent-memory, desktop-pet, local-first, Tauri, React, and TypeScript
- private vulnerability reporting is enabled
- CI evidence: https://github.com/artpoet/Memoryling/actions/runs/31373607683

## Known gaps

- scaffold icons have not been replaced with final Memoryling artwork
- accessibility requires a dedicated keyboard and screen-reader audit
- memory schema and database are undecided
- Codex source format must be handled through a narrow, versioned adapter
- conversation model strategy remains intentionally open
- there are no packaged installers or releases

## Next bundle

Implement the “first real memory” vertical slice described in AI-WAKEUP.md. The slice ends only when one synthetic Codex memory can be previewed, approved, stored locally, turned into one explainable creature mark, and fully forgotten.

## Do not redo

- do not replace the Tauri + React foundation without new evidence
- do not add open-ended AI chat before the memory lineage path exists
- do not weaken the visible prototype boundary
- do not add cloud sync, telemetry, or remote memory processing by assumption
