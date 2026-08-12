# Memoryling｜記憶獸

**讓你的 Agent 記憶，長成一個生命。**

[English](README.md) · [Windows 測試指南](docs/zh-TW/USER_GUIDE.md) · [產品願景](docs/zh-TW/PRODUCT_VISION.md) · [架構](docs/ARCHITECTURE.md) · [路線圖](docs/ROADMAP.md)

Memoryling 是一個開源、local-first 的桌面生命。牠會從 AI Agent 的持久記憶中成長；外觀、對話、連續事件與偶爾出現的重要提醒，都應該有可以追溯的原因。

這個 Repo 目前包含雙語、pet-first 的 Windows 桌面外殼，以及僅使用 fixture 的記憶流程；**尚未讀取任何真實 Agent 記憶，也沒有連接使用者的 Codex tool-home**。

## 它有什麼不同

多數桌面寵物的核心是陪伴與裝飾；Memoryling 的核心則是「記憶造成可理解的改變」：

- **記憶會留下後果：**反覆出現的點子、完成的工作、尚未兌現的承諾與你守住的價值，會改變牠與牠的世界。
- **重要變化都能解釋：**目前的 fixture 試行已包含「為什麼會發生？」來源鏈檢視；未來每個真實記憶造成的影響也必須遵守同一標準。
- **矛盾會變成故事：**不同 Agent 的衝突記憶不被偷偷抹平，而會成為牠世界裡的事件。
- **主動性有界線：**安靜時段、每日提醒額度與敏感度由你控制。
- **遺忘是一整條鏈：**刪除來源後，衍生出的特徵、事件與提醒也要被刪除或重新計算。

## 現有 pet-first 外殼與 fixture 試行

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

瀏覽器預覽不能執行這條流程，並刻意維持誠實的詳細表面；它不會假裝有浮動寵物、原生選單、系統匣、single-instance lifecycle 或原生持久化。這項試行不能掃描任意路徑、讀取使用者的 Codex 檔案，也不能寫入 Agent 記憶庫，而且不會發出網路請求。

## Windows x64 pet-first fixture-only 測試版

測試使用者的正式入口是目前使用者（current-user）NSIS 安裝程式 `Memoryling_0.2.0_x64-setup.exe`。這是本機產出的未簽章 Windows x64 artifact，尚未達到公開發布品質；真實 Codex 記憶存取仍維持關閉。

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

Memoryling 目前是 **fixture-backed 開發階段（v0.2.0）**。pet-first 雙表面外殼、SQLite／來源鏈 v1 基礎，以及本機產出的未簽章 Windows x64 NSIS 測試 artifact 已可使用。這個完全相同的 v0.2.0 artifact 已通過 Explorer 啟動的 current-user 安裝、installed shortcut 的 single-instance 與 pet lifecycle smoke、明確 Quit，以及保留 App data 的解除安裝。WebView2 缺失分支、其餘 accessibility／DPI／救援驗收、真實記憶 connector、系統通知、程式碼簽章與可公開發布的正式安裝包仍在路線圖上。

fixture 基礎與目前的 pet-first 合成資料 bundle 都已實作，正常安裝的目前主機 smoke 也已通過；但 Phase 1 exit condition 尚未完成，目前沒有選取或匯入任何使用者擁有的 Codex 記憶。下一個實作 slice 可以開始驗證一種真實 Codex 格式，並維持使用者明確選取、唯讀、先預覽再同意的邊界。其餘 DPI／accessibility／session-recovery matrix 仍會阻擋完整 shell acceptance 與公開發布宣稱。

## 參與開發

歡迎提出有內容的 issue 與 pull request。請先閱讀 [CONTRIBUTING.md](CONTRIBUTING.md)、[SECURITY.md](SECURITY.md) 與 [Code of Conduct](CODE_OF_CONDUCT.md)。

## 授權

MIT © 2026 Yupo Huang 與 Memoryling contributors。
