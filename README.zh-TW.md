# Memoryling｜記憶獸

**讓你的 Agent 記憶，長成一個生命。**

[English](README.md) · [產品願景](docs/zh-TW/PRODUCT_VISION.md) · [架構](docs/ARCHITECTURE.md) · [路線圖](docs/ROADMAP.md)

Memoryling 是一個開源、local-first 的桌面生命。牠會從 AI Agent 的持久記憶中成長；外觀、對話、連續事件與偶爾出現的重要提醒，都應該有可以追溯的原因。

這個 Repo 目前是一個可互動的雙語概念原型，**尚未讀取任何真實 Agent 記憶**。

## 它有什麼不同

多數桌面寵物的核心是陪伴與裝飾；Memoryling 的核心則是「記憶造成可理解的改變」：

- **記憶會留下後果：**反覆出現的點子、完成的工作、尚未兌現的承諾與你守住的價值，會改變牠與牠的世界。
- **重要變化都能解釋：**未來的「為什麼會發生？」會追溯到來源記憶與信心值。
- **矛盾會變成故事：**不同 Agent 的衝突記憶不被偷偷抹平，而會成為牠世界裡的事件。
- **主動性有界線：**安靜時段、每日提醒額度與敏感度由你控制。
- **遺忘是一整條鏈：**刪除來源後，衍生出的特徵、事件與提醒也要被刪除或重新計算。

## 現有概念原型

目前的 Tauri + React 原型包含：

- 英文／繁中切換，並記住語言偏好
- 以 CSS 建立的桌面生命視覺方向
- 可檢視的示意記憶訊號
- 一個連續寵物事件與有限提醒設定
- 清楚標示「尚未讀取記憶」與 local-first 隱私承諾

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

Memoryling 目前是 **概念原型階段（v0.1.0）**。視覺體驗已可執行；真實記憶 connector、本機衍生記憶資料庫、系統通知與正式安裝包仍在路線圖上。

第一個工程里程碑會是 Codex 記憶的唯讀 connector，並包含匯入預覽與明確同意閘門。

## 參與開發

歡迎提出有內容的 issue 與 pull request。請先閱讀 [CONTRIBUTING.md](CONTRIBUTING.md)、[SECURITY.md](SECURITY.md) 與 [Code of Conduct](CODE_OF_CONDUCT.md)。

## 授權

MIT © 2026 Yupo Huang 與 Memoryling contributors。
