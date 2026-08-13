# ADR-0009: Agent conversation is the primary control surface and wakes the pet

- Status: Superseded by [ADR-0010](0010-installed-app-teaches-agent-activation.md)
- Date: 2026-08-13
- Extends: [ADR-0008](0008-agent-operated-memoryling-protocol.md)
- Related: [ADR-0003](0003-pet-first-two-window-desktop-shell.md)

## Context

An operation that requires the user to open Memoryling before speaking the slogan is not truly Agent-operated. A first-run setup gate and a single-instance callback that opens the detail window also break the intended pet-first result.

The user should remain in the Codex／Claude-style conversation. The Agent can perform the bounded local handoff and launch the installed desktop app after the same explicit phrase.

## Decision

1. `運作 Memoryling`／`Run Memoryling` means compile one authorized operation, submit it locally, start or recall Memoryling, wait for bounded local application confirmation, and report in the same Agent conversation.
2. The normal flow never asks the user to launch the App, open a settings screen, paste JSON, or run the PowerShell helper.
3. A wake-only phrase such as `叫出 Memoryling`／`Show Memoryling` starts or recalls the existing pet without reading memory or creating a new operation.
4. The submit helper resolves Memoryling 0.6.0 or newer before writing the inbox. It accepts an explicit development path, an existing Memoryling process, the current-user uninstall registration, or two exact current-user install candidates. It does not search `PATH` or accept a differently named binary.
5. After atomic submission, the helper starts the resolved executable and waits up to 15 seconds for the exact inbox file to be consumed. Missing／stale App and unconfirmed application are reported as bounded failures without printing paths or package content.
6. Cold launch shows the pet directly. There is no blocking first-run setup screen; initial language follows the OS／WebView locale and remains changeable in detail view.
7. A second launch reuses the existing single instance and returns to the pet, hiding detail if needed. It does not create another process-owned pet or SQLite writer.
8. Native detail, tray, clear, hide, and Quit controls remain recovery and safety surfaces. “Conversation-first” does not remove visible local user control.

## Consequences

- The complete happy path is one conversational instruction followed by the visible pet.
- Installed-version discovery becomes part of the local trust boundary and needs packaged Windows acceptance.
- Source-only and browser tests cannot prove installed executable discovery or single-instance wake behavior.
- Wake-only does not authorize context access; operation phrases retain ADR-0008's minimization boundary.

## Rejected alternatives

- **Ask the user to open the App first:** adds a setup ritual and makes the slogan incomplete.
- **Keep first-run setup as a blocking detail page:** places configuration before the creature and duplicates language context already available from the OS.
- **Launch any `Memoryling` found on `PATH`:** widens executable trust to mutable search order.
- **Add a localhost or cloud control API:** introduces an unnecessary listening or network surface for a local single-instance action.

## Rollback

Restore manual App launch documentation, remove `Start-Memoryling.ps1` from submission, and return the single-instance callback to detail recovery. Operation packages and Agent-owned context remain unchanged.
