# Memoryling｜記憶獸

**讓你的 Agent 記憶，長成一個生命。**

[English](README.md) · [Windows 測試指南](docs/zh-TW/USER_GUIDE.md) · [產品願景](docs/zh-TW/PRODUCT_VISION.md) · [架構](docs/ARCHITECTURE.md) · [路線圖](docs/ROADMAP.md)

Memoryling 是一個開源、local-first 的桌面生命，設計目標是從使用者明確核准的 AI Agent 記憶來源中成長；外觀、對話、連續事件與偶爾出現的重要提醒，都應該有可以追溯的原因。

目前 source tree 是 **v0.4.0 開發階段**，包含雙語、pet-first 的 Windows 桌面外殼、合成 fixture 流程、綁定特定版本的實驗性 Codex 工作紀錄／thread history 試行，以及選配的 BYOK 每日記憶情報。工作紀錄試行不是 Codex 持久記憶存取，也不是正式 connector。

## 它有什麼不同

多數桌面寵物的核心是陪伴與裝飾；Memoryling 的核心則是「記憶造成可理解的改變」：

- **記憶會留下後果：**反覆出現的點子、完成的工作、尚未兌現的承諾與你守住的價值，會改變牠與牠的世界。
- **重要變化都能解釋：**目前的 fixture 試行已包含「為什麼會發生？」來源鏈檢視；未來每個真實記憶造成的影響也必須遵守同一標準。
- **矛盾會變成故事：**不同 Agent 的衝突記憶不被偷偷抹平，而會成為牠世界裡的事件。
- **主動性有界線：**安靜時段、每日提醒額度與敏感度由你控制。
- **遺忘是一整條鏈：**刪除來源後，衍生出的特徵、事件與提醒也要被刪除或重新計算。
- **不只是裝飾，也能幫得上忙：**使用者自願接上 OpenAI API 後，每日記憶情報每天可為近期核准工作帶回一則簡短、附來源的實用資訊。

## 現有 pet-first 外殼、fixture 流程與工作紀錄試行

目前的 Tauri + React App 包含：

- 英文／繁中切換，並記住語言偏好
- 以透明浮動寵物作為一般原生表面，只在需要時開啟完整詳細視窗
- 原生右鍵與已聚焦鍵盤選單、系統匣救援、single-instance 再次啟動，以及關閉／最小化詳細視窗後回到寵物的 lifecycle
- 一個來自 Rust 持久化狀態、可檢視的完成訊號與記憶獸印記
- 內容最小化的 pet render state；私密來源鏈與 fixture 控制只留在詳細表面
- 清楚區分 fixture 試行、規劃中功能與真實記憶存取的狀態

在 Tauri 桌面執行環境中，fixture 試行會完整走過一條範圍受限的本機流程：

1. 選擇唯一獲准的來源：一筆隨 App 打包、仿 Codex 格式的虛構 JSON 紀錄。
2. 在來源內容持久化前，檢查精確存取範圍、預覽、紀錄選擇與同意內容。
3. 將核准後的正規化紀錄與來源鏈存進 Memoryling 的 App 本機 SQLite 資料庫。
4. 確定性地衍生一顆完成之星，並檢視它出現的原因。
5. 遺忘本機匯入副本，並移除或重新計算目前支援的下游影響。

瀏覽器預覽不能執行這條流程，並刻意維持誠實的詳細表面；它不會假裝有浮動寵物、原生選單、系統匣、single-instance lifecycle 或原生持久化。fixture 流程不能掃描任意路徑、讀取使用者的 Codex 檔案，也不能寫入 Agent 記憶庫，而且不會發出網路請求。

### 僅在 source tree 的實驗性 Codex 工作紀錄試行

v0.4.0 source tree 也保留一條範圍狹窄的本機 **Codex 工作紀錄／thread history** 試行，絕不稱為「Codex 記憶」。只有標準本機 Codex Desktop CLI 回報完全相符的測試版本 `codex-cli 0.134.0` 時才會繼續，其他版本一律 fail closed。流程刻意要求每一步都由使用者明確觸發：

1. 啟動時不會探索任何內容；使用者必須主動選擇 **瀏覽本機 Codex 工作紀錄**。
2. 內容最小化的 `thread/list` 只產生短效、中性的候選項目，不顯示 thread 標題、摘要、路徑、原始識別碼、提示詞、回覆或工具輸出。
3. 只有使用者明確選取一個候選後，Memoryling 才能透過本機 stdio 呼叫 `thread/read`；只考慮最後一個已完成 turn 中、phase 為 `final_answer` 的最終 `agentMessage`，其餘內容類別都排除。
4. 預覽只顯示有限的數量、時間／來源 metadata、排除項目與精確同意範圍，不會顯示所選 thread 文字。
5. 只有完成精確同意與明確的「工作已完成」確認後，所選 final answer 才能被正規化並存進 Memoryling 的 App 本機 SQLite。
6. adapter 為唯讀、不接受任意路徑、不會呼叫模型或外部網路，也不能寫入或刪除 Codex 資料。同一時間只能有一個核准來源；**遺忘**只刪除 Memoryling 的本機副本與目前支援的下游影響，不會刪除原始 thread。

畫面上的持久記憶存取仍維持關閉。截至 2026-08-12，私人 thread UAT 尚未獲得授權，也尚未執行；source implementation 與不含內容的 catalog smoke 不代表已經有打包版或正式支援的 connector。

### 選配的每日記憶情報

Memoryling 現在不只會作為寵物回應。在 v0.4.0 source build 中，使用者可以自願連接自己的 OpenAI API key，開啟**每個本機日期最多一次、附來源的 Web Search 嘗試**。Memoryling 只從一筆已核准工作紀錄編譯畫面可見的粗略摘要，在 App 運行且到達使用者選定的日間時間後搜尋，再帶回 1–3 句寵物訊息與最多三個可開啟來源。

這個功能預設關閉，普通本機寵物完全不需要 API。Key 存在 Windows Credential Manager，不會回傳給 WebView；Rust 固定 OpenAI endpoint、模型、`store: false` 與 Web Search 工具。Prompt、final answer 原文、路徑、thread ID、憑證與任意私密句子都排除在外送脈絡之外。費用由使用者自己的 API 帳戶負擔，一般 OpenAI API 的濫用監控保存仍可能適用。關閉功能會停止未來嘗試；刪除支援來源會移除相依的本機情報並使同意失效。

目前已有 synthetic provider、citation 與每日一次測試，但尚未宣稱完成真實付費請求、私人紀錄 UAT 或 v0.4.0 打包版驗收。

## Windows x64 pet-first fixture-only 測試版

唯一完成原生安裝 UAT 的測試入口，仍是目前使用者（current-user）NSIS artifact `Memoryling_0.2.0_x64-setup.exe`。它僅使用 fixture、未簽章，也尚未達到公開發布品質。精確檔案大小為 2,875,965 bytes，SHA-256 為 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`。

source version 已是 v0.4.0，不代表 v0.4.0 安裝器已經 build、測試或核准。除非這個 artifact 或相關 packaging 行為改變，完全相同的 v0.2.0 artifact 及其已完成的安裝／lifecycle／保留資料解除安裝證據，都是禁止重做的基準線。

安裝前請先閱讀 [Windows x64 測試指南](docs/zh-TW/USER_GUIDE.md)。指南包含完整 fixture 操作流程、WebView2 前置下載、Windows 安全警告、解除安裝時的 App data 保留行為，以及 raw release exe 為何不是 portable 發布包。

在本機啟動：

    npm install
    npm run tauri dev

只預覽前端：

    npm run dev

驗證專案：

    npm run check

需求：Node.js 20+，以及目前 Tauri 對應平台的必要環境。

## 產品邊界

Memoryling 不是通用 AI 助理、套著吉祥物的待辦工具，也不是隱形監控層。

預計的連接方式是：

1. 只透過各來源專用的唯讀 adapter，讀取使用者明確核准的持久記憶。
2. 將選取的紀錄轉成統一的本機事件格式。
3. 在本機衍生可檢視的特徵、故事、提醒與外觀變化。
4. 保留來源鏈，讓使用者理解並刪除下游影響。

原始記憶、憑證、提示詞與私密工作內容，不得被 commit 到此 Repo，也不得在未告知下上傳。詳見 [隱私原則](docs/PRIVACY_PRINCIPLES.md)。

## 專案狀態

Memoryling 目前是 **v0.4.0 source 開發階段**。pet-first 雙表面外殼、本機 SQLite／來源鏈基礎、合成 fixture 路徑、綁定版本的實驗性 Codex 工作紀錄試行，以及選配的每日記憶情報都已在 source 中實作。唯一完成安裝 UAT 的仍是上方所述、僅使用 fixture 的精確 v0.2.0 artifact。

Phase 1 仍未完成：目前沒有受支援的 Codex 持久記憶介面，工作紀錄試行依賴實驗性 App Server host，且另需授權的單一私人 thread UAT 尚未執行。每日記憶情報也還需要明確授權的付費 smoke 與打包原生驗收。WebView2 缺失分支、其餘 accessibility／DPI／救援驗收、正式支援的記憶 connector、系統通知、程式碼簽章與可公開發布的正式安裝包，仍在路線圖上。

## 參與開發

歡迎提出有內容的 issue 與 pull request。請先閱讀 [CONTRIBUTING.md](CONTRIBUTING.md)、[SECURITY.md](SECURITY.md) 與 [Code of Conduct](CODE_OF_CONDUCT.md)。

## 授權

MIT © 2026 Yupo Huang 與 Memoryling contributors。
