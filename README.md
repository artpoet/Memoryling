# Memoryling

**Your agent memories, alive.**

[繁體中文](README.zh-TW.md) · [Windows test guide](docs/USER_GUIDE.md) · [Product vision](docs/PRODUCT_VISION.md) · [Architecture](docs/ARCHITECTURE.md) · [Roadmap](docs/ROADMAP.md)

[![CI](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml/badge.svg)](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-7c63d9.svg)](LICENSE)

Memoryling is an open-source, local-first desktop creature that grows from the durable memories of your AI agents. Its appearance, conversations, recurring story events, and occasional reminders should change for reasons you can inspect.

This repository currently contains a bilingual interactive concept shell plus a fixture-only desktop memory pipeline. It does **not** read real agent memories or connect to a user's Codex tool-home.

## Why it is different

Most desktop pets are decorative companions. Memoryling is designed around memory causality:

- **Memory becomes consequence.** Repeated ideas, completed work, unresolved promises, and protected values can shape the creature and its world.
- **Every meaningful change is explainable.** The fixture pilot includes a “Why did this happen?” lineage view; the same standard applies to every future real-memory effect.
- **Conflicts become stories.** Contradictory memories from different agents can become visible events instead of being silently flattened.
- **Initiative has limits.** Quiet hours, daily nudge budgets, and user-controlled sensitivity keep the companion useful without making it noisy.
- **Forgetting is a full chain.** Removing a source should also remove or recompute the effects derived from it.

## Concept shell and fixture pilot

The current Tauri + React app demonstrates:

- English and Traditional Chinese UI with a remembered language preference
- a living desktop-creature direction built with CSS
- one inspectable completion signal and creature mark derived from persisted Rust state
- a continuing pet event and bounded reminder controls
- explicit labels separating the fixture pilot from planned features and real memory access

In the Tauri desktop runtime, the fixture pilot exercises a narrow local path end to end:

1. Select the only approved source: one fictional Codex-shaped JSON record bundled with the app.
2. Review its exact scope, preview, record selection, and consent before source content is persisted.
3. Store the approved normalized record and lineage in Memoryling's app-local SQLite database.
4. Derive one deterministic completion star and inspect why it appeared.
5. Forget the local imported copy and remove or recompute its supported downstream effects.

The browser preview cannot run this pipeline. The pilot cannot scan arbitrary paths, read user-owned Codex files, or write to an agent's memory store, and it makes no network request.

## Windows x64 fixture-only test build

The supported tester entry is the current-user NSIS installer named `Memoryling_0.1.0_x64-setup.exe`. This local Windows x64 artifact is unsigned and not a public release-ready package. Real Codex memory access remains off.

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

Memoryling is at a **fixture-backed development stage (v0.1.0)**. The visual experience, v1 SQLite/lineage foundation, and a local unsigned Windows x64 NSIS test artifact are available. Real memory connectors, notification delivery, code signing, and a public release-ready package remain roadmap work.

The fixture foundation for the first engineering milestone is implemented, but the Phase 1 exit is not met: no user-owned Codex memory has been selected or imported. The next connector work must validate a real Codex format and remain explicitly selected, read-only, previewed, and consented.

## Contributing

Thoughtful issues and pull requests are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and the [Code of Conduct](CODE_OF_CONDUCT.md) first.

## License

MIT © 2026 Yupo Huang and Memoryling contributors.
