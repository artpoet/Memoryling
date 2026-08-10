# Changelog

All notable changes to Memoryling will be documented here.

The project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and intends to use semantic versioning after its first public release.

## [Unreleased]

### Added

- Fixture-only first-memory flow with source selection, exact scope disclosure, preview, record selection, explicit consent, and cancel
- Versioned normalized `completion` event schema and app-local SQLite schema v1 with migration 0001
- Deterministic completion-star derivation with persisted source → event → signal → effect lineage
- “Why did this happen?” inspector plus transactional local-source forgetting and recomputation
- Bilingual desktop UI and automated Rust and React coverage for the supported synthetic path

### Changed

- Creature marks now depend on persisted Rust state; remaining event and reminder examples are explicitly labeled as concept or planned UI
- Real-memory access remains visibly off during the fixture pilot and in browser preview

### Planned

- User-selected, read-only connector for a validated real Codex durable-memory format
- Redacted real-source preview behavior and explicitly authorized private-data UAT
- Native reminder delivery with quiet hours and daily budgets

## [0.1.0] - 2026-08-10

### Added

- Bilingual English and Traditional Chinese concept shell
- Interactive CSS Memoryling creature
- Continuing-event, memory-signal, and bounded-initiative panels
- Explicit prototype and local-first privacy states
- Initial architecture, privacy, roadmap, and contributor documentation
