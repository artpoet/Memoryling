# Memoryling｜記憶獸

**讓你的 Agent 記憶，長成一個生命。**

[English](README.md) · [產品願景](docs/zh-TW/PRODUCT_VISION.md) · [架構](docs/ARCHITECTURE.md) · [路線圖](docs/ROADMAP.md)

Memoryling 是一個開源、local-first 的桌面生命。牠會從 AI Agent 的持久記憶中成長；外觀、對話、連續事件與偶爾出現的重要提醒，都應該有可以追溯的原因。

這個 Repo 目前包含可互動的雙語概念原型，以及僅使用 fixture 的桌面記憶流程；**尚未讀取任何真實 Agent 記憶，也沒有連接使用者的 Codex tool-home**。

## 它有什麼不同

多數桌面寵物的核心是陪伴與裝飾；Memoryling 的核心則是「記憶造成可理解的改變」：

- **記憶會留下後果：**反覆出現的點子、完成的工作、尚未兌現的承諾與你守住的價值，會改變牠與牠的世界。
- **重要變化都能解釋：**目前的 fixture 試行已包含「為什麼會發生？」來源鏈檢視；未來每個真實記憶造成的影響也必須遵守同一標準。
- **矛盾會變成故事：**不同 Agent 的衝突記憶不被偷偷抹平，而會成為牠世界裡的事件。
- **主動性有界線：**安靜時段、每日提醒額度與敏感度由你控制。
- **遺忘是一整條鏈：**刪除來源後，衍生出的特徵、事件與提醒也要被刪除或重新計算。

## 現有概念原型與 fixture 試行

目前的 Tauri + React App 包含：

- 英文／繁中切換，並記住語言偏好
- 以 CSS 建立的桌面生命視覺方向
- 一個來自 Rust 持久化狀態、可檢視的完成訊號與記憶獸印記
- 一個連續寵物事件與有限提醒設定
- 清楚區分 fixture 試行、規劃中功能與真實記憶存取的狀態

在 Tauri 桌面執行環境中，fixture 試行會完整走過一條範圍受限的本機流程：

1. 選擇唯一獲准的來源：一筆隨 App 打包、仿 Codex 格式的虛構 JSON 紀錄。
2. 在來源內容持久化前，檢查精確存取範圍、預覽、紀錄選擇與同意內容。
3. 將核准後的正規化紀錄與來源鏈存進 Memoryling 的 App 本機 SQLite 資料庫。
4. 確定性地衍生一顆完成之星，並檢視它出現的原因。
5. 遺忘本機匯入副本，並移除或重新計算目前支援的下游影響。

瀏覽器預覽不能執行這條流程。這項試行不能掃描任意路徑、讀取使用者的 Codex 檔案，也不能寫入 Agent 記憶庫，而且不會發出網路請求。

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

Memoryling 目前是 **fixture-backed 開發階段（v0.1.0）**。視覺體驗與 SQLite／來源鏈 v1 基礎已能在本機針對打包的合成紀錄運作；真實記憶 connector、系統通知與正式安裝包仍在路線圖上。

第一個工程里程碑的 fixture 基礎已實作，但 Phase 1 的 exit condition 尚未達成：目前沒有選取或匯入任何使用者擁有的 Codex 記憶。下一步 connector 工作必須先驗證真實 Codex 格式，並維持使用者明確選取、唯讀、先預覽再同意的邊界。

## 參與開發

歡迎提出有內容的 issue 與 pull request。請先閱讀 [CONTRIBUTING.md](CONTRIBUTING.md)、[SECURITY.md](SECURITY.md) 與 [Code of Conduct](CODE_OF_CONDUCT.md)。

## 授權

MIT © 2026 Yupo Huang 與 Memoryling contributors。
