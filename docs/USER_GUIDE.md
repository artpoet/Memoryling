# Memoryling Windows x64 Test Guide

[繁體中文](zh-TW/USER_GUIDE.md) · [Project README](../README.md) · [Privacy principles](PRIVACY_PRINCIPLES.md)

## Read this first

Memoryling v0.1.0 is currently a **fixture-only Windows x64 test build**. Its local preview, approval, SQLite persistence, explanation, and forgetting path is functional for one fictional record bundled with the app.

It does **not** read real Codex memory, scan a Codex tool-home, accept arbitrary files, or connect to a production memory source. The app must continue to show that real-memory access is off, including while the synthetic fixture pilot is active.

This test build is not a signed or public release-ready package.

## Supported tester entry

For normal tester use, start with the per-user NSIS installer:

    Memoryling_0.1.0_x64-setup.exe

It installs for the current Windows user. The locally generated developer artifact is located at:

    src-tauri\target\release\bundle\nsis\Memoryling_0.1.0_x64-setup.exe

Do not treat this repository path as a published download location. Build output under `src-tauri/target/` is local and is not committed.

### The raw executable is not portable

`src-tauri\target\release\memoryling.exe` is a build output, not the supported tester entry and not a standalone portable package. The fixture-only runtime expects this sidecar resource beside the raw executable:

    src-tauri\target\release\fixtures\codex-first-memory-v1.json

Moving or sharing only `memoryling.exe` can leave the fixture unavailable. Use the NSIS installer unless you are debugging the build tree and keep the generated sidecar layout intact.

## Safety before installation

- This test installer is currently unsigned. Windows may show **Unknown publisher** or a Microsoft Defender SmartScreen warning.
- Do not disable SmartScreen, antivirus, or other Windows protections for Memoryling. Do not bypass a warning merely because this guide exists.
- Install only an artifact obtained through a project channel you trust. If the file's origin or identity is unclear, cancel installation.
- The finalized local test installer built on 2026-08-10 is 2,759,655 bytes with SHA-256 `62FE4E5D87E4F221174F120F84A94303345C3694CA57090353438037F271D79B`. Recheck the hash after any rebuild. The packaging and icon source bundle passed [Windows CI run 31394540587](https://github.com/artpoet/Memoryling/actions/runs/31394540587).
- The package is built for Windows x64. Other Windows architectures are not covered by this test artifact.

## Install and open

1. Double-click `Memoryling_0.1.0_x64-setup.exe`.
2. Read every Windows security prompt. Stop if you cannot verify the artifact's origin; this guide does not recommend bypassing Windows protection.
3. Continue through the current-user installer in English or Traditional Chinese.
4. If Microsoft Edge WebView2 is missing, the installer is configured to download and install Microsoft's WebView2 bootstrapper. This prerequisite step may require an internet connection.
5. Finish installation, then open **Memoryling** from the Windows Start menu.

The possible WebView2 prerequisite download is part of installation. The fixture memory pipeline itself has no network client and makes no memory-content network request.

The installer has been generated and its configuration inspected, but a complete human install/open/uninstall click-through has not yet been recorded as passed UAT. Treat any unexpected installer behavior as a test finding rather than assuming the flow is release-ready.

## Run the fixture-only memory tour

The source and record shown below are fictional repository fixtures.

### 1. Preview

1. Confirm the status still says that real-memory access is off.
2. Select **Codex · First memory fixture**.
3. Review the exact access scope. It should say that the adapter reads one bundled synthetic JSON fixture, cannot scan arbitrary paths or Codex tool-home files, cannot write to the source, and makes no network request.
4. Select **Preview selected source**.

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
- WebView runtime data such as `EBWebView`.

The normal **Forget this source** flow removes imported fixture records and supported downstream effects but may leave an empty database and WebView data directory.

The generated uninstaller includes a **Delete the application data** option. App data may remain unless that checkbox is explicitly selected. The installer's full uninstall click-through and that deletion option have not yet completed human UAT, so do not assume the folder was removed: after uninstalling, inspect `%LOCALAPPDATA%\app.memoryling.desktop` if deletion matters to you.

Do not share, attach, print, or commit a real local database. Although the current fixture is fictional, the same location is reserved for future sensitive local state.

## Troubleshooting boundaries

- **The app says real-memory access is off:** expected. This build has no real connector.
- **The browser preview says desktop runtime required:** expected. SQLite and fixture commands are available only in Tauri desktop runtime.
- **WebView2 installation fails:** stop and retry only through the trusted installer and a trusted network or obtain WebView2 through an official Microsoft channel. Do not use an unknown third-party runtime download.
- **Windows blocks the unsigned installer:** do not weaken system protections. Verify the artifact through the project owner or wait for a signed, release-ready build.
- **A copied raw executable cannot find the fixture:** restore the generated `fixtures` sidecar layout or use the NSIS installer. The raw executable is not a portable distribution.

## Developer build

From the repository root:

    npm install
    npm run build:windows

The supported installer artifact is generated at:

    src-tauri\target\release\bundle\nsis\Memoryling_0.1.0_x64-setup.exe

The command builds the frontend, compiles the Tauri application, bundles the synthetic fixture resource, and creates the current-user NSIS installer. Before sharing any artifact, rerun the project checks, complete installer click-through UAT, and recheck the checksum and CI evidence for that exact file. Do not substitute the raw release executable for the installer.

## Test artwork status

The icon and in-app brand artwork in this test build were generated with Codex's built-in ImageGen. The source PNG and generated PNG icon assets were checked for an alpha channel and transparent pixels.

That is a technical transparency check, not public-release approval. The artwork, unsigned installer, store presentation, licensing review, and distribution flow have not been declared public release-ready.
