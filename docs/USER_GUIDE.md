# Memoryling Windows x64 Test Guide

[繁體中文](zh-TW/USER_GUIDE.md) · [Project README](../README.md) · [Privacy principles](PRIVACY_PRINCIPLES.md)

## Read this first

Memoryling v0.2.0 is currently a **pet-first, fixture-only Windows x64 test build**. Its native floating-pet shell and its local preview, approval, SQLite persistence, explanation, and forgetting path are functional for one fictional record bundled with the app.

It does **not** read real Codex memory, scan a Codex tool-home, accept arbitrary files, or connect to a production memory source. The app must continue to show that real-memory access is off, including while the synthetic fixture pilot is active.

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

## Use the pet-first shell

1. Normal launch shows the floating pet instead of opening the full detail window. An eligible first run may also show the one-time bilingual guide.
2. To open details, right-click the pet, then choose **Open Memoryling**. When the pet already has keyboard focus, Enter, Space, the Menu key, or `Shift+F10` opens the same native menu.
3. Drag the pet to reposition it. Closing or minimizing the detail window returns to the pet; opening or restoring details hides it so only one Memoryling surface is normally visible.
4. Use the native menu or system tray to show, hide, or open Memoryling. `Win+B` can reach the Windows tray, but that exact keyboard recovery path still needs dedicated packaged acceptance.
5. Choose **Quit Memoryling** to end the resident process. Hiding the pet or closing details is not Quit.

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

## Local data and uninstall behavior

Memoryling's current-user app data is stored under:

    %LOCALAPPDATA%\app.memoryling.desktop

The folder can include:

- `memoryling.sqlite3`, containing approved normalized fixture text, hashes, lineage, and derived state;
- `desktop-shell-v1.json` and a possible `desktop-shell-v1.json.bak`, containing only local shell settings such as onboarding, always-on-top, and safe pet position state;
- WebView runtime data such as `EBWebView`.

The normal **Forget this source** flow removes imported fixture records and supported downstream effects but may leave an empty database and WebView data directory.

The generated uninstaller includes a **Delete the application data** option. v0.2.0 native UAT verified that leaving it clear retained the app-data tree while removing the installed program, HKCU registration, shortcuts, and process. Only metadata was inspected; no database content was read. Historical v0.1.0 two-cycle UAT also verified that selecting the option removed the whole app-data tree, but that older result is not evidence that the current v0.2.0 delete-data path has been retested. If deletion matters, inspect `%LOCALAPPDATA%\app.memoryling.desktop` after uninstall. This is application-level cleanup evidence, not a physical secure-erasure guarantee.

Do not share, attach, print, or commit a real local database. Although the current fixture is fictional, the same location is reserved for future sensitive local state.

## Troubleshooting boundaries

- **The app says real-memory access is off:** expected. This build has no real connector.
- **The browser preview stays in the detail layout:** expected. Browser mode does not imitate the native floating pet, context menu, tray, single-instance lifecycle, SQLite, or persistence.
- **WebView2 installation fails:** stop and retry only through the trusted installer and a trusted network or obtain WebView2 through an official Microsoft channel. Do not use an unknown third-party runtime download.
- **Windows blocks the unsigned installer:** do not weaken system protections. Verify the artifact through the project owner or wait for a signed, release-ready build.
- **A copied raw executable cannot find the fixture:** restore the generated `fixtures` sidecar layout or use the NSIS installer. The raw executable is not a portable distribution.

## Developer build

From the repository root:

    npm install
    npm run build:windows

The supported installer artifact is generated at:

    src-tauri\target\release\bundle\nsis\Memoryling_0.2.0_x64-setup.exe

The command builds the frontend, compiles the Tauri application, bundles the synthetic fixture resource, and creates the current-user NSIS installer. Before sharing any rebuilt artifact, rerun the project checks and native installer click-through, then recheck the checksum and CI evidence for that exact file. Do not substitute the raw release executable for the installer.

## Test artwork status

The icon and in-app brand artwork in this test build were generated with Codex's built-in ImageGen. The source PNG and generated PNG icon assets were checked for an alpha channel and transparent pixels.

That is a technical transparency check, not public-release approval. The artwork, unsigned installer, store presentation, licensing review, and distribution flow have not been declared public release-ready.
