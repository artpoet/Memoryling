# 每日記憶情報（Daily Memory Scout）— 深度訪談產出

> 訪談日期：2026-08-13（Asia/Taipei）  
> 輪數：6  
> 最終模糊度：9%  
> 狀態：正常收斂  
> 文件狀態：產品／技術規劃草案，不代表已實作、已啟用網路或已接受新的隱私邊界

## 結論

這個方向值得做。它讓 Memoryling 從「根據記憶回應的桌面寵物」前進到「在使用者同意下，每天帶回一則與近期工作有關的新情報」。第一版應命名為 **Daily Memory Scout／每日記憶情報**，而不是一般新聞、無邊界助理或背景監控。

MVP 使用使用者自備的 OpenAI API key（BYOK），每天最多執行一次 Web Search。功能預設關閉；未提供 API key、未完成同意或使用者關閉功能時，Memoryling 必須完整保留目前不連網的普通寵物行為。每日搜尋會根據已授權記憶中的近期工作生成一份有用但受限的工作脈絡，交給 OpenAI Responses API 的 Web Search；寵物只說 1–3 句，另提供一張可展開的精簡來源卡。

這項功能會改變 Memoryling 現行「沒有 runtime memory-content network request」的產品事實。因此，在實作前必須新增獨立 ADR、隱私資料流與同意版本；不能只把一個 API 欄位接進現有 UI。

## 清晰度分析

| 維度 | 分數 | 權重 | 加權 | 狀態 |
|---|---:|---:|---:|---|
| 目標清晰度 | 96% | 40% | 38.4% | 清晰 |
| 約束清晰度 | 90% | 30% | 27.0% | 清晰 |
| 成功指標 | 84% | 30% | 25.2% | 可接受 |
| **模糊度** |  |  | **9.4%** | 已低於 20% 門檻 |

## 一句話目標

在使用者明確開啟並提供 OpenAI API key 後，Memoryling 每天從已授權記憶判斷近期主要工作，搜尋一則最可能有幫助的最新技巧、工作流或模型更新，再用簡短的寵物語言與可查證來源告訴使用者。

## 使用情境

### 情境 A：Agent 程式工作

Memoryling 從近期已授權工作記憶看見使用者正在用 Agent 寫程式，且最近重點是長任務、驗證或多代理協作。每日情報搜尋近期官方更新與可靠的 vibe coding 工作流，最後由寵物說：

> 「我今天在工具森林裡撿到一個小技巧：長任務先切成有驗收條件的小段，比一直加長提示更不容易迷路。要看看來源嗎？」

展開卡片只顯示：`工作流技巧`、來源名稱、發布日期／更新日期、搜尋時間與「因為你最近在進行 Agent 程式工作」。

### 情境 B：AI 影片工作

Memoryling 看見使用者近期正以 MiniMax H3 製作影片，關注角色一致性、鏡頭銜接與高效率產出。它搜尋官方更新、可信工作流或可重現技巧，再把最佳候選轉成簡短寵物語言。

### 情境 C：沒有重大更新

當天仍執行一次搜尋並回傳最佳候選，但不可把普通資訊包裝成重大新聞。訊息應降級為 `小技巧` 或 `今天較平靜`：

> 「今天沒有大風吹過來，不過我找到一個可能省你幾步的小做法。」

### 情境 D：沒有可用工作記憶

若沒有任何已授權且仍有效的工作脈絡，Memoryling 不應把泛科技新聞假裝成個人情報，也不應把空白內容送往 API。寵物仍要說明：

> 「我今天還不知道你在忙哪一片森林；等我有一點獲准的工作線索，再替你去找。」

這是本機訊息，不消耗 Web Search，也不偽造個人化。

## 核心產品決策

### 1. 兩種模式完全分離

| 模式 | 網路／API | 行為 |
|---|---|---|
| 普通寵物 | 無 | 保留現有本機互動、記憶效果與說話能力 |
| 每日記憶情報 | 使用者明確啟用的 OpenAI BYOK | 每天最多一次相關資訊搜尋與一則寵物情報 |

- 新安裝、升級與 API key 遺失後，一律保持普通寵物模式。
- 關閉每日情報不得降低其他本機功能，也不得持續排程、重試或送出資料。
- 不以灰色勾選、預設開啟或把普通寵物功能綁在 API 同意上。

### 2. 每天都要有交代，但不能假裝有新聞

- 有有效工作脈絡：每天搜尋一次，選當日最佳候選，即使只是小技巧也要說。
- 沒有重大消息：以 `小技巧`／`今天較平靜` 標示資訊強度。
- 沒有已授權工作脈絡：不呼叫 API，使用本機誠實訊息。
- API 失敗：顯示「今天沒有成功帶回情報」，不拿舊內容冒充今天的新搜尋。
- 不以「沒有足夠好內容」為由無聲失敗；錯誤與低價值結果要能區分。

### 3. 一次同意後自動執行

使用者完成一次具體的 `DailyInsightConsentScope` 後，Memoryling 可以每日自動送出範圍內的工作脈絡，不需要每次預覽與確認。以下任何擴張都必須產生新 consent revision 並重新取得同意：

- 新增記憶來源；
- 新增送往 API 的資料類別；
- 把用途從「每日搜尋與寵物情報」擴大；
- 改用其他 API 供應商；
- 大幅增加外送內容上限；
- 開始保存過去未保存的原始內容。

### 4. 第一版只支援 OpenAI

- Provider 固定為 OpenAI，底層保留 provider interface，但 MVP 不提供多家選單。
- 設定頁提供官方捷徑：<https://platform.openai.com/api-keys>。
- 可另提供官方開發者 quickstart：<https://platform.openai.com/docs/quickstart/make-your-first-api-request>。
- 固定官方 API host；MVP 不接受使用者自訂 base URL，以免 key 被送往不明端點。
- 模型在實作時從當時官方支援 Responses API + Web Search 的成本平衡型模型中選定並 pin 版本。規格不把 2026-08-13 的「最新模型」永久寫死成產品契約。

## 資料流與隱私契約

```text
已授權記憶／工作事件
  -> 本機 eligibility filter
  -> 本機 recent-work context compiler
  -> 本機敏感資料刪除與大小上限
  -> DailySearchContext（可外送資料）
  -> Rust 原生層 + OpenAI Responses API + Web Search
  -> 結構化候選、URL citations、資訊強度
  -> 本機驗證／正規化
  -> lineage-aware DailyInsight
  -> 1–3 句寵物語言 + 精簡證據卡
```

### 可以送出的內容

`DailySearchContextV1` 應有足夠資訊改善搜尋品質，但不是整批原始記憶。建議欄位：

- 最近主要工作類型，例如 Agent 程式開發、AI 影片製作；
- 使用中的公開工具／模型名稱，例如 Codex、MiniMax H3；
- 當前工作目標，例如角色一致性、鏡頭銜接、驗證長任務；
- 非敏感限制，例如 Windows、本機優先、希望降低成本；
- 粗略時間範圍與資料新鮮度；
- 偏好的情報類別：模型更新、官方更新、工作流技巧。

初始上限建議：最近 14 天、最多 12 個合格工作訊號、外送文字最多 3,000 UTF-8 字元。這是待 synthetic eval 調整的工程起點，不是永久產品承諾。

### 一律排除

- API key、token、credential、cookie；
- 私人 prompt 全文、final answer 全文、對話全文；
- 本機路徑、repo 私有 URL、thread ID、資料庫內容；
- 客戶名稱、真實姓名、email、電話、付款資料；
- 未明確同意的來源或資料類別；
- 心理、醫療、政治、宗教、感情等敏感推論；
- Agent reasoning、工具輸出、命令紀錄與錯誤 log 原文；
- 任何只因「可能有幫助」而臨時擴張的內容。

### API 資料告知

啟用頁必須用簡短人話清楚告知：

1. 整理後的近期工作脈絡會送往 OpenAI，用於每天一次的搜尋與訊息生成；
2. OpenAI API 資料預設不拿來訓練模型，除非帳號主動選擇分享；
3. 一般 API 使用仍可能進入最長 30 天的濫用監控保存；符合資格的組織才可能使用額外的 retention controls；
4. Memoryling 的 Responses request 必須明確使用 `store: false`，且 MVP 不用 Conversations、background mode、File Search、remote MCP 或檔案上傳；
5. 關閉或刪除 Memoryling 本機資料，不能承諾刪除 OpenAI 已依法／依服務政策保留的濫用監控資料；
6. 費用由使用者的 OpenAI API 帳戶負擔，ChatGPT 訂閱不能被描述成等同 API 額度。

官方基準（AS_OF 2026-08-13）：

- [OpenAI API quickstart and Web Search example](https://platform.openai.com/docs/quickstart/make-your-first-api-request)
- [OpenAI API data controls](https://platform.openai.com/docs/models/default-usage-policies-by-endpoint)
- [OpenAI Responses API reference](https://platform.openai.com/docs/api-reference/responses)
- [Current OpenAI model catalog](https://developers.openai.com/api/docs/models)

## API key 安全

API key 是功能能否成立的硬門檻，不是一般偏好設定。

- React／WebView 不得持有、讀回或發送已保存的 key。
- key 只透過一次性的密碼輸入命令交給 Rust；命令返回後前端欄位立即清空。
- Rust 使用抽象 `CredentialVault`；Windows 實作存入 Windows Credential Manager 或等價的作業系統安全憑證庫。
- 不得存入 SQLite、JSON settings、localStorage、log、crash report、clipboard history、repo 或測試 fixture。
- UI 只顯示「已連接」與可選的非機密尾碼，不提供顯示完整 key。
- 提供「更換 key」「刪除 key」「測試連線」；刪除 key 同時停用每日搜尋。
- `pet` surface 無權呼叫 credential、consent、full insight 或 network command；只有 `main` 可以管理。
- 所有 API HTTP 呼叫由 Rust 原生層執行，前端不能控制 endpoint、Authorization header、tool list 或 system instructions。
- 測試連線是使用者觸發、無 Web Search 的獨立驗證行為；UI 必須提醒它也可能產生 API 使用。它不計入「每日一次搜尋」，但不得自動重複。

## 每日排程契約

### 定義

- 「每天一次」以目前使用者的本機曆日與記錄的 IANA timezone／Windows timezone 為界。
- 每個 local date 最多建立一個 `DailySearchAttempt`；應用程式重啟、睡眠喚醒、視窗重開或時區回撥不得重複搜尋。
- 預設送達時間建議為本地 10:00，使用者可以調整；必須落在 quiet hours 之外。
- App 關閉時不建立 OS 背景服務。錯過時間後，在下一次 Memoryling 正常運行且不在 quiet hours 的時機執行。
- 不補跑錯過的日子；今天只處理今天。
- 一次嘗試失敗後不自動再做第二次付費搜尋。寵物回報失敗，隔天再試。
- 第一版不使用原生通知；結果先進入寵物的待讀狀態，使用者下次看見／開啟 Memoryling 時呈現，避免偷渡尚未完成的通知政策。

### 不應計入每日搜尋的行為

- 本機 eligibility／context compilation；
- 查看昨天已保存的情報；
- 展開來源卡；
- 開啟來源連結；
- 使用者主動測試 API key 的非搜尋連線驗證。

## 搜尋、挑選與可信度規則

### 搜尋目標

依近期工作脈絡，優先尋找：

1. 官方模型／產品更新與 release notes；
2. 可重現的新工作流；
3. 能直接改善當前問題的技巧；
4. 若無重大更新，選擇一個仍具實用價值的近期或常青做法。

### 來源優先級

`官方／第一方文件 > 原始發布者與 release notes > 高品質技術來源 > 有明確證據的社群工作流 > 其他內容`

- 模型推出、規格、價格、日期與政策更新等容易變動的主張，至少需要一個第一方來源；沒有就標為未確認，不得用寵物口吻說成事實。
- 最終卡片最多顯示 3 個來源；至少 1 個可開啟 URL citation 才能稱為「最新資訊」。
- API response 的 citations 必須取自正式 `url_citation` annotations／web search source items；不信任模型在純文字中自行拼出的 URL。
- 外部網頁是未受信任資料，不得改寫 system policy、要求讀取本機檔案、執行指令、洩漏 key 或啟動其他工具。
- 此工作流只允許 Web Search；不允許 shell、computer use、remote MCP、任意 fetch、帳號登入或外部寫入。

### 資訊強度

| 等級 | 顯示 | 條件 |
|---|---|---|
| 3 | 值得留意 | 新且高度相關，有可信來源 |
| 2 | 工作流技巧 | 直接有用，但不是重大更新 |
| 1 | 今天較平靜 | 最佳候選價值有限，明確降級 |
| 0 | 未成功帶回 | API／來源／驗證失敗，不展示未驗證內容 |

## 寵物語言與證據卡

### 寵物訊息

- 1–3 句；目標是自然、溫暖、具體，不像摘要報告。
- 保留核心事實、限制與不確定性；可愛語氣不能把「可能」改成「確定」。
- 不說「我監視到你」「我讀了你的私密工作」；應說「因為你最近允許我參考……」。
- 不使用誇張詞，例如「革命性」「一定有效」「所有人都在用」，除非來源能直接支持。
- 英文與繁體中文須有同等語義，不把 citation 或限制只放在其中一種語言。

### 精簡證據卡

寵物旁只顯示分類與展開按鈕。展開後最多顯示：

1. 一句「為什麼適合你」；
2. 來源標題／網域；
3. 來源發布或更新日期（能取得時）；
4. Memoryling 搜尋時間；
5. 最多 3 個來源連結。

預設不顯示完整 outbound context、prompt、模型 reasoning、搜尋 query 列表或冗長評分。設定中的進階隱私頁可顯示「今天送出的脈絡摘要」與 lineage，不放在主寵物介面。

## 建議資料模型

### `DailyInsightConsentScope`

- `scope_id`
- `provider = openai`
- `source_scope_ids[]`
- `allowed_categories[]`
- `purpose = daily_relevant_web_search_and_pet_message`
- `context_schema_version`
- `selection_policy_version`
- `prompt_version`
- `max_context_chars`
- `max_searches_per_local_date = 1`
- `timezone`
- `delivery_time`
- `consent_revision`
- `consented_at`
- `disabled_at?`

### `DailySearchContext`

- `context_id`
- `scope_id`
- `local_date`
- `work_domains[]`
- `public_tools_and_models[]`
- `current_goals[]`
- `non_sensitive_constraints[]`
- `evidence_window`
- `source_event_refs[]`（只留本機）
- `outbound_text_hash`
- `redaction_version`

### `DailySearchAttempt`

- `attempt_id`
- `local_date`
- `scope_id`
- `context_id?`
- `provider_profile_version`
- `started_at`
- `finished_at?`
- `status`
- `response_id_hash?`
- `error_class?`（不得含 provider 原始錯誤 body 或 key）

### `DailyInsight`

- `insight_id`
- `attempt_id`
- `category`
- `value_level`
- `pet_message_en`
- `pet_message_zh_tw`
- `why_relevant_en`
- `why_relevant_zh_tw`
- `searched_at`
- `citations[]`
- `lineage_version`
- `expires_or_superseded_at?`

## Lineage、忘記與停用

完整 lineage 必須是：

```text
approved source records
  -> eligible work signals
  -> DailySearchContext hash
  -> DailySearchAttempt
  -> cited web sources
  -> DailyInsight
  -> rendered pet message
```

- 忘記任一支援來源後，所有依賴它的未過期 context、insight、pet 待讀狀態與 UI cache 必須在同一受控流程中刪除或重新判定。
- 既有 insight 不應自動重新搜尋，否則可能突破每日上限；若支援證據被忘記，直接移除該 insight。
- 停用功能會取消未執行排程並清除 pending context；是否保留已讀情報歷史由使用者控制。
- 「刪除每日情報歷史」與「刪除 API key」是不同動作，但設定頁應提供一鍵同時停用、刪除 key 與清除本機情報資料。
- 不承諾刪除 OpenAI 在服務端依法／依政策保留的資料。

## UI 流程

### 設定入口

`Settings -> Daily Memory Scout／每日記憶情報`

初始卡片：

- 狀態：Off；
- 一句價值說明；
- `Connect OpenAI API`；
- `Get an OpenAI API key` 官方連結；
- 「普通寵物不需要 API」明確文字。

### 啟用流程

1. 說明功能與每天一次上限；
2. 說明會送出與不會送出的資料；
3. 說明 API key、可能費用與 OpenAI retention 邊界；
4. 輸入並安全保存 key；
5. 可選擇明確觸發一次非搜尋連線測試；
6. 選擇送達時間／quiet hours；
7. 顯示一個 synthetic outbound context 範例；
8. 使用者明確勾選同意並開啟。

### 日常體驗

1. 寵物出現小型「帶回情報」狀態；
2. 顯示 1–3 句寵物訊息；
3. 使用者可展開精簡來源卡；
4. 可選 `有幫助`／`不太有用`，只在本機保存作為未來 ranking 資料；MVP 不因按鈕立即再搜尋；
5. 使用者可說「明天少找這類」，影響下次本機 context policy，不觸發今天第二次呼叫。

## 失敗與降級

| 情況 | 行為 |
|---|---|
| 功能未開／無 key／無 consent | 完全普通寵物模式；零網路呼叫 |
| 沒有合格工作記憶 | 本機誠實訊息；零網路呼叫 |
| key 無效／quota／billing 問題 | 一次失敗狀態；設定頁引導修正，不自動重試 |
| timeout／離線 | 當日標記失敗；不把昨天內容冒充今天 |
| citations 缺失 | 丟棄候選，顯示未成功帶回 |
| 來源日期不明 | 可作一般技巧，但不得宣稱「最新更新」 |
| model／response schema 漂移 | fail closed，不顯示未驗證內容 |
| 忘記支援來源 | 移除依賴 insight，不重新付費搜尋 |
| 關閉功能 | 立即取消未執行工作；普通寵物不受影響 |

## MVP 非目標

- 多家 AI／搜尋 API 選擇；
- 一天多次搜尋或按「再來一則」立即付費重跑；
- 把整批原始記憶、對話或檔案上傳；
- Memoryling 代管使用者 API key 的雲端服務；
- 背景 OS service、開機即搜尋或 App 關閉時排程；
- 原生系統通知；
- 自動執行搜尋到的指令、安裝工具、改專案或發送訊息；
- 開放式網頁瀏覽器、登入付費網站或爬取個人帳號；
- 根據敏感內容推論人格、健康或政治立場；
- 把每日情報直接當成永久形態變化的證據；
- 宣稱功能完全離線、零資料保留或沒有 API 費用。

## 成功指標與驗收條件

### 產品行為

- [ ] Off／無 key／無 consent 三種狀態各自證明 0 個 OpenAI request。
- [ ] 同一 local date 經重啟、睡眠喚醒、視窗重開與時區回撥仍最多 1 次 Web Search。
- [ ] 有有效脈絡時，每日產生 1 則情報或 1 個誠實失敗狀態，不無聲消失。
- [ ] 訊息維持 1–3 句；展開卡不超過「為何適合、來源、日期、搜尋時間」核心資訊。
- [ ] 沒有重大更新時使用低強度標籤，不誇大。
- [ ] 普通寵物功能在未接 API、停用或 API 故障時完整可用。

### 隱私與安全

- [ ] API key 不出現在 frontend IPC response、DOM、SQLite、JSON settings、log、crash artifact、test snapshot 或 git。
- [ ] Rust 固定官方 endpoint、tool allowlist 與 `store: false`；WebView 無法改寫。
- [ ] outbound context 通過資料類別 allowlist、secret／PII redaction、大小上限與 scope hash 驗證。
- [ ] source scope、資料類別、purpose、provider 或上限擴張都要求新 consent revision。
- [ ] `pet` 無法呼叫 credential、consent、network、full lineage 或 delete commands。
- [ ] 外部網頁 prompt injection 無法觸發本機讀取、命令執行或其他工具。
- [ ] forget／disable／delete-key 路徑不留下 ghost insight、pending context 或第二次搜尋。

### 資訊品質

- [ ] 至少 30 組 synthetic recent-work fixtures，topical relevance 人工評估達 80% 以上。
- [ ] 所有模型推出、價格、版本、發布日期等易變主張都有第一方 citation，或明確標為未確認。
- [ ] 100% 顯示的外部連結來自 API citation annotations／source items，而非模型純文字 URL。
- [ ] 英文與繁體中文的事實、限制、強度與 citation 語義一致。
- [ ] 低價值 fixture 能穩定降級成 `今天較平靜`，而不是虛構重大更新。

### Beta 成效（本機／自願回報，不加 telemetry）

- [ ] 5–10 位外部測試者能獨立完成 key 申請、啟用、停用與刪除。
- [ ] 連續 7 天的自願回饋中，至少 60% 情報被標為「有幫助」；此為內部迭代門檻，不是行銷宣稱。
- [ ] 至少完成一次公開 feedback／issue -> 修正 -> follow-up release 維護循環。

## 建議實作順序

### Bundle 0：架構與隱私批准

- 新增 Proposed ADR：optional BYOK daily web intelligence；
- 更新 architecture／privacy data-flow 與雙語產品說明；
- 定義 `DailyInsightConsentScopeV1`、外送 allowlist、刪除語義與 cost boundary；
- 對 outbound context 做 threat model：secret leak、prompt injection、重複扣款、scope creep。

Exit：未寫任何 live API code 前，資料流、同意、key 與忘記契約先可審查。

### Bundle 1：完全 synthetic 的本機垂直切片

- Rust scheduler／daily idempotency；
- synthetic recent-work context compiler 與 redaction；
- mock provider、mock citations、結構化 insight；
- 寵物訊息、證據卡、lineage、forget／disable；
- 英文／繁體中文與 frontend／Rust 測試。

Exit：沒有 key、沒有網路也能證明整條狀態機與刪除鏈。

### Bundle 2：OpenAI BYOK 與安全網路邊界

- OS credential vault；
- Rust-only fixed OpenAI Responses client；
- Web Search only、`store: false`、timeout、輸出上限、schema validation、citation extraction；
- connection test、quota／billing／auth 錯誤分類；
- one-attempt-per-day transaction guard。

Exit：synthetic context 的付費 live smoke 能在不輸出 key／private content 的情況下通過，且失敗不重複扣款。

### Bundle 3：精簡 UX 與使用者控制

- 設定／同意／官方 key 連結；
- 時間、quiet hours、停用、換 key、刪 key；
- 1–3 句訊息與精簡卡；
- 本機 feedback／明日偏好；
- 無 API 普通寵物完整回歸。

Exit：一般使用者能理解「什麼會離開裝置、每天幾次、由誰付費、怎麼停止」。

### Bundle 4：真實來源 UAT 與公開 beta gate

- 先用 synthetic work contexts 完成 packaged Windows UAT；
- 私人記憶 UAT 必須另由使用者指定來源與 scope，證據只記錄 content-free pass／fail；
- 驗證一次每日搜尋、重啟不重跑、來源卡、忘記、停用與 key 刪除；
- 更新 Release、demo、privacy、security、checksums 與 maintainer evidence。

Exit：不能以 mock／browser 測試取代 packaged native proof，也不能把私人內容放入 screenshot、issue、CI 或 repo。

## 關鍵實體

| 名稱 | 類型 | 關鍵屬性 | 關係 |
|---|---|---|---|
| 使用者 | core | 工作、語言、quiet hours、同意 | 擁有 key、來源與控制權 |
| 已授權工作記憶 | core | source scope、時間、資料類別 | 產生近期工作訊號 |
| DailySearchContext | core | 有限工作脈絡、hash、redaction version | 由本機編譯後送往 OpenAI |
| DailySearchAttempt | core | local date、一次上限、status | 觸發一次搜尋並防止重複 |
| DailyInsight | core | 類別、強度、訊息、citations | 呈現在寵物與證據卡 |
| DailyInsightConsentScope | core | provider、purpose、categories、revision | 授權每日自動外送的確切範圍 |
| OpenAI API key | external | 使用者自備、credential vault | 只供 Rust 原生 client 使用 |
| OpenAI Responses／Web Search | external | fixed endpoint、tool allowlist、store false | 搜尋、選擇並回傳 citations |
| 寵物語言 | supporting | 1–3 句、雙語、不得扭曲事實 | 將 insight 轉成可親近訊息 |
| 精簡證據卡 | supporting | why、source、date、searched_at | 讓使用者按需查證 |
| 資訊強度 | supporting | 3／2／1／0 | 防止把普通候選誇大成重大新聞 |
| CredentialVault | supporting | save／replace／delete／status | 保護 key，不讓 WebView 讀回 |

## 假設與釐清

| 原假設 | 如何被挑戰 | 最終決定 |
|---|---|---|
| 只有重大消息才說 | 使用者要求即使普通也要說 | 每天有有效脈絡就搜尋；以強度誠實降級 |
| 每次搜尋前都預覽 | 會破壞寵物的每日主動性 | 一次精確同意後自動送出；scope 擴張才重新同意 |
| 只送極小關鍵字最安全 | 使用者認為資訊太少會降低品質 | 送有用但受限的工作脈絡，排除原文與敏感細節 |
| 寵物訊息本身就夠 | 難以查證最新資訊 | 加精簡證據卡，但預設保持乾淨 |
| 一開始支援多家 API | 安全、費用與測試矩陣太大 | MVP 只支援 OpenAI BYOK，保留 provider interface |
| 搜不到好東西就沉默 | 使用者要求每天仍有交代 | 顯示最佳候選、平靜標籤或誠實失敗狀態 |

## 實體穩定度追蹤

| 輪次 | 實體數 | 維持 | 變動 | 新增 | 重點 |
|---:|---:|---:|---:|---:|---|
| 1 | 7 | 0 | 0 | 7 | 最近工作、每日搜尋與寵物訊息成立 |
| 2 | 8 | 7 | 0 | 1 | 新增資訊強度 |
| 3 | 11 | 8 | 0 | 3 | 新增 BYOK、搜尋脈絡與同意 |
| 4 | 12 | 11 | 0 | 1 | 新增最近工作判斷器 |
| 5 | 13 | 12 | 0 | 1 | 新增精簡證據卡 |
| 6 | 14 | 13 | 0 | 1 | 固定 OpenAI provider profile |

## 訪談全文

<details>
<summary>展開查看所有問答</summary>

### 第 1 輪：目標清晰度

**Q**：當每天只能主動一次時，理想情境中寵物看到什麼工作、搜尋什麼、最後怎麼告訴你？  
**A**：看到使用者以 Agent 寫程式，就分享最新 vibe coding 技巧；看到使用 MiniMax H3 製作影片，就分享最新好工作流或模型更新。  
**評分**：目標 80%／約束 60%／指標 35% -> 模糊度 40%。

### 第 2 輪：成功指標

**Q**：什麼內容才值得主動說；若沒有足夠有價值的內容，是否沉默？  
**A**：由 AI 盡量判斷有價值內容；真的不夠好也沒辦法，但仍要說。  
**評分**：目標 85%／約束 60%／指標 60% -> 模糊度 30%。

### 第 3 輪：約束清晰度

**Q**：可否把原始記憶送往外部 API，或需先縮成最小主題？  
**A**：可適當縮小以避免資訊太多，但不要縮得太小；API 與功能欄位要提醒，使用者確定後才開啟。  
**評分**：目標 88%／約束 78%／指標 60% -> 模糊度 23%。

### 第 4 輪：反面人模式

**Q**：一次同意後可每日自動送出，還是每次都需預覽？  
**A**：自動送出，由 AI 判斷最新工作來決定找資料方向。  
**評分**：目標 92%／約束 80%／指標 55% -> 模糊度 23%。

### 第 5 輪：成功指標

**Q**：是否需要可展開卡片，顯示來源、日期、搜尋時間與為何告訴使用者？  
**A**：好，但內容不要太繁複。  
**評分**：目標 94%／約束 72%／指標 82% -> 模糊度 16%。

### 第 6 輪：簡化人模式

**Q**：第一版只支援一種 BYOK 服務，還是一開始就支援多家？  
**A**：預設 OpenAI，並提供超連結讓使用者快速前往申請。  
**評分**：目標 96%／約束 90%／指標 84% -> 模糊度 9%。

</details>

## 下一個產品決策點

這份文件先固定產品方向與 MVP 邊界，但不授權實作或 live API 呼叫。下一步應先審查並決定：

1. 是否接受一般 OpenAI API 最長 30 天濫用監控保存的外送邊界；
2. 是否接受 Windows Credential Manager 作為第一版 BYOK 保存方式；
3. 是否接受「App 運行時、每天最多一次、失敗不自動重試」的費用上限；
4. 是否接受先做完全 synthetic Bundle 1，再接一次明確授權的付費 live smoke。

四項都接受後，再建立 Proposed ADR 與實作計畫；在此之前，Memoryling 的 runtime network／privacy 現況不變。
