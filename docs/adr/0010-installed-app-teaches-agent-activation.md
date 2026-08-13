# ADR-0010: Installed App is the entry surface and teaches the Agent activation phrase

- Status: Accepted
- Date: 2026-08-13
- Supersedes: [ADR-0009](0009-conversation-first-pet-wake.md)
- Extends: [ADR-0008](0008-agent-operated-memoryling-protocol.md)

## Context

Automatically launching a desktop App from an Agent phrase hides the normal installed-software boundary and makes the conversational workflow responsible for process control. The intended experience is easier to understand when the user installs and opens Memoryling normally, sees the creature, and receives an explicit reminder about what to type in the current Agent project.

## Decision

1. The user installs Memoryling through the Windows EXE installer and opens it through the installed executable, shortcut, or Start menu.
2. Cold launch remains pet-first. The first-run pet guide names the activation phrase, and an idle pet continues to show the phrase after the guide is dismissed until an Agent operation exists.
3. `運作 Memoryling`／`Run Memoryling` means compile one authorized operation, submit it to an already-running compatible Memoryling process, wait for bounded local application confirmation, and report in the Agent conversation.
4. The Agent workflow never starts an executable. If Memoryling 0.6.0 or newer is not already open, submission fails before inbox write and tells the user to open the installed App before trying the phrase again. If the running App does not consume the inbox within the bounded wait, the helper removes the exact unconfirmed item.
5. The submit helper trusts only a running `Memoryling.exe` whose product identity is `Memoryling` and whose version is 0.6.0 or newer. An explicit executable path is development-only evidence and must also match a running process.
6. Manual resident relaunch remains single-instance and returns to the existing pet. It is a normal App action, not part of Agent submission.
7. The App still does not read Agent memory or call an AI API. The activation phrase retains ADR-0008's authorization and minimization boundaries.

## Consequences

- The visible journey becomes install EXE → open pet → read phrase reminder → use phrase in the current Agent project → watch the open pet update.
- Users can understand where the desktop process comes from and when it is running.
- A closed App produces a clear, content-free failure instead of a hidden launch or a delayed inbox update.
- Packaged acceptance must verify the installed launch surfaces, bilingual phrase reminder, running-process check, inbox consumption, and single-instance manual relaunch.

## Rejected alternatives

- **Keep automatic launch after the phrase:** contradicts the chosen installed-App entry and makes the Agent a process launcher.
- **Write the inbox while the App is closed:** creates a delayed update that may apply later without immediate user feedback.
- **Show the phrase only in documentation:** makes first use depend on external setup knowledge.
- **Add a localhost or cloud control API:** introduces an unnecessary listening or network surface.

## Rollback

Restore ADR-0009's launcher helper and automatic-wake submission only through a new accepted decision. The operation package, App inbox, and Agent-owned semantic boundary do not otherwise change.
