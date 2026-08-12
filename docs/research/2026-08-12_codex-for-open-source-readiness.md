# Codex for Open Source readiness plan

AS_OF: 2026-08-12 (Asia/Taipei)

## Decision

Memoryling is eligible in form but is **not ready for a competitive application yet**.

The product direction fits the Codex and open-source ecosystem: it is an MIT-licensed, local-first project that makes Agent-memory effects explainable, source-traceable, and reversible. The current public evidence is still that of a new pre-release prototype, not an adopted project with a demonstrated maintainer workload. Do not submit until the readiness gates below are met.

This is a rolling, discretionary maintainer-support program—not a contest, a fixed `$1,200` cash award, or a guaranteed benefit for publishing any repository.

## Official sources

- [Program page](https://developers.openai.com/community/codex-for-oss)
- [Application form](https://openai.com/form/codex-for-oss/)
- [Program terms](https://learn.chatgpt.com/docs/codex-for-oss-terms)

OpenAI says it considers meaningful usage, broad adoption or clear ecosystem importance, evidence of active maintenance, and the applicant's primary／core maintainer role. Examples include pull-request review, issue triage, and release management. There is no published minimum star or download count. Applications are reviewed on a rolling basis, selection remains discretionary, and benefits have no cash value.

## Current public evidence

Snapshot captured from public GitHub data on 2026-08-12:

- repository: `artpoet/Memoryling`, public, MIT-licensed
- repository age: created 2026-08-10
- commits on `main`: 13
- contributors: 1 (`artpoet`)
- stars／forks／watchers: 0／0／0
- GitHub Releases: 0
- issues／pull requests: 0／0
- five recorded Windows CI runs passed
- implemented proof: synthetic fixture → preview and consent → app-local SQLite → deterministic mark → machine-readable lineage → transactional forgetting and recomputation
- not implemented: user-owned real-memory connector, floating pet shell, deterministic growth renderer, public packaged release, or external-user maintainer loop

These counts are dynamic evidence. Refresh them immediately before any application; never copy this snapshot as if it were current.

## Application positioning

Lead with the open trust layer, not only the mascot:

> Memoryling is a local-first, source-traceable and reversible Agent-memory layer, made understandable through a living desktop creature.

The memorable product line is:

> Every change has a reason. Every reason can be revoked.

The strongest ecosystem case is a reusable read-only adapter contract, normalized event format, lineage／forgetting reference implementation, and synthetic conformance kit that other Agent communities can safely extend. The pet makes this infrastructure emotionally legible; it is not the entire qualification argument.

## Readiness gates

Complete in this order:

1. **Close Windows test-build UAT.** Run two clean current-user NSIS cycles: install → launch → fixture preview／approve／explain／forget → restart where relevant → uninstall. Prove both app-data retention and explicit delete-app-data paths, and refresh hashes after any rebuild.
2. **Ship the pet-first synthetic vertical slice.** Implement the floating pet, on-demand detail window, native menu／tray／single-instance recovery, permission separation, cross-surface state consistency, accessibility basics, and packaged native smoke without widening memory access.
3. **Prove one real source safely.** Support one documented, user-selected, read-only real Agent-memory source with redacted preview, narrow consent, lineage, correction／forgetting, and explicitly authorized private-data UAT. Never infer a public format from a private tool-home file.
4. **Prepare a public beta.** Add a strong README hero, a 60–90 second synthetic-data demo, a verified GitHub Release, checksums, release notes, installation boundaries, and a clear `Working now / Next / Vision` split. Decide signing and distribution honestly.
5. **Create genuine adoption evidence.** Recruit real external testers and record consented feedback, release downloads, issues or discussions, fixes, and a follow-up release. Demonstrate at least one complete `feedback／issue → maintainer response → fix → verified release` loop.
6. **Complete public maintainer identity and governance.** Make the GitHub profile understandable, keep CI／security／privacy／contribution guidance current, and show ongoing release and issue responsibility.
7. **Refresh and submit.** Recheck the official program page and terms, public GitHub metrics, ChatGPT-account email, OpenAI Organization ID, and every claim on the day of submission.

Internal credibility target: seek roughly 5–10 genuine external testers before submission if practical. This is a planning target, **not an OpenAI requirement**. Do not manufacture stars, downloads, issues, testimonials, or contributors.

## Demo and public proof

Use only synthetic data in screenshots and recording. A concise demo should show:

1. real-memory access is visibly off or narrowly scoped;
2. an approved record creates one deterministic visible consequence;
3. the user opens `Why did this happen?` and sees source → event → signal → effect;
4. forgetting the source removes or recomputes the consequence;
5. future pet and growth material is labeled `NEXT / CONCEPT` until implemented.

Do not show private paths, memory text, prompts, databases, tokens, or unredacted tool-home content.

## Draft application answers

The form currently asks for a public GitHub identity and repository, primary／core maintainer role, OpenAI Organization ID, and answers of at most 500 characters. Replace every bracketed metric with live verified evidence before submission.

### Why does this repository qualify? — draft, 398 characters

> Memoryling is an MIT-licensed, local-first Tauri/Rust project building a source-traceable and reversible memory layer for AI-agent workflows. Approved records produce explainable state; removing a source recomputes downstream effects. I maintain its architecture, security, CI, releases, issues, and contributor review. Current evidence: [N] testers, [N] release downloads, [N] external issues/PRs.

### How will you use API credits? — draft, 330 characters

> I will use credits only on public code and synthetic fixtures for PR review, issue triage, adapter compatibility and security checks, migration tests, bilingual release notes, and release automation. Memoryling remains local-first: private user memories will not be sent to the API, and all generated changes require human review.

### Anything else — draft, 358 characters

> Memoryling turns agent memory into visible consequences: every change has a source, every source can be inspected, and forgetting recomputes downstream state. The current release is honest about its limits; real-source access is opt-in and read-only. Codex supports my day-to-day architecture, implementation review, testing, documentation, and release work.

## Non-negotiable application boundaries

- Do not submit until the readiness gates have produced real public evidence, unless the user explicitly changes this decision after reviewing the current facts.
- Do not describe fixture-only behavior as access to real Codex or Agent memories.
- Do not describe planned pet, connector, or growth behavior as implemented.
- Do not add runtime cloud AI, telemetry, or private-memory uploads merely to request API credits.
- Do not call the program a guaranteed `$1,200`, funding, prize, or cash benefit.
- Do not invent or inflate stars, downloads, testers, issues, pull requests, releases, testimonials, maintainer work, or ecosystem importance.
- Do not put confidential information in the application.

## Refresh checklist before submission

- [ ] official program page, form fields, terms, and rolling status rechecked
- [ ] ChatGPT-account email confirmed
- [ ] OpenAI Organization ID confirmed
- [ ] GitHub profile and repository publicly visible and complete
- [ ] repo metrics, release downloads, external testers, issues, PRs, and contributors re-queried
- [ ] every current-versus-planned claim cross-checked against `PROJECT_STATUS.md`
- [ ] all three English answers are at most 500 characters after metric replacement
- [ ] no confidential or private-memory content appears in the form or linked materials
- [ ] public release, demo, checksums, CI, privacy, security, and maintainer-loop evidence are mutually consistent
