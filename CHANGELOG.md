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
- Current-user Windows x64 NSIS test-build configuration and `npm run build:windows` command
- English and Traditional Chinese Windows test guides covering installation, fixture use, local data, uninstall limits, and raw-executable sidecars
- Test-build Memoryling icon and in-app brand artwork generated with Codex's built-in ImageGen; PNG transparency was checked for the test-build assets
- Product and technical design draft for deterministic, lineage-aware creature growth with large-form evolution, adjacent-stage EvolutionBridges, reversible time states, local rendering, and no runtime image-generation dependency
- User-confirmed pet-first desktop-shell design, proposed ADR-0003, and a staged native acceptance plan for one floating pet, on-demand detail, right-click menu, tray recovery, single-instance lifecycle, and content-minimized render state
- User-confirmed shared biological／organic and restrained sacred-premium creature language, plus proposed ADR-0004 for deterministic, lineage-aware bounded-variant evolution and recomputation after forgetting

### Changed

- Creature marks now depend on persisted Rust state; remaining event and reminder examples are explicitly labeled as concept or planned UI
- Real-memory access remains visibly off during the fixture pilot and in browser preview
- Windows bundling now targets an unsigned bilingual NSIS test installer and can download Microsoft's WebView2 bootstrapper when the prerequisite is missing
- Future creature growth now treats concept forms as visual-vocabulary references, not a fixed route roster: approved durable Agent-activity evidence may deterministically compile into many bounded `MorphologyRecipe` variants, while live Agent presence remains reversible presentation state

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
