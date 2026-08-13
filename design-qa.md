# Product design QA — first-run creature setup

- Reference: existing Memoryling detail surface, procedural memory-seed renderer, and established violet／mint visual system
- Implementation: native `Memoryling` detail window at 1180 × 780
- States checked: local-only default, OpenAI preparation expanded, English, Traditional Chinese
- Captured: 2026-08-13 through the running Tauri desktop application

## Findings

- P0: none
- P1: none
- P2: none after compacting the expanded API state to fit the native viewport without scrolling
- P3: the form intentionally uses the operating system title bar because this is the real detail window, not a frameless prototype

The live screen keeps the accepted egg-shaped programmatic creature visible, uses the existing visual language, makes local-only the recommended default, and clearly separates key preparation from later data consent and feature activation. Both language states fit the same viewport and retain keyboard-accessible native controls.

final result: passed
