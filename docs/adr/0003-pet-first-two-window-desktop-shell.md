# ADR-0003: Pet-first two-window desktop shell

- Status: Proposed
- Date: 2026-08-11
- Related: [ADR-0001](0001-local-first-derived-memory.md), [ADR-0002](0002-sqlite-v1-fixture-first-memory.md)
- Design detail: [Pet-first desktop shell](../drafts/pet-first-desktop-shell-2026-08-11.md)

## Context

Memoryling is intended to feel like a small desktop life. The 0.2.0 vertical slice now implements the user-confirmed pet-first direction: normal presence is one transparent floating creature, while detailed memory, lineage, and settings content opens on demand in one conventional detail window. Real-source and creature-growth work have not started.

Making right-click the only entry would create discoverability and recovery failures. A transparent resident window also introduces Windows-specific lifecycle, focus, DPI, multi-monitor, accessibility, and single-instance requirements that cannot be solved as a CSS-only redesign.

## Proposed decision

Memoryling will adopt one native process with two pre-created Tauri windows, canonical Rust／SQLite memory state, and a separate content-free local shell-settings record:

1. **`pet` is the normal surface.** It is a tightly bounded, transparent, undecorated, fixed-size, skip-taskbar window. It does not take initial focus. It is always-on-top by default, with an explicit user toggle and Hide action.
2. **`main` is the detail surface.** It retains standard Windows decorations, taskbar and Alt+Tab behavior, resizing, semantic application UI, and full lineage controls. It is created at startup but remains hidden until explicitly opened.
3. **Right-click is primary, not exclusive.** The pet opens a native context menu whose first action is Open Memoryling. System tray through `Win+B`, Start Menu, and packaged UAT-confirmed installed shortcuts remain recovery paths. Enter／Space／Menu key／`Shift+F10` open the same menu only when the pet has focus.
4. **Only one pet is visible.** Opening or restoring `main` hides or docks `pet`; closing or minimizing `main` restores `pet`. Hide, close detail, forget a source, and quit remain distinct actions.
5. **Rust owns native lifecycle.** Rust creates the context menu and tray, validates the calling window label, handles show／hide／focus／quit, intercepts only detail `CloseRequested`, restores safe positions, and registers the single-instance handler. The frontend receives no broad cross-window, menu, tray, or window-creation capability.
6. **App commands are window-scoped.** Tauri app-command permissions generated with `AppManifest::commands` grant full memory commands only to `main`; `pet` receives only its render-safe state and necessary interaction commands. Sensitive Rust commands also reject non-`main` caller labels as defense in depth. Neither surface inherits an unreviewed `core:default` set.
7. **Pet state is content-minimized.** `pet` receives a dedicated `CreatureRenderState` that contains only render-safe appearance and neutral status data. It never receives approved memory text, source paths or locators, private lineage explanations, or arbitrary memory payloads.
8. **Real-memory honesty remains visible.** Until a real connector and consent flow exist, the one pet surface includes a low-interference but readable “memory access is off” badge. Browser preview does not fake native windows, tray, or persistence.
9. **Position is recoverable.** Pet placement is stored in logical coordinates with monitor and work-area context, then clamped at launch, show／recovery, settled move／scale change, and single-instance recovery. A controlled poll revalidates visible-pet monitor／work-area topology. P0 does not use click-through.
10. **No new network boundary.** This shell adds no remote AI, runtime image generation, telemetry, cloud sync, arbitrary filesystem access, autostart, or global shortcut.

The architecture is implemented, but this ADR remains **Proposed** until the remaining live Windows accessibility, DPI, multi-monitor, desktop-hitbox, and session-lifecycle acceptance gates pass.

## 0.2.0 implementation evidence

- **Automated:** 23 frontend tests and 29 Rust tests pass. The Rust suite covers an eight-thread first-open SQLite migration race, transition rollback, position／anchor recovery, content-minimized DTOs, exact local-only capabilities, and shell settings recovery.
- **Two independent security layers:** a production-authority invoke harness denies `pet` access to list, preview, cancel-preview, full-state, approve, and forget before handler entry; an empty-authority harness independently proves the dual WebView／native-window `MainCaller` label guard denies the same six. A `main` list invoke is the positive control.
- **Native core:** transparent pet, one-time onboarding, pointer and focused-keyboard native menu paths, single-instance recovery, close／minimize／restore, raw movement／second-monitor observation, core pet／main state transitions, and explicit native Quit pass on the current Windows host. Tray actions and position recovery have automated evidence but are not overstated as completed live acceptance.
- **Fixture continuity:** raw bundled fixture preview／approve, restart persistence, source → event → signal → completion-star lineage, cross-surface state, and complete forgetting pass. No real user memory was used.
- **Packaged:** a normal Explorer-launched current-user NSIS install and the actual installed Start shortcut pass cold launch and resident single-instance relaunch. Explicit Quit and retained-data uninstall also pass.
- **Artifact:** `Memoryling_0.2.0_x64-setup.exe`, 2,875,965 bytes, SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`, version 0.2.0, `NotSigned`.
- **Harness boundary:** an earlier agent-direct installer launch triggered Windows virtualization. That route is invalid acceptance evidence and is not classified as a product failure; packaged claims use normal Explorer and installed-shortcut launches.
- **Still pending:** live 125–200%／mixed-DPI movement, monitor hot-unplug, taskbar relocation, adjacent-desktop hitbox probing, `Win+B`, Narrator／NVDA, sign-out／shutdown, and compact／wide／tall／long growth envelopes. WebView2-missing bootstrapper testing is deferred.

## Consequences

### Positive

- The everyday product becomes the creature rather than a permanently open dashboard.
- Detailed and privacy-sensitive controls remain in a conventional, accessible window.
- Native menu, tray, and single-instance recovery reduce the chance of losing a hidden pet.
- A narrow pet DTO, app-command permissions, caller checks, and separate core capabilities preserve least privilege and prevent accidental display of memory content.
- The current fixture lineage and forgetting pipeline can remain the canonical backend rather than being duplicated per surface.

### Costs and limits

- The app becomes a resident two-window lifecycle and requires real desktop tests; browser smoke cannot prove it.
- Pre-creating the hidden detail WebView uses more memory but avoids Windows handler-time window-creation risk.
- Transparent Windows still intercept mouse input across their rectangular bounds, so asset bounds and hit regions must remain tight.
- Always-on-top can obstruct other work and therefore needs visible controls, tray recovery, and fullscreen behavior review.
- Mixed-DPI and monitor removal require custom validation beyond simply restoring saved coordinates.
- Gesture-threshold drag behavior is not guaranteed by Tauri's documented immediate-drag example and needs a packaged Windows spike before the click／drag contract is accepted.

## Rejected alternatives

- **Keep the dashboard as the primary window:** rejected because the creature remains an illustration inside an app rather than a persistent desktop presence.
- **Use right-click as the only entry:** rejected because hidden, off-screen, keyboard-only, and first-run states need recovery paths.
- **Build a branded WebView popover first:** deferred because native context menus already provide reliable positioning, DPI, keyboard, Esc, and screen-reader behavior.
- **Create the detail WebView on demand from a synchronous handler:** rejected for the first slice because Tauri documents a Windows deadlock risk in synchronous command or event handlers.
- **Give the pet broad frontend window permissions:** rejected because Rust can own lifecycle with a narrower trust boundary.
- **Rely only on a safe pet DTO while leaving app commands global:** rejected because Tauri permits `invoke_handler` commands to every window by default unless the app generates and assigns command permissions.
- **Use runtime AI image generation for growth:** out of scope; permanent growth remains deterministic, local, versioned, and lineage-aware under the existing product contract.

## Acceptance gate

Before this ADR can be marked Accepted, the packaged Windows build must close every remaining unchecked gate:

- [x] fresh launch shows one transparent pet without stealing focus or adding a taskbar item;
- [x] native right-click, focused-pet keyboard menu, Start Menu, and the actual installed shortcut open or recover one detail window;
- [x] detail close or minimize restores the pet; detail restore hides it; explicit Quit ends the one process;
- [x] repeat installed-shortcut launch keeps one resident process and reuses the existing app surfaces;
- [x] pet IPC and events contain no memory text, source locator, path, explanation, source identity, or content hash;
- [x] `pet` attempts to invoke list, preview, cancel-preview, full-state, approve, or forget are denied fail-closed by both production ACL and caller-label tests;
- [x] raw fixture approve, restart, explanation lineage, forget, and failed-transaction states stay consistent across both surfaces;
- [x] real-memory access remains visibly off and the shell adds no network boundary;
- [ ] live 125–200% scaling, mixed-DPI movement, monitor hot-unplug, taskbar relocation, and adjacent-desktop hitbox probing pass;
- [ ] direct tray Open／Show／Hide／always-on-top／Quit, `Win+B`, Narrator／NVDA, and remaining keyboard-only acceptance pass;
- [ ] Windows sign-out／shutdown prove the resident close logic does not block session exit;
- [ ] compact／wide／tall／long envelope bounds and interaction pass; only compact baseline behavior exists today;
- [ ] the deferred WebView2-missing bootstrapper check is completed before public distribution.
