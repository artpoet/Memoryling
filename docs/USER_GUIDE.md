# Memoryling v0.7.0 Source Test Guide

AS_OF: 2026-08-14. English and Traditional Chinese instructions are paired below.

## Read this first／請先閱讀

Version 0.7.0 is a source vertical slice. The last installed-UAT artifact is v0.2.0, so there is no accepted v0.7.0 public installer yet.

v0.7.0 是原始碼垂直切片；最後完成安裝 UAT 的成品仍是 v0.2.0，目前還沒有驗收完成的 v0.7.0 公開安裝程式。

Use only synthetic context unless you have explicit authorization for a real Agent project. Never paste or commit a real operation package.

除非已明確取得真實 Agent 專案授權，否則只使用合成脈絡；不要貼出或提交真實操作包。

## Installed-App start／由安裝版 App 啟動

```powershell
npm install
npm run tauri dev
```

The ordinary user installs and opens Memoryling first. The pet appears without a blocking setup page and tells the user which activation phrase to enter in the current Agent project. Initial language follows the OS locale and remains changeable in detail view.

普通使用者先安裝並打開 Memoryling。寵物會直接出現，不經過阻擋流程的設定頁，並提醒使用者要在目前 Agent 專案輸入哪一句發動語。初始語言依作業系統語系決定，之後仍可在詳細頁切換。

## Operate from the Agent／由 Agent 運作

1. Install and open Memoryling through the Windows EXE, shortcut, or Start menu; keep the pet running.
2. Open the project you want to use in Codex, Claude Code, or another AGENTS-compatible environment.
3. Let the Agent read `AGENTS.md` and the wake-up chain.
4. Use the pet's copy button, then paste the phrase into the Agent project: **`Memoryling, wake up`** or **`醒來吧我的寵物`**.
5. The Agent reads `skills/memoryling-operation/SKILL.md`, verifies the App is ready before memory read, uses only already-authorized context, creates a temporary protocol-v2 package, and runs the local helper itself.
6. The helper verifies that Memoryling 0.7.0 or newer is already running, submits without launching a process, waits for local application, and reports in the same conversation.

1. 透過 Windows EXE、捷徑或開始功能表安裝並打開 Memoryling，讓寵物保持執行。
2. 在 Codex、Claude Code 或其他支援 AGENTS 的環境開啟想使用的專案。
3. 讓 Agent 先讀 `AGENTS.md` 與喚醒文件鏈。
4. 按寵物畫面的複製按鈕，再把發動語貼到 Agent 專案：**`醒來吧我的寵物`** 或 **`Memoryling, wake up`**。
5. Agent 會讀 `skills/memoryling-operation/SKILL.md`，先確認 App 已就緒才讀取這次所需記憶，只使用原本已授權的脈絡，建立暫存 protocol-v2 更新包，並自行執行本機工具。
6. 工具確認 Memoryling 0.7.0 以上版本已開啟、不啟動任何程式，提交更新並等待本機套用，最後在同一段對話回報。

The phrase authorizes one bounded pet update only. It does not authorize new private sources, cloud connectors, external AI calls, email, browser accounts, credentials, or changes to Agent memory.

口號只授權一份有限的寵物更新，不代表授權新的私人來源、雲端 connector、外部 AI 呼叫、信箱、瀏覽器帳號、憑證或 Agent 記憶寫入。

If Memoryling is not installed or no compatible pet is open, the Agent stops before reading memory and asks you to install it if needed, open the pet, and enter the phrase again. It does not launch the App automatically.

如果尚未安裝 Memoryling，或沒有相容寵物正在執行，Agent 會在讀取記憶之前停止，提醒你先安裝（若需要）、打開寵物，再重新輸入口號；它不會自行啟動 App。

## Expected result／預期結果

- Detail status changes from waiting to **Agent operation applied**.
- The pet receives a coarse activity-colored aura and may receive a milestone star.
- One opening dialogue appears in a dismissible speech bubble; clicking the pet requests another eligible line.
- Dialogue appears in the selected English or Traditional Chinese locale.
- Ambient dialogue uses a 35–70 minute local cadence, stays silent from 22:00 to 09:00, waits ten minutes after any line, and is limited to seven lines per local day.
- A qualified visible appearance change applies at most once per local day; a second qualified plan waits for the next day.
- The app never shows source paths, memory text, evidence hashes, prompts, or reasoning.

- 詳細頁狀態由等待改為 **Agent 更新已套用**。
- 寵物會取得粗粒度活動色彩光環，里程碑操作也可能出現星號。
- 先在可關閉的說話框顯示一則開啟對話；點擊寵物會要求下一則符合條件的內容。
- 對話依選定語言顯示英文或繁體中文。
- 環境對話採 35–70 分鐘本機節奏、22:00–09:00 保持安靜、任一句之後至少等待十分鐘，每日本機上限七則。
- 符合證據門檻的可見外觀變化每個本機日最多一次；同日第二個合格計畫會等到隔日。
- App 不會顯示來源路徑、記憶原文、證據雜湊、prompt 或 reasoning。

## Safe synthetic handoff／安全合成交接

To test only the app inbox without Agent context:

若只想測試 App 收件匣、不讀 Agent 脈絡：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path examples/agent-operation-v2.synthetic.json
```

Open Memoryling first. The helper validates the package, verifies the running App, writes the exact inbox, waits for application, and prints only bounded status plus the operation ID and dialogue count:

請先打開 Memoryling。工具會驗證更新包與正在執行的 App、寫入唯一收件匣並等待套用；只輸出有限狀態、operation ID 與對話數量：

```text
%LOCALAPPDATA%\app.memoryling.desktop\agent-inbox\operation-v2.json
```

The app removes the inbox item after successful processing. Invalid input is rejected and removed without displaying its content.

App 成功處理後會刪除收件匣檔案；不合規資料會被拒絕並移除，也不會顯示其內容。

## Replace and clear／替換與清除

Every valid new operation atomically replaces the previous semantic snapshot. Unchanged stable dialogue keeps only its counters; retired dialogue content is deleted. The App does not build a hidden operation archive.

每一份有效新操作都會以 transaction 取代上一份語意快照；內容完全不變的穩定對話只延續使用計數，退場內容會刪除，不會建立隱藏歷史庫。

Use **Clear this pet update／清除這次寵物更新** in the detail window for immediate local removal. This clears Memoryling's operation, dialogue, evidence hashes, and counters. It never edits Agent memory. Run the slogan again to rebuild from current authorized context.

## Native pet controls／原生寵物控制

- drag with the primary pointer to move;
- click to request eligible dialogue;
- right-click or use Enter／Space／Menu key／`Shift+F10` to open the native menu;
- use the tray or installed entry to recover the detail window;
- choose Quit for process termination—hiding or closing details is not deletion.

- 以主要指標拖曳移動；
- 點一下要求下一則符合規則的對話；
- 右鍵或 Enter／Space／Menu key／`Shift+F10` 開啟原生選單；
- 從系統匣或安裝入口找回詳細視窗；
- 要終止程式請選 Quit；隱藏或關閉詳細頁不等於刪除資料。

## Browser boundary／瀏覽器邊界

`npm run dev` provides a visual detail preview only. It has no native inbox, SQLite persistence, tray, transparent pet, or Agent operation. The page must continue to say memory access is off.

`npm run dev` 只提供詳細頁畫面預覽，沒有原生收件匣、SQLite 持久化、系統匣、透明寵物或 Agent 操作；頁面必須持續誠實顯示記憶存取關閉。

## Local data and removal／本機資料與移除

State is stored under `%LOCALAPPDATA%\app.memoryling.desktop`. The clear control removes the current Agent-derived operation. Uninstall behavior depends on the installer's delete-app-data choice. SQLite `secure_delete` is enabled, but the project does not promise cryptographic erasure from backups or storage media.

資料位於 `%LOCALAPPDATA%\app.memoryling.desktop`。清除按鈕會移除目前 Agent 衍生操作；解除安裝是否刪除其他資料，取決於安裝程式的 delete-app-data 選項。SQLite 已開啟 `secure_delete`，但不宣稱能從備份或儲存媒體進行密碼學不可復原抹除。

## Developer verification／開發驗證

```powershell
npm run check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

Visible changes also require browser or native desktop smoke evidence. Record only synthetic, content-free proof.

畫面修改還需要瀏覽器或原生桌面 smoke 證據，且只能記錄合成、無內容的證明。
