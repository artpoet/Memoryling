# Memoryling v0.6.0 原始碼測試指南

[雙語完整指南](../USER_GUIDE.md) · [繁中 README](../../README.zh-TW.md) · [隱私原則](../PRIVACY_PRINCIPLES.md)

## 先確認版本邊界

v0.6.0 是 Agent-operated 的原始碼垂直切片；最後完成安裝 UAT 的 artifact 仍是 v0.2.0。尚未宣稱 v0.6.0 packaged acceptance、私人記憶 UAT 或公開發布。

除非已取得明確授權，否則只使用合成資料。不要貼出、輸出或提交由真實 Agent 記憶衍生的操作包。

## 由對話啟動

```powershell
npm install
npm run tauri dev
```

普通使用者不需要先開 App。直接在已設定的 Agent 對話喊出口號；Agent 會提交更新並啟動或叫回已安裝的寵物。沒有阻擋流程的首次設定頁；初始語言依作業系統語系決定，之後仍可切換。

## 喊出口號

1. 在 Codex、Claude Code 或其他支援 AGENTS 的環境開啟本 repo。
2. 讓 Agent 讀取 `AGENTS.md` 與喚醒文件鏈。
3. 說 **`運作 Memoryling`**、**`執行 Memoryling`** 或 **`Run Memoryling`**。
4. Agent 會依 `skills/memoryling-operation/SKILL.md`，只使用原本已授權的脈絡，建立暫存更新包並自行執行本機提交工具。
5. 工具確認相容 App、提交更新、啟動或叫回寵物、等待本機套用，最後在同一段對話回報。

只想顯示既有寵物、不更新內容時，說 **`叫出 Memoryling`** 或 **`Show Memoryling`**。Wake-only 不讀記憶，也不建立更新包。

這個口號只授權一次有限寵物更新；不代表授權新私人來源、雲端 connector、外部 AI 呼叫、信箱、登入帳號、憑證或 Agent 記憶寫入。

## 預期結果

- 詳細頁改為「Agent 更新已套用」。
- 寵物出現粗粒度活動色彩；milestone 可能出現星號。
- 先顯示一則開啟對話；點擊後向本機規則引擎要求另一則對話。
- 對話依英文／繁體中文語言顯示。
- 22:00–09:00 不顯示環境對話，每日環境對話上限兩則。
- UI 不顯示來源路徑、記憶原文、證據雜湊、prompt 或 reasoning。

## 只測合成收件匣

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/Submit-MemorylingOperation.ps1 -Path examples/agent-operation-v1.synthetic.json
```

工具會確認 Memoryling 0.6.0 以上版本，然後提交、打開寵物並等待套用；只輸出有限狀態、operation ID 與對話數。App 成功處理後會刪除收件匣檔案。不合規資料會被拒絕並移除，不會顯示內容。

## 替換與清除

每一份新操作都是 authoritative snapshot，會 transactionally 取代上一份操作與下游對話，不建立隱藏歷史庫。

需要立即移除時，在詳細頁選 **清除這次寵物更新**。這只刪除 Memoryling 本機衍生資料，不會修改 Agent 記憶。再次喊口號即可依目前脈絡重建。

## 瀏覽器與原生邊界

`npm run dev` 只提供詳細頁預覽，沒有原生收件匣、SQLite、透明寵物、系統匣或持久化。頁面必須誠實顯示記憶存取關閉。

原生寵物可拖曳、點擊對話、右鍵開選單；Enter／Space／Menu key／`Shift+F10` 是鍵盤等效路徑。隱藏、關閉詳細頁、Quit 與清除資料是不同動作。

## 完整檢查

```powershell
npm run check
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

畫面修改還要做 browser 或 native smoke，且只能留下合成、無內容的證據。
