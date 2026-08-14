# Memoryling

**Your Agent remembers. Your Memoryling lives.**

Memoryling is a local-first desktop pet for people who already work with AI Agents. Install and open the Windows app, then return to a configured project and say:

> **Memoryling, wake up**

The open pet shows this activation reminder itself. The current Agent then uses only context it is already authorized to read, compiles a small pet update, submits it to the running App, and waits for local application. Memoryling carries that update forward through appearance, bilingual dialogue, cooldowns, expiry, quiet hours, and daily limits.

[繁體中文](README.zh-TW.md) · [Product vision](docs/PRODUCT_VISION.md) · [Architecture](docs/ARCHITECTURE.md) · [Privacy](docs/PRIVACY_PRINCIPLES.md)

## Why it is different

- **Agent-operated:** semantic understanding happens in the Agent the user already chose.
- **Clear installed-App entry:** the user opens the EXE normally; the pet teaches the Agent activation phrase on first run, offers a one-click copy button, and keeps reminding while idle.
- **No app-side AI API:** the ordinary pet needs no API key and makes no model request.
- **Local persistent life:** the app owns state, timing, rendering, and user controls.
- **Privacy-minimized handoff:** packages contain generated pet state and hashed references, never raw memories, prompts, paths, secrets, or reasoning.
- **Bounded initiative:** ambient dialogue respects 22:00–09:00 quiet hours and a two-per-day budget.
- **Replaceable and clearable:** each run replaces the previous operation; the user can clear it locally.

## How it works

```text
Install and open Memoryling
  → pet shows “Memoryling, wake up” with a copy button
  → user pastes or says “Memoryling, wake up” in the current Agent project
  → Agent reads already-authorized memory + recent work + project context
  → Agent skill compiles protocol-v1 JSON
  → local helper verifies the compatible pet is already running
  → helper writes one exact inbox file and waits for application
  → Rust validates and stores the newest operation
  → pet appearance and dialogue follow deterministic local rules
```

The project entrypoint recognizes `Memoryling, wake up` and `寵物醒來`. Requests to read `AI-WAKEUP.md` or wake the project do not activate the pet. The reusable workflow is in [`skills/memoryling-operation/SKILL.md`](skills/memoryling-operation/SKILL.md); the strict contract is [`schemas/agent-operation-v1.schema.json`](schemas/agent-operation-v1.schema.json).

## Current v0.6.0 source

- Tauri 2 pet-first Windows shell with transparent pet and detail window
- SQLite schema v5 Agent operation persistence
- render-state schema v6 with coarse activity appearance accents
- 3–12 English／Traditional Chinese dialogue cards per operation
- on-open, click, and bounded ambient triggers
- exact-file inbox polling with strict size, symlink, schema, and identity checks
- pet-first manual launch, copyable bilingual activation reminder, persistent idle reminder, and single-instance recovery
- local clear control and authoritative replacement semantics
- synthetic Rust, React, and submit-helper coverage

Browser preview is intentionally detail-only: it has no desktop inbox, persistence, or memory access.

The older fixture import, direct Codex-memory connector, one-thread pilot, and BYOK Daily Scout remain compatibility experiments in source. They are not started automatically or shown as the primary product path.

## Develop

Requirements: Windows 11, Node.js, Rust, and the Tauri prerequisites.

```powershell
npm install
npm run tauri dev
```

Run the full check:

```powershell
npm run check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

Test the handoff only with the committed synthetic package:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path examples/agent-operation-v1.synthetic.json
```

Do not commit or print real Agent memories, prompts, credentials, local databases, or user-derived operation packages.

## Product and release boundary

Version 0.6.0 is currently a source vertical slice. The unsigned v0.2.0 installer is the last installed-UAT baseline and does not prove the new Agent-operated path. Code signing, extended accessibility／mixed-DPI testing, packaged upgrade testing, and public-release evidence remain open.

Local state lives under `%LOCALAPPDATA%\app.memoryling.desktop`. Clearing an operation deletes Memoryling's local derived package but never modifies Agent-owned memory.

## Contributing and license

Read [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [AGENTS.md](AGENTS.md) before changing source or protocol boundaries.

MIT License. See [LICENSE](LICENSE).
