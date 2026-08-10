# Memoryling

**Your agent memories, alive.**

[繁體中文](README.zh-TW.md) · [Product vision](docs/PRODUCT_VISION.md) · [Architecture](docs/ARCHITECTURE.md) · [Roadmap](docs/ROADMAP.md)

[![CI](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml/badge.svg)](https://github.com/artpoet/Memoryling/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-7c63d9.svg)](LICENSE)

Memoryling is an open-source, local-first desktop creature that grows from the durable memories of your AI agents. Its appearance, conversations, recurring story events, and occasional reminders should change for reasons you can inspect.

This repository currently contains a bilingual interactive concept shell. It does **not** read real agent memories yet.

## Why it is different

Most desktop pets are decorative companions. Memoryling is designed around memory causality:

- **Memory becomes consequence.** Repeated ideas, completed work, unresolved promises, and protected values can shape the creature and its world.
- **Every meaningful change is explainable.** A future “Why did this happen?” view will trace changes back to source memories and confidence.
- **Conflicts become stories.** Contradictory memories from different agents can become visible events instead of being silently flattened.
- **Initiative has limits.** Quiet hours, daily nudge budgets, and user-controlled sensitivity keep the companion useful without making it noisy.
- **Forgetting is a full chain.** Removing a source should also remove or recompute the effects derived from it.

## Concept shell

The current Tauri + React prototype demonstrates:

- English and Traditional Chinese UI with a remembered language preference
- a living desktop-creature direction built with CSS
- inspectable sample memory signals
- a continuing pet event and bounded reminder controls
- explicit “memory access is off” and local-first privacy states

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

Memoryling is at **concept-shell stage (v0.1.0)**. The visual experience is runnable; real memory connectors, the local derived-memory store, notification delivery, and packaged releases are roadmap work.

The first engineering milestone is a read-only Codex memory connector with a local import preview and explicit consent gate.

## Contributing

Thoughtful issues and pull requests are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and the [Code of Conduct](CODE_OF_CONDUCT.md) first.

## License

MIT © 2026 Yupo Huang and Memoryling contributors.
