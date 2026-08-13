# Memoryling

**Your Agent remembers. Your Memoryling lives.**

Memoryling is a local-first desktop pet for people who already work with AI Agents. In a configured project, say:

> **Run Memoryling**

The current Agent uses only context it is already authorized to read, compiles a small pet update, starts or recalls the installed pet, and waits for local application. Memoryling then carries that update forward through appearance, bilingual dialogue, cooldowns, expiry, quiet hours, and daily limits. The user stays in the Agent conversation throughout the ordinary flow.

[繁體中文](README.zh-TW.md) · [Product vision](docs/PRODUCT_VISION.md) · [Architecture](docs/ARCHITECTURE.md) · [Privacy](docs/PRIVACY_PRINCIPLES.md)

## Why it is different

- **Agent-operated:** semantic understanding happens in the Agent the user already chose.
- **Conversation-first:** the Agent submits the update and opens the pet; no manual app launch or setup screen is required.
- **No app-side AI API:** the ordinary pet needs no API key and makes no model request.
- **Local persistent life:** the app owns state, timing, rendering, and user controls.
- **Privacy-minimized handoff:** packages contain generated pet state and hashed references, never raw memories, prompts, paths, secrets, or reasoning.
- **Bounded initiative:** ambient dialogue respects 22:00–09:00 quiet hours and a two-per-day budget.
- **Replaceable and clearable:** each run replaces the previous operation; the user can clear it locally.

## How it works

```text
“Run Memoryling”
  → Agent reads already-authorized memory + recent work + project context
  → Agent skill compiles protocol-v1 JSON
  → local helper verifies the installed app and writes one exact inbox file
  → helper starts or recalls the pet
  → Rust validates and stores the newest operation
  → pet appearance and dialogue follow deterministic local rules
```

The project entrypoint recognizes `Run Memoryling`, `運作 Memoryling`, and `執行 Memoryling`. The reusable workflow is in [`skills/memoryling-operation/SKILL.md`](skills/memoryling-operation/SKILL.md); the strict contract is [`schemas/agent-operation-v1.schema.json`](schemas/agent-operation-v1.schema.json).

## Current v0.6.0 source

- Tauri 2 pet-first Windows shell with transparent pet and detail window
- SQLite schema v5 Agent operation persistence
- render-state schema v6 with coarse activity appearance accents
- 3–12 English／Traditional Chinese dialogue cards per operation
- on-open, click, and bounded ambient triggers
- exact-file inbox polling with strict size, symlink, schema, and identity checks
- automatic cold launch or single-instance pet recovery after submission
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
