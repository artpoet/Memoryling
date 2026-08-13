# Memoryling｜記憶獸

**你的 Agent 記得；你的 Memoryling 活著。**

Memoryling 是給 AI Agent 使用者的 local-first 桌面寵物。在已設定的專案中，只要說：

> **運作 Memoryling**

目前的 Agent 只使用它原本獲准讀取的脈絡，編譯成一份小型寵物更新，自動啟動或叫回已安裝的寵物，並等待本機套用完成。普通流程裡，使用者全程留在 Agent 對話即可；接著由 Memoryling 持續管理外觀、雙語對話、冷卻、期限、安靜時段與每日額度。

[English](README.md) · [產品願景](docs/PRODUCT_VISION.md) · [架構](docs/ARCHITECTURE.md) · [隱私](docs/PRIVACY_PRINCIPLES.md)

## 它有什麼不同

- **由 Agent 運作：** 語意理解留在使用者原本選擇的 Agent。
- **對話優先：** Agent 會提交更新並打開寵物，不需要手動啟動 App 或通過設定頁。
- **App 不自行呼叫 AI API：** 普通寵物不需要 API key，也不會自行發出模型請求。
- **本機持續生活：** App 負責狀態、時機、呈現與使用者控制。
- **最小化交接：** 更新包只有生成後的寵物狀態與雜湊引用，不含原始記憶、prompt、路徑、祕密或 reasoning。
- **有限主動性：** 環境對話遵守 22:00–09:00 安靜時段與每日兩次額度。
- **可替換、可清除：** 每次運作會取代上一份操作；使用者也能在本機清除。

## 運作方式

```text
「運作 Memoryling」
  → Agent 讀取原本已授權的記憶＋近期工作＋專案脈絡
  → Agent skill 編譯 protocol-v1 JSON
  → 本機工具確認已安裝 App 並寫入唯一收件匣檔案
  → 工具啟動或叫回寵物
  → Rust 驗證並只保留最新操作
  → 寵物依本機確定性規則改變外觀與說話
```

專案入口會辨識 `運作 Memoryling`、`執行 Memoryling` 與 `Run Memoryling`。可重複執行的流程在 [`skills/memoryling-operation/SKILL.md`](skills/memoryling-operation/SKILL.md)，嚴格格式在 [`schemas/agent-operation-v1.schema.json`](schemas/agent-operation-v1.schema.json)。

## v0.6.0 原始碼目前有什麼

- Tauri 2 pet-first Windows 外殼、透明寵物與詳細視窗
- SQLite schema v5 Agent 操作持久化
- render-state schema v6 與粗粒度活動外觀色彩
- 每次操作 3–12 組英文／繁體中文對話卡
- 開啟、點擊與有限環境觸發
- 唯一檔案收件匣輪詢，以及大小、symlink、schema、ID 嚴格檢查
- 提交後自動冷啟動，或透過 single-instance 叫回既有寵物
- 本機清除控制與權威快照替換語意
- 合成資料 Rust、React 與提交工具測試

瀏覽器預覽刻意只顯示詳細頁；它沒有桌面收件匣、持久化或記憶存取。

舊版 fixture 匯入、直接 Codex 記憶 connector、單一 thread 試行與 BYOK Daily Scout 仍保留為相容性實驗原始碼；它們不會自動啟動，也不是主要產品流程。

## 開發

需求：Windows 11、Node.js、Rust 與 Tauri 前置環境。

```powershell
npm install
npm run tauri dev
```

完整檢查：

```powershell
npm run check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

只能用已提交的合成資料測試交接：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path examples/agent-operation-v1.synthetic.json
```

不要提交或輸出真實 Agent 記憶、prompt、憑證、本機資料庫或由使用者資料衍生的操作包。

## 產品與發布邊界

v0.6.0 目前是 source vertical slice。未簽章的 v0.2.0 installer 是最後一個完成安裝 UAT 的基準，不能證明新的 Agent-operated 路徑。Code signing、擴大無障礙／mixed-DPI、封裝升級測試與公開發布證據仍未完成。

本機狀態位於 `%LOCALAPPDATA%\app.memoryling.desktop`。清除操作只會刪除 Memoryling 的本機衍生更新，不會修改 Agent 擁有的記憶。

## 參與開發與授權

修改原始碼或協定邊界前，請先讀 [CONTRIBUTING.md](CONTRIBUTING.md)、[SECURITY.md](SECURITY.md) 與 [AGENTS.md](AGENTS.md)。

MIT License，詳見 [LICENSE](LICENSE)。
