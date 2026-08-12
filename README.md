# Memoryling

**Your agent memories, alive.**

[繁體中文](README.zh-TW.md) · [Windows test guide](docs/USER_GUIDE.md) · [Product vision](docs/PRODUCT_VISION.md) · [Architecture](docs/ARCHITECTURE.md) · [Roadmap](docs/ROADMAP.md)

[![CI](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml/badge.svg)](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-7c63d9.svg)](LICENSE)

Memoryling is an open-source, local-first desktop creature designed to grow from explicitly approved AI-agent memory sources. Its appearance, conversations, recurring story events, and occasional reminders should change for reasons you can inspect.

The source tree is currently at the **v0.3.0 development stage**. It contains a bilingual pet-first Windows desktop shell, the synthetic fixture pipeline, and a version-bound experimental Codex work／thread-history pilot. The pilot is not Codex durable-memory access and is not a production connector: OpenAI does not currently publish a stable durable-memory export API or compatibility-guaranteed memory-file schema for this use.

## Why it is different

Most desktop pets are decorative companions. Memoryling is designed around memory causality:

- **Memory becomes consequence.** Repeated ideas, completed work, unresolved promises, and protected values can shape the creature and its world.
- **Every meaningful change is explainable.** The fixture pilot includes a “Why did this happen?” lineage view; the same standard applies to every future real-memory effect.
- **Conflicts become stories.** Contradictory memories from different agents can become visible events instead of being silently flattened.
- **Initiative has limits.** Quiet hours, daily nudge budgets, and user-controlled sensitivity keep the companion useful without making it noisy.
- **Forgetting is a full chain.** Removing a source should also remove or recompute the effects derived from it.

## Pet-first shell, fixture flow, and work-record pilot

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

The browser preview cannot run this pipeline and intentionally stays on the honest detail surface; it does not imitate a floating pet, native menu, tray, single-instance lifecycle, or native persistence. The fixture flow cannot scan arbitrary paths, read user-owned Codex files, or write to an agent's memory store, and it makes no network request.

### Source-only experimental Codex work-record pilot

The v0.3.0 source tree also implements a narrow local pilot for **Codex work records／thread history**, never “Codex memories.” It fails closed unless the standard local Codex Desktop CLI reports the exact tested version `codex-cli 0.134.0`. The flow is deliberately explicit:

1. Nothing is discovered at startup. The user must choose **Browse local Codex work records**.
2. A content-minimized `thread/list` produces short-lived neutral candidates without thread titles, summaries, paths, raw identifiers, prompts, responses, or tool output.
3. Only after the user selects exactly one candidate may Memoryling call local stdio `thread/read`. It considers only the final `agentMessage` in phase `final_answer` from the last completed turn; other content categories are excluded.
4. The preview shows bounded counts, time／source metadata, exclusions, and the exact consent scope, but never displays the selected thread text.
5. Only after exact consent and an explicit completed-work confirmation may the selected final answer be normalized and stored in Memoryling's app-local SQLite database.
6. The adapter is read-only, accepts no arbitrary path, makes no model or external network call, and cannot write to or delete Codex data. Only one approved source may be active. **Forget** removes Memoryling's local copy and supported downstream effects, not the original thread.

Visible durable-memory access remains off. As of 2026-08-12, no private thread UAT has been authorized or performed; the source implementation and content-free catalog smoke do not make this a packaged or production-supported connector.

## Windows x64 pet-first fixture-only test build

The only installer with completed native installation UAT remains the current-user NSIS artifact `Memoryling_0.2.0_x64-setup.exe`. It is fixture-only, unsigned, and not public release-ready. Its exact size is 2,875,965 bytes and its SHA-256 is `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`.

The v0.3.0 source version does not imply that a v0.3.0 installer has been built, tested, or approved. The exact v0.2.0 artifact and its completed install／lifecycle／retain-data uninstall evidence are a no-redo baseline unless that artifact or relevant packaging behavior changes.

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

Memoryling is at a **v0.3.0 source-development stage**. The pet-first two-surface shell, local SQLite／lineage foundation, synthetic fixture path, and the version-pinned experimental Codex work-record pilot are implemented in source. The only completed installer UAT remains the exact fixture-only v0.2.0 artifact described above.

Phase 1 is still open. There is no supported Codex durable-memory interface, the work-record pilot depends on an experimental App Server host, and its separately authorized one-thread private UAT has not run. The WebView2-missing branch, remaining accessibility／DPI／recovery acceptance, production-supported memory connectors, notification delivery, code signing, and a public release-ready package also remain roadmap work.

## Contributing

Thoughtful issues and pull requests are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and the [Code of Conduct](CODE_OF_CONDUCT.md) first.

## License

MIT © 2026 Yupo Huang and Memoryling contributors.
