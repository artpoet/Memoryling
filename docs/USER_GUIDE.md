# Memoryling Windows x64 Test Guide

[繁體中文](zh-TW/USER_GUIDE.md) · [Project README](../README.md) · [Privacy principles](PRIVACY_PRINCIPLES.md)

## Read this first

The only packaged Memoryling build with completed installation UAT is the **pet-first, fixture-only Windows x64 v0.2.0 test artifact**. Its native floating-pet shell and its local preview, approval, SQLite persistence, explanation, and forgetting path are functional for one fictional record bundled with the app.

It does **not** read real Codex memory, scan a Codex tool-home, accept arbitrary files, or connect to a production memory source. The app must continue to show that durable-memory access is off, including while the synthetic fixture pilot is active.

Separately, the repository source tree is at v0.5.0. It includes the primary read-only **Codex Agent-memory** connector, supplementary version-bound **work-record／thread-history** pilot, and optional **Daily Memory Scout**. None is present in the tested v0.2.0 installer; private-memory, paid API, and packaged v0.5.0 acceptance remain unclaimed.

This test build is not a signed or public release-ready package.

## Supported tester entry

For normal tester use, start with the per-user NSIS installer:

    Memoryling_0.2.0_x64-setup.exe

It installs for the current Windows user. The locally generated developer artifact is located at:

    src-tauri\target\release\bundle\nsis\Memoryling_0.2.0_x64-setup.exe

Do not treat this repository path as a published download location. Build output under `src-tauri/target/` is local and is not committed.

### The raw executable is not portable

`src-tauri\target\release\memoryling.exe` is a build output, not the supported tester entry and not a standalone portable package. The fixture-only runtime expects this sidecar resource beside the raw executable:

    src-tauri\target\release\fixtures\codex-first-memory-v1.json

Moving or sharing only `memoryling.exe` can leave the fixture unavailable. Use the NSIS installer unless you are debugging the build tree and keep the generated sidecar layout intact.

## Safety before installation

- This test installer is currently unsigned. Windows may show **Unknown publisher** or a Microsoft Defender SmartScreen warning.
- Do not disable SmartScreen, antivirus, or other Windows protections for Memoryling. Do not bypass a warning merely because this guide exists.
- Install only an artifact obtained through a project channel you trust. If the file's origin or identity is unclear, cancel installation.
- The finalized local v0.2.0 test installer built on 2026-08-12 is 2,875,965 bytes with SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`. Recheck the hash after any rebuild.
- The package is built for Windows x64. Other Windows architectures are not covered by this test artifact.

## Install and open

1. Double-click `Memoryling_0.2.0_x64-setup.exe` in Windows Explorer.
2. Read every Windows security prompt. Stop if you cannot verify the artifact's origin; this guide does not recommend bypassing Windows protection.
3. Continue through the current-user installer in English or Traditional Chinese.
4. If Microsoft Edge WebView2 is missing, the installer is configured to download and install Microsoft's WebView2 bootstrapper. This prerequisite step may require an internet connection.
5. Finish installation, then open **Memoryling** from its normal Windows Start menu or desktop shortcut.

The possible WebView2 prerequisite download is part of installation. The fixture memory pipeline itself has no network client and makes no memory-content network request.

On 2026-08-12, this exact v0.2.0 artifact passed an Explorer-launched current-user installation into the real per-user LocalAppData location. Normal Start menu and desktop shortcuts were present, and the HKCU uninstall registration reported version 0.2.0. A shortcut cold launch showed the pet first; a resident relaunch stayed single-instance, right-click → **Open Memoryling** opened the detail window, closing detail returned to the pet, and explicit **Quit Memoryling** left no running process. Uninstalling with **Delete the application data** clear removed the program, HKCU uninstall registration, shortcuts, and process while retaining `%LOCALAPPDATA%\app.memoryling.desktop`; only filesystem and registration metadata were inspected, never database content.

An earlier agent-direct installer launch was affected by Windows virtualization. Its residue was removed and that attempt is excluded from product evidence. The WebView2-missing branch remains deferred until a safe disposable Windows x64 environment is available; the installed host runtime must not be removed merely to test it. This evidence does not make the unsigned build release-ready.

The exact v0.2.0 artifact, size, hash, and completed install／lifecycle／retain-data uninstall evidence are a no-redo baseline. Do not rebuild or repeat that acceptance unless the artifact or relevant packaging behavior changes.

## Use the pet-first shell

1. A brand-new v0.5.0 source build opens one first-run creation screen. Choose English or Traditional Chinese, then keep the recommended local-only pet or optionally save an OpenAI API key in Windows Credential Manager. Saving a key does not test it, enable Daily Scout, send context, or perform Web Search; those actions still require the later review and consent flow. Existing installations with older shell settings skip this new first-run screen.
2. After setup, normal launch shows the floating pet instead of opening the full detail window. An eligible first run may also show the one-time bilingual pet guide.
3. To open details, right-click the pet, then choose **Open Memoryling**. When the pet already has keyboard focus, Enter, Space, the Menu key, or `Shift+F10` opens the same native menu.
4. Drag the pet to reposition it. Closing or minimizing the detail window returns to the pet; opening or restoring details hides it so only one Memoryling surface is normally visible.
5. Use the native menu or system tray to show, hide, or open Memoryling. `Win+B` can reach the Windows tray, but that exact keyboard recovery path still needs dedicated packaged acceptance.
6. Choose **Quit Memoryling** to end the resident process. Hiding the pet or closing details is not Quit.

## Development source only: connect Codex Agent memories

This path is implemented in source v0.5.0, not the accepted v0.2.0 installer. Automated tests use synthetic temporary files; no private-memory acceptance is claimed.

1. Open details and choose **Codex · Local Agent memories**. Memoryling reads only enough to prepare a redacted preview of the two allowlisted documents; it never shows their text.
2. Confirm the preview lists only `memory-summary` and／or `durable-memory-registry`, exact adapter／category／purpose scope, read-only behavior, and automatic local sync. Individual document checkboxes are fixed because consent is source-level.
3. Accept the one-time consent and choose **Approve & store**. A memory halo should appear and status should show read-only auto-sync.
4. Use **Sync now** for an immediate check. Memoryling otherwise checks at startup and every 15 minutes while running.
5. If the source disappears, the UI should report it and withdraw the halo without deleting consent. When the same source returns, sync should restore derived state. Unsafe or changed scope reports attention needed and preserves the last valid copy.
6. Choose **Forget this source** to remove Memoryling's consent, local copy, lineage, sync state, and halo. The original Codex memory files must remain unchanged.

Agent-memory text is local-only and is never eligible for Daily Memory Scout. Do not capture private text in screenshots, logs, issues, or test evidence.

## Run the fixture-only memory tour

The source and record shown below are fictional repository fixtures.

### 1. Preview

1. If details are not open, right-click the pet and choose **Open Memoryling**.
2. Confirm the status still says that real-memory access is off.
3. Select **Codex · First memory fixture**.
4. Review the exact access scope. It should say that the adapter reads one bundled synthetic JSON fixture, cannot scan arbitrary paths or Codex tool-home files, cannot write to the source, and makes no network request.
5. Select **Preview selected source**.

The preview is prepared in Rust memory. Previewing does not approve or persist the fixture's source content.

### 2. Approve and store locally

1. Review the fictional record, timestamp, type, and shortened content hash.
2. Keep **Include this record** selected.
3. Select the explicit consent checkbox.
4. Select **Approve & store 1 memory locally**.

Memoryling stores the approved normalized fixture record and its lineage in the app-local SQLite database. This is real local persistence applied only to synthetic input; it is not access to a user's Codex memory.

### 3. Explain the mark

1. Confirm that the completion star appears on the creature.
2. Select **Why did this happen?**
3. Inspect the source → normalized event → deterministic signal → creature effect chain.

The visible status must continue to separate the fixture pilot from real-memory access.

### 4. Forget the imported copy

1. Select **Forget this source**.
2. Read and select the deletion confirmation.
3. Select **Forget source and remove mark**.
4. Confirm that the imported source, normalized event, signal, and star disappear from Memoryling state.

Forgetting removes the app's local imported copy and supported downstream graph. It does not modify or delete the read-only fixture bundled with the installed app, and it is not a physical secure-erasure guarantee.

## Development source only: experimental Codex work-record pilot

The v0.5.0 source retains **Codex work records／thread history** as a supplementary one-record path through a narrow local App Server stdio boundary. The App Server host remains experimental and unsupported for production; this path is separate from the primary Agent-memory connector above.

This pilot is fail-closed and requires the standard local Codex Desktop CLI to report exactly `codex-cli 0.134.0`. It does not claim compatibility with earlier or later CLI versions. Its intended developer flow is:

1. Launch the current source build and explicitly choose **Browse local Codex work records**. The app does not list anything at startup or in the background.
2. Review the content-minimized catalog. Candidates have neutral labels and bounded source／time metadata; they do not reveal thread titles, summaries, repository paths, raw identifiers, prompts, responses, or tool output.
3. Explicitly select one candidate. Only then may the backend call local stdio `thread/read` for that one thread.
4. If the thread is complete, the backend considers only the final `agentMessage` whose phase is `final_answer` from the last completed turn. User prompts, commentary, reasoning, commands, tool output, and other items are excluded.
5. Review the redacted preview and exact consent scope. The preview reports bounded counts, source／time metadata, proposed completion-event count, and excluded data categories; it never displays the selected thread text.
6. Confirm that the selected work is complete and give exact consent. Only then may that final-answer content be normalized and persisted in Memoryling's app-local SQLite database. It is not returned through frontend IPC or shown in the UI.
7. Use **Forget** to remove Memoryling's local imported copy, lineage, and supported downstream effects. It never edits, archives, deletes, or otherwise changes the original Codex thread.

The adapter is read-only. It accepts no arbitrary path, does not scan Codex durable-memory files, makes no model or external network call, performs no source write, and permits only one approved source at a time. A different source requires forgetting the current Memoryling copy and completing a new browse, selection, preview, consent, and import flow.

Durable-memory access must remain visibly off. As of 2026-08-12, private-thread UAT has neither been authorized nor performed. Do not browse or read a private thread for acceptance until the user explicitly names the source／thread scope; even a later successful UAT would validate only this pinned experimental pilot, not a production connector or Phase 1 completion.

## Development source only: optional Daily Memory Scout

The v0.5.0 source detail window retains **Daily Memory Scout** as a compact opt-in panel. The ordinary pet does not need an API, and Agent-memory documents are never eligible input. To enable it:

1. First approve one supported Codex work record. Synthetic fixture data is never sent.
2. Paste a personal OpenAI API key and save it to Windows Credential Manager. The field clears immediately and the saved key cannot be shown again. Use the in-app official links to create a key or read the API guide.
3. Review the displayed coarse work summary. It may contain work categories, public tool／model names, generic goals, non-sensitive constraints, and dates. It excludes prompt／answer text, paths, repository URLs, thread IDs, credentials, and arbitrary private phrases.
4. Choose a time from 08:00 through 21:59, accept the cost／retention disclosure, and enable. While Memoryling is running, it will make at most one Web Search attempt per local date; a failure is not retried that day and missed days are not replayed.
5. Open the detail window when the pet says it found something useful. The compact card shows a short message, why it matches, search time, and up to three annotation-derived source links.

OpenAI is the only provider in this milestone. Rust fixes `gpt-5.6-terra`, the official endpoint, Web Search, and `store: false`. API usage may cost money, and ordinary OpenAI abuse-monitoring retention may still be up to 30 days. **Turn off** stops future attempts; **Clear local insight**, **Delete key**, and the full reset are separate controls. Forgetting the supporting work source removes dependent insights and disables this consent. No real paid request has been used as acceptance evidence yet.

## Local data and uninstall behavior

Memoryling's current-user app data is stored under:

    %LOCALAPPDATA%\app.memoryling.desktop

The folder can include:

- `memoryling.sqlite3`, containing approved normalized Agent-memory／fixture／work-record text, hashes, consent, sync state, lineage, derived state, and—when enabled—Daily Scout settings, attempt status, cited insights, and source lineage;
- the OpenAI API key is separate and stored in Windows Credential Manager, not this folder;
- `desktop-shell-v1.json` and a possible `desktop-shell-v1.json.bak`, containing only the content-free first-run completion bit and local shell settings such as onboarding, always-on-top, and safe pet position state;
- WebView runtime data such as `EBWebView`.

The normal **Forget this source** flow removes imported fixture records and supported downstream effects but may leave an empty database and WebView data directory.

The generated uninstaller includes a **Delete the application data** option. v0.2.0 native UAT verified that leaving it clear retained the app-data tree while removing the installed program, HKCU registration, shortcuts, and process. Only metadata was inspected; no database content was read. Historical v0.1.0 two-cycle UAT also verified that selecting the option removed the whole app-data tree, but that older result is not evidence that the current v0.2.0 delete-data path has been retested. If deletion matters, inspect `%LOCALAPPDATA%\app.memoryling.desktop` after uninstall. This is application-level cleanup evidence, not a physical secure-erasure guarantee.

Do not share, attach, print, or commit a real local database. Although the current fixture is fictional, the same location is reserved for future sensitive local state.

## Troubleshooting boundaries

- **The app says memory access is off:** expected in the v0.2.0 installer, browser preview, before Agent-memory consent, or while an approved source is missing. Source v0.5.0 shows `codex-local` only while approved local Agent-memory events are available.
- **Daily Scout is disabled:** save a key, approve one supported work record, review the outbound context, and consent. It deliberately stays off in browser preview and for the synthetic fixture.
- **Today's search failed:** it will not retry automatically today. Check the API account or connection, then wait until the next local date; do not delete history to force a second paid attempt.
- **The Codex work browser says the CLI version is unsupported:** expected fail-closed behavior unless the standard local Codex Desktop CLI reports exactly `codex-cli 0.134.0`. Do not bypass the pin or point the app at an arbitrary executable.
- **The browser preview stays in the detail layout:** expected. Browser mode does not imitate the native floating pet, context menu, tray, single-instance lifecycle, SQLite, or persistence.
- **WebView2 installation fails:** stop and retry only through the trusted installer and a trusted network or obtain WebView2 through an official Microsoft channel. Do not use an unknown third-party runtime download.
- **Windows blocks the unsigned installer:** do not weaken system protections. Verify the artifact through the project owner or wait for a signed, release-ready build.
- **A copied raw executable cannot find the fixture:** restore the generated `fixtures` sidecar layout or use the NSIS installer. The raw executable is not a portable distribution.

## Developer build

From the repository root:

    npm install
    npm run check
    npm run tauri dev

The only installer with completed native UAT remains the existing v0.2.0 artifact at:

    src-tauri\target\release\bundle\nsis\Memoryling_0.2.0_x64-setup.exe

`npm run build:windows` builds the current source version and therefore creates a new, source-versioned, unverified artifact; it does not reproduce the exact tested v0.2.0 file above. Do not describe any rebuilt artifact as supported until its exact bytes have passed the required native installer click-through and its size, checksum, and CI evidence have been recorded. Do not substitute the raw release executable for an installer.

## Test artwork status

The icon and in-app brand artwork in this test build were generated with Codex's built-in ImageGen. The source PNG and generated PNG icon assets were checked for an alpha channel and transparent pixels.

That is a technical transparency check, not public-release approval. The artwork, unsigned installer, store presentation, licensing review, and distribution flow have not been declared public release-ready.
