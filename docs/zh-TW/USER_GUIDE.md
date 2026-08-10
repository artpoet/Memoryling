# Memoryling Windows x64 測試版使用指南

[English](../USER_GUIDE.md) · [專案 README](../../README.zh-TW.md) · [隱私原則](../PRIVACY_PRINCIPLES.md)

## 請先閱讀

Memoryling v0.1.0 目前是**僅使用 fixture 的 Windows x64 測試版**。它能針對一筆隨 App 打包的虛構紀錄，完整執行本機預覽、核准、SQLite 持久化、來源解釋與遺忘流程。

它**不會**讀取真實 Codex 記憶、不會掃描 Codex tool-home、不接受任意檔案，也沒有連接正式記憶來源。即使合成 fixture 試行正在運作，App 仍必須顯示「真實記憶存取關閉」。

這個測試版尚未簽章，也不是已可公開發布的正式安裝包。

## 測試使用者的正式入口

一般測試請使用目前使用者（current-user）的 NSIS 安裝程式：

    Memoryling_0.1.0_x64-setup.exe

開發機本機產出的檔案位於：

    src-tauri\target\release\bundle\nsis\Memoryling_0.1.0_x64-setup.exe

不要把這個 Repo 路徑當成公開下載網址。`src-tauri/target/` 底下是未 commit 的本機 build output。

### Raw exe 不是 portable 版本

`src-tauri\target\release\memoryling.exe` 是 build output，不是建議給測試者使用的入口，也不是單檔 portable 安裝包。fixture-only runtime 需要 raw exe 旁保留這個 sidecar 資源：

    src-tauri\target\release\fixtures\codex-first-memory-v1.json

若只移動或分享 `memoryling.exe`，fixture 可能無法使用。除非正在除錯 build tree，否則請使用 NSIS 安裝程式；若要執行 raw exe，必須保留產出時的 sidecar 目錄結構。

## 安裝前的安全提醒

- 目前測試安裝程式沒有程式碼簽章。Windows 可能顯示 **Unknown publisher（未知的發行者）** 或 Microsoft Defender SmartScreen 警告。
- 不要為了 Memoryling 關閉 SmartScreen、防毒軟體或其他 Windows 防護；也不要只因為看過這份指南就略過安全警告。
- 只安裝從你信任的專案管道取得的 artifact。若無法確認檔案來源或身分，請取消安裝。
- 2026-08-10 完成的本機測試安裝器大小為 2,759,655 bytes，SHA-256 是 `62FE4E5D87E4F221174F120F84A94303345C3694CA57090353438037F271D79B`。每次重建後都必須重新核對 hash；這次包裝與 icon source bundle 已通過 [Windows CI run 31394540587](https://github.com/artpoet/Memoryling/actions/runs/31394540587)。
- 此 artifact 針對 Windows x64 建置；本測試包不涵蓋其他 Windows 架構。

## 安裝與開啟

1. 雙擊 `Memoryling_0.1.0_x64-setup.exe`。
2. 閱讀每一個 Windows 安全提示。若無法確認 artifact 來源，請停止；本指南不建議繞過 Windows 防護。
3. 以英文或繁體中文完成目前使用者（current-user）安裝流程。
4. 若電腦缺少 Microsoft Edge WebView2，安裝器已設定為下載並安裝 Microsoft 的 WebView2 bootstrapper；這個前置步驟可能需要網路連線。
5. 完成安裝後，從 Windows 開始功能表開啟 **Memoryling**。

可能發生的 WebView2 下載屬於安裝前置需求。fixture 記憶流程本身沒有網路 client，也不會把記憶內容送出網路。

目前已成功產生安裝器並檢查設定，但尚未記錄一輪完整的人工「安裝 → 開啟 → 解除安裝」click-through UAT。若安裝器出現非預期行為，應把它視為測試發現，而不是假設整條流程已達正式發布品質。

## 執行 fixture-only 記憶流程

下方來源與紀錄都是 Repo 內的虛構 fixture。

### 1. 預覽

1. 先確認狀態仍顯示「真實記憶存取關閉」。
2. 選取 **Codex · First memory fixture**。
3. 檢查精確存取範圍：它應說明只讀取一個隨 App 打包的合成 JSON fixture、不能掃描任意路徑或 Codex tool-home、不能修改來源，也不會發出網路請求。
4. 選取 **預覽所選來源**。

預覽內容會在 Rust 記憶體中準備。預覽不等於核准，也不會把 fixture 的來源內容持久化。

### 2. 核准並儲存在本機

1. 檢查虛構紀錄、時間、類型與縮短顯示的 content hash。
2. 保持 **納入這筆紀錄** 為已選取。
3. 明確勾選同意選項。
4. 選取 **核准並在本機儲存 1 筆記憶**。

Memoryling 會把核准後的正規化 fixture 紀錄與來源鏈存入 App 本機 SQLite。這是真正的本機持久化，但輸入仍只有合成 fixture；它不代表 App 已存取使用者的 Codex 記憶。

### 3. 解釋印記

1. 確認記憶獸身上出現完成之星。
2. 選取 **為什麼會發生？**
3. 檢查「來源 → 正規化事件 → 確定性訊號 → 記憶獸影響」來源鏈。

畫面狀態必須持續把 fixture 試行與真實記憶存取分開顯示。

### 4. 遺忘本機匯入副本

1. 選取 **遺忘這個來源**。
2. 閱讀並勾選刪除確認。
3. 選取 **遺忘來源並移除印記**。
4. 確認匯入來源、正規化事件、訊號與完成之星都從 Memoryling 狀態消失。

遺忘流程會移除 App 的本機匯入副本與目前支援的下游來源鏈；它不會修改或刪除隨安裝包附帶的唯讀 fixture，也不是實體儲存媒體不可恢復抹除的保證。

## 本機資料與解除安裝

Memoryling 的目前使用者 App 資料位於：

    %LOCALAPPDATA%\app.memoryling.desktop

這個資料夾可能包含：

- `memoryling.sqlite3`：核准後的正規化 fixture 文字、hash、來源鏈與衍生狀態；
- `EBWebView` 等 WebView runtime 資料。

一般的 **遺忘這個來源** 流程會移除匯入 fixture 紀錄與目前支援的下游影響，但可能留下空的資料庫與 WebView 資料夾。

產生出的解除安裝程式包含 **刪除應用程式數據（Delete the application data）** 選項。若沒有明確勾選，App data 可能保留。目前尚未完成人工解除安裝 click-through 與這個刪除選項的 UAT；因此若你在意資料是否刪除，解除安裝後仍應檢查 `%LOCALAPPDATA%\app.memoryling.desktop`，不要直接假設資料夾已消失。

不得分享、附加、印出或 commit 真實本機資料庫。雖然目前 fixture 是虛構資料，同一位置仍是未來敏感本機狀態的保留區。

## 疑難排解邊界

- **App 顯示「真實記憶存取關閉」：**這是正確狀態；目前沒有真實 connector。
- **瀏覽器預覽顯示需要桌面執行環境：**這是正確狀態；SQLite 與 fixture 指令只存在於 Tauri desktop runtime。
- **WebView2 安裝失敗：**只透過可信任安裝器與可信任網路重試，或從 Microsoft 官方管道取得 WebView2；不要使用不明第三方 runtime 下載。
- **Windows 阻擋未簽章安裝器：**不要降低系統防護。請向專案擁有者確認 artifact，或等待已簽章、達公開發布品質的版本。
- **複製 raw exe 後找不到 fixture：**請還原產出時的 `fixtures` sidecar 結構，或改用 NSIS 安裝程式；raw exe 不是 portable 發布包。

## 開發者建置

在 Repo 根目錄執行：

    npm install
    npm run build:windows

正式測試入口的安裝器會產生在：

    src-tauri\target\release\bundle\nsis\Memoryling_0.1.0_x64-setup.exe

這個指令會 build 前端、編譯 Tauri App、打包合成 fixture 資源，並建立 current-user NSIS 安裝程式。分享任何 artifact 前，應重新執行專案檢查、完成人工安裝器 click-through UAT，並重新核對該檔案的 checksum 與 CI 證據。不得以 raw release exe 取代安裝器。

## 測試版圖像狀態

這個測試版的 icon 與 App 內品牌圖像由 Codex 內建 ImageGen 產出。已檢查來源 PNG 與產生的 PNG icon assets，確認包含 alpha channel 與透明像素。

這只是透明背景的技術檢查，不等於已核准公開發布。圖像、未簽章安裝器、商店呈現、授權審查與散布流程都尚未被宣稱為 public release-ready。
