# Memoryling

**Your agent memories, alive.**

[繁體中文](README.zh-TW.md) · [Windows test guide](docs/USER_GUIDE.md) · [Product vision](docs/PRODUCT_VISION.md) · [Architecture](docs/ARCHITECTURE.md) · [Roadmap](docs/ROADMAP.md)

[![CI](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml/badge.svg)](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-7c63d9.svg)](LICENSE)

Memoryling is an open-source, local-first desktop creature designed to grow from explicitly approved AI-agent memory sources. Its appearance, conversations, recurring story events, and occasional reminders should change for reasons you can inspect.

The source tree is currently at the **v0.5.0 development stage**. Its primary source path is a one-time, read-only connection to the current local Codex Agent-memory store, followed by bounded local auto-sync. The pet-first shell, synthetic fixture, supplementary version-bound work-record pilot, and optional BYOK Daily Memory Scout remain available. No private-memory or packaged v0.5.0 acceptance is claimed.

## Why it is different

Most desktop pets are decorative companions. Memoryling is designed around memory causality:

- **Memory becomes consequence.** Repeated ideas, completed work, unresolved promises, and protected values can shape the creature and its world.
- **Every meaningful change is explainable.** The fixture pilot includes a “Why did this happen?” lineage view; the same standard applies to every future real-memory effect.
- **Conflicts become stories.** Contradictory memories from different agents can become visible events instead of being silently flattened.
- **Initiative has limits.** Quiet hours, daily nudge budgets, and user-controlled sensitivity keep the companion useful without making it noisy.
- **Forgetting is a full chain.** Removing a source should also remove or recompute the effects derived from it.
- **It can be useful, not just decorative.** If the user opts in and supplies an OpenAI API key, Daily Memory Scout can bring back one short, cited insight for the approved recent work each day.

## Pet-first shell and memory sources

The current Tauri + React app demonstrates:

- English and Traditional Chinese UI with a remembered language preference
- a transparent floating pet as the normal native surface, with the full detail window opened only on demand
- native right-click and focused-keyboard menus, tray recovery, single-instance relaunch, and close／minimize return-to-pet behavior
- one inspectable completion signal and creature mark derived from persisted Rust state
- a content-minimized pet render state, while private lineage and fixture controls remain confined to the detail surface
- explicit labels separating the fixture pilot from planned features and real memory access

In the Tauri desktop runtime, the fixture pilot exercises a narrow local path end to end:

1. Select the only approved source: one fictional Codex-shaped JSON record bundled with the app.
2. Review its exact scope, preview, record selection, and consent before source content is persisted.
3. Store the approved normalized record and lineage in Memoryling's app-local SQLite database.
4. Derive one deterministic completion star and inspect why it appeared.
5. Forget the local imported copy and remove or recompute its supported downstream effects.

The browser preview cannot run these native pipelines and intentionally stays on the honest detail surface; it does not imitate a floating pet, native menu, tray, single-instance lifecycle, or native persistence.

### Primary Codex Agent-memory source in v0.5.0

The user chooses **Codex · Local Agent memories** once and reviews a redacted source-level consent. Adapter v1 reads only top-level `memory_summary.md` and `MEMORY.md` from the configured Codex `memories` directory. It rejects symlinks, non-UTF-8 input, unsafe paths, oversized files, and changed source roots. Raw memory text remains in Rust and Memoryling's local SQLite store; the WebView receives only counts, timestamps, character counts, and hashes.

After approval, Memoryling checks the same read-only source at startup, every 15 minutes while running, and on **Sync now**. A successful change transactionally replaces local events and recomputes one lineage-backed memory halo. If the source disappears, local events and effects are withdrawn until it returns; unsafe or scope-changed input preserves the last valid state and reports attention needed. **Forget** removes Memoryling's consent, copy, lineage, and effects without editing Codex. Agent-memory content is never eligible for Daily Memory Scout.

### Supplementary experimental Codex work-record pilot

The v0.5.0 source tree retains the narrow local pilot for **Codex work records／thread history** as a supplementary compatibility source, not Agent memory. It fails closed unless the standard local Codex Desktop CLI reports the exact tested version `codex-cli 0.134.0`. The flow is deliberately explicit:

1. Nothing is discovered at startup. The user must choose **Browse local Codex work records**.
2. A content-minimized `thread/list` produces short-lived neutral candidates without thread titles, summaries, paths, raw identifiers, prompts, responses, or tool output.
3. Only after the user selects exactly one candidate may Memoryling call local stdio `thread/read`. It considers only the final `agentMessage` in phase `final_answer` from the last completed turn; other content categories are excluded.
4. The preview shows bounded counts, time／source metadata, exclusions, and the exact consent scope, but never displays the selected thread text.
5. Only after exact consent and an explicit completed-work confirmation may the selected final answer be normalized and stored in Memoryling's app-local SQLite database.
6. The adapter is read-only, accepts no arbitrary path, makes no model or external network call, and cannot write to or delete Codex data. Only one approved source may be active. **Forget** removes Memoryling's local copy and supported downstream effects, not the original thread.

As of 2026-08-13, no private Agent-memory or thread UAT has been authorized or performed; synthetic source proof does not make either path a packaged public release.

### Optional Daily Memory Scout

Memoryling can now do more than react as a pet. In the v0.5.0 source build, a user may optionally connect their own OpenAI API key and enable **one source-linked Web Search attempt per local day**. Memoryling compiles a visible, coarse summary only from a separately approved work record—not Agent memory—then searches after the chosen daytime setting while the app is running and returns a 1–3 sentence pet message with up to three clickable citations.

This feature is off by default. The ordinary local pet needs no API. The key is stored in Windows Credential Manager and is never returned to the WebView; Rust fixes the OpenAI endpoint, model, `store: false`, and Web Search tool. Prompts, final-answer text, paths, thread IDs, credentials, and arbitrary private phrases are excluded from the outbound context. The user's API account pays any cost, and ordinary OpenAI API abuse-monitoring retention may still apply. Turning the feature off stops future attempts; deleting a supporting source removes dependent local insights and invalidates consent.

The implementation has synthetic provider／citation／once-per-day coverage, but no real paid request, private-record UAT, or packaged v0.5.0 acceptance is claimed yet.

## Windows x64 pet-first fixture-only test build

The only installer with completed native installation UAT remains the current-user NSIS artifact `Memoryling_0.2.0_x64-setup.exe`. It is fixture-only, unsigned, and not public release-ready. Its exact size is 2,875,965 bytes and its SHA-256 is `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`.

The v0.5.0 source version does not imply that a v0.5.0 installer has been built, tested, or approved. The exact v0.2.0 artifact and its completed install／lifecycle／retain-data uninstall evidence are a no-redo baseline unless that artifact or relevant packaging behavior changes.

Read the [Windows x64 test guide](docs/USER_GUIDE.md) before installing. It covers the full fixture tour, WebView2 prerequisite download, Windows security warnings, app-data retention during uninstall, and why the raw release executable is not a portable distribution.

Run it locally:

    npm install
    npm run tauri dev

Frontend-only preview:

    npm run dev

Validate the project:

    npm run check

Requirements: Node.js 20+ and the current Tauri prerequisites for your platform.

## Product boundary

Memoryling is not a general AI assistant, a task manager with a mascot, or a hidden surveillance layer.

The intended connector model is:

1. Read explicitly approved durable-memory sources through source-specific, read-only adapters.
2. Normalize selected records into a local event format.
3. Derive inspectable traits, story hooks, reminders, and visual changes locally.
4. Preserve source lineage so a user can understand and delete downstream effects.

Raw memory files, credentials, prompts, and private work must never be committed to this repository or silently uploaded. See [Privacy principles](docs/PRIVACY_PRINCIPLES.md).

## Project status

Memoryling is at a **v0.5.0 source-development stage**. The pet-first shell, SQLite v4 lineage／sync foundation, primary Codex Agent-memory connector, synthetic fixture, supplementary work-record pilot, and opt-in Daily Memory Scout are implemented in source. The only completed installer UAT remains the exact fixture-only v0.2.0 artifact described above.

Phase 1 is still open for real-data and packaged acceptance. The generated Codex memory-file layout is not a stable third-party schema, no private Agent-memory UAT has run, and the supplementary work-record pilot still depends on an experimental App Server host. Daily Memory Scout also needs an explicitly authorized paid smoke and packaged native acceptance. The WebView2-missing branch, remaining accessibility／DPI／recovery acceptance, notification delivery, code signing, and a public release-ready package remain roadmap work.

## Contributing

Thoughtful issues and pull requests are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and the [Code of Conduct](CODE_OF_CONDUCT.md) first.

## License

MIT © 2026 Yupo Huang and Memoryling contributors.
