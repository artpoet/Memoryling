# Agent 記憶連動變體規則 — 深度訪談產出

> 訪談日期：2026-08-12 | 輪數：5 | 最終模糊度：18.2% | 狀態：正常收斂
> Scope：決定 Memoryling 如何從已授權 Agent 記憶形成可辨識、可解釋且可重算的外觀變體
> Truth boundary：本文件是 PM 規格；目前產品尚未實作真實來源連接、活動分類、`MorphologyRecipe` compiler 或程序化寵物 renderer

## 清晰度分析

| 維度 | 分數 | 權重 | 加權 | 狀態 |
|---|---:|---:|---:|---|
| 目標清晰度 | 95% | 40% | 38.0% | 清晰 |
| 約束清晰度 | 86% | 30% | 25.8% | 清晰 |
| 成功指標 | 60% | 30% | 18.0% | 可接受 |
| **模糊度** |  |  | **18.2%** | 正常收斂 |

## 目標

Memoryling 應優先反映使用者經常用 Agent 做什麼，其次呈現共同累積的歷程與成果，最後才以較輕的動作或局部細節表達合作方式；使用者一眼能辨識主要傾向，混合經歷與形成原因則由詳細頁解釋。

優先順序固定為：

1. `AgentActivityPattern` — 使用者經常用 Agent 做什麼；
2. `AccumulatedJourney` — 使用者與 Agent 累積了哪些歷程與成果；
3. `CollaborationStyle` — 使用者如何與 Agent 合作。

| 資訊層 | 視覺責任 | 上限與衝突規則 | 缺少證據時 |
|---|---|---|---|
| A `AgentActivityPattern` | 主剪影、主要附肢、主要運動方向 | 永遠只有一個 dominant silhouette；混合型最多一個次要 structural modifier | 回到 neutral juvenile／上一個有效 canonical recipe |
| C `AccumulatedJourney` | maturity、stage、成果里程碑與具 lineage 印記 | 不得選擇或覆蓋 A 的主輪廓語彙；印記受 visual-slot cap | 保持既有 maturity，不因等待時間自行升階 |
| B `CollaborationStyle` | `motion_rhythm_accent`、姿態或一次短反應 | 最多一項局部節奏 accent，不得形成職業、人格或情緒標籤 | 使用 neutral rhythm modifier |
| 內容領域 | 材質、表面節奏、局部色偏或 pattern | 最多一項 secondary accent；不可新增主要器官或改 stage | `general／unknown`，不做可見推論 |

當 A、C、B 或內容領域的候選輸出互不相容時，依本表由上往下保留；低優先層不得擠掉高優先層。C 在 A 缺少證據時仍只保留／推進 neutral base 的 maturity 與成果印記，不自行選物種。超出 module compatibility matrix 時 fail closed，維持上一個有效 canonical recipe。

## 已確認的產品規則

### 1. 雙層分類

- **行為／成果層**共用一組已授權 evidence graph，但責任分離：A 行為活動單獨決定主輪廓、比例、主要附肢與主要運動方向；C 歷程成果只門控永久重塑、推進 maturity／stage 並形成里程碑印記。
- **內容領域層**只影響次要表現：表面節奏、材質、局部色偏、受限 pattern accent 或詳細頁說明，不直接決定物種、永久印記或職業。
- 內容領域可以自動衍生，不必逐筆核准；但不得把醫療、心理、政治、宗教、關係、創傷、情緒或其他敏感推論轉成可見分類。
- 任何分類都描述可觀察的 Agent 使用活動或成果，不描述「你是什麼樣的人」。

### 2. 一次同意，範圍內自動衍生

- 使用者第一次連接時選擇**一個明確來源**，並同意哪些資料類別可用於「活動模式、歷程成果、合作方式與內容領域的本機寵物成長」；同意記錄綁定 adapter／consent／mapping version。一個 `SourceConsentScope` 只對應一個 source；增加另一個來源必須建立另一個 scope 並重新同意。
- 同一 `SourceConsentScope` 內的新紀錄可以自動衍生與套用，不逐筆詢問。
- Memoryling 不掃描未授權位置、不寫回來源、不上傳記憶、不呼叫 runtime 圖片或語言模型。
- 使用者可以查看形成原因、修正活動／領域分類、關閉自動衍生、停用來源或忘記本機資料。
- 擴張來源位置或把既有資料用於新的用途時，必須重新取得範圍同意；規則更新不能靜默擴張用途。

### 3. 一眼讀主傾向，細節進說明

- 主要 `AgentActivityPattern` 應控制 220–260 px 下仍可辨識的剪影或主動作。
- 次要活動最多改變一組局部結構、材質或動態 accent，不能與主要傾向爭奪輪廓焦點。
- `AccumulatedJourney` 主要控制 maturity、stage、重要里程碑印記與 EvolutionBridge 歷史。
- `CollaborationStyle` 只控制受限的 `motion_rhythm_accent`、姿態或短反應；A 的 `primary_motion_family` 仍是主要運動方向，不得形成敏感人格標籤。
- 詳細頁說明「哪些已授權活動／成果類別造成哪些變化」，不在 pet surface 顯示來源名稱、專案名稱、原始文字或內容領域標籤。

### 4. 長期基底與近期狀態分層

- **長期歷程形成穩定基底。**永久 `EvolutionPathProfile` 與 `MorphologyRecipe` 由去重後、具 lineage 的成果證據重算。
- **近期改變先成為暫時反應。**最近使用傾向可改變姿態、視線、動作節奏、局部光感與暫時表面 accent，但不直接寫入 genome。
- **持續且有成果才重新塑形。**新傾向只有累積多個獨立、符合資格且具成果語意的 canonical evidence groups，跨過版本化 support／diversity 門檻後，才成為永久 profile contribution。
- 日曆天數、App 開啟時數、token 數、prompt 數、Agent 數量或原始紀錄數都不能提供永久成長分數。時間只用於 ephemeral TTL 與呈現，不是 XP。
- 不使用未保存亂數或只靠等待時間的 hysteresis。相同已授權 canonical event 集合、identity seed 與規則版本必須得到相同結果。

## 初始活動與成果詞彙表

這是供 synthetic prototype 使用的候選，不是固定物種清單；每個 axis 是影響向量，不與某一具身體一對一。

| Activity axis | 可觀察證據例子 | 主要視覺影響候選 | 不可推論 |
|---|---|---|---|
| `making` | 建立、實作、產出、交付、迭代成品 | 穩定主軸、構造片、抓持／組裝式動作 | 創造者人格、能力高低 |
| `inquiry` | 研究、比較、驗證、探索、建立理解 | 感知結構、開放負空間、掃視／探測動作 | 智者、先知、智商 |
| `stewardship` | 修復、整理、維護、歸檔、保持連續 | 包覆／承接結構、層理、收納／整理動作 | 守護者、控制欲、責任人格 |
| `exchange` | 協作、教學、回饋、交接、分享 | 對外展開結構、回應節奏、呼應式動作 | 外向、討好、關係品質 |

正式 schema 應允許一筆事件支持一個主要 activity axis 與有限的成果 qualifier，例如 `completed`、`validated`、`repaired`、`organized`、`shared`。單純出現動詞、不完整嘗試或無法確認成果的紀錄，最多形成近期 ephemeral hint，不直接形成永久 morphology。

## 內容領域的受限作用

內容領域不控制主要 anatomy。第一版可採有限、非敏感且可修正的 coarse domain，例如：

- software／automation；
- design／media；
- writing／communication；
- research／learning；
- planning／operations；
- general／unknown。

每個 domain 只能從 allowlist 選擇一種 surface family、material modifier 或局部 pattern vocabulary；它不能新增主要附肢、改變 maturity、決定 stage 或覆蓋 activity silhouette。unknown、低信心、互相衝突或敏感內容一律回到 `general／unknown`，不做可見領域推論。

## 變體組成規則

```text
IdentityCore
+ maturity stage from AccumulatedJourney
+ quantized AgentActivityPattern profile
+ bounded DomainInfluence
+ bounded CollaborationStyle
+ lineage-bearing WorldEffect marks
= deterministic MorphologyRecipe
```

每個 recipe 只可使用版本化 module catalog 與 compatibility allowlist：

- 一個主要 silhouette family；
- 最多一個次要 structural modifier；
- 一個 surface／material family；
- 一個由 A 決定的 `primary_motion_family`，加上最多一個由 B 決定的 `motion_rhythm_accent`；
- 最多一個 domain accent；
- 受現有 visual-slot cap 限制的 memory marks。

一隻寵物同時只能有一個主輪廓焦點、一種主要表面節奏與一個主要運動方向。超出相容矩陣或密度上限時必須 fail closed，保留上一個有效 canonical recipe；不可任意把所有高分模組疊上去。

## 發展與轉變規則

### Ephemeral hint

- 最近位於已同意範圍內、通過 allowlist 的合格紀錄可以產生 content-minimized activity hint。
- hint 有 TTL，只存在 render state，不寫入 `PathContribution`、genome、stage 或 Growth Journal。
- 冷啟動、來源停用、TTL 到期或證據失效時回到 neutral。

### Durable contribution

永久改變至少需要：

- 多個去重後的 canonical evidence groups；
- 可觀察的成果 qualifier，而非只有操作次數；
- 版本化 confidence／support 門檻；
- 至少一項跨 group 的 evidence diversity，避免同一件事被重複記錄後灌分；
- 完整 source → event → signal → contribution → recipe-decision lineage。

舊 `PathMappingV1` 只保留為隔離的 legacy regression，不是新 A／C／B gate 的正式契約。為讓下一輪能寫出唯一預期，synthetic prototype 使用新的 `AgentGrowthMappingPrototypeV2`；它仍是測試提案，不可接真實來源或被當成 Accepted production mapping。

### `AgentGrowthMappingPrototypeV2` 的 deterministic oracle

以下只供 synthetic fixtures；所有 key 與 enum 都由 fixture 明列，不從自由文字猜測：

- A axes 固定為 `making`、`inquiry`、`stewardship`、`exchange`；每個合格 canonical group 只能支持一個 A axis，提供 `1000` 整數 units，重複來源／紀錄只合併 lineage，不增加 units；單 axis cap `5000`。
- C outcome allowlist 固定為 `completed`、`delivered`、`validated`、`repaired`、`organized`、`shared`。不在 allowlist、未完成或 outcome 不明的 group 可形成近期 hint，但不通過 C gate。
- `canonical_group_key` 去重同一成果；`independence_key` 證明不是同一工作單位的副本；`evidence_period_key` 只證明證據分布於至少兩個已明列觀察區段。等待時間本身不建立 period、不增加 units。
- A profile 至少需要三個彼此獨立的合格 groups，並按互斥順序判定：① 至少三軸皆 ≥ `2000` 且所有合格軸的最高與最低差 ≤ `1000` 時輸出 `balanced-confluence`；否則 ② 前兩軸皆 ≥ `2000` 且差 < `1000` 時，輸出以 stable axis order 命名的 two-axis hybrid；否則 ③ 最高 axis ≥ `3000` 且領先第二名 ≥ `1000` 時為 dominant；否則 ④ 維持 neutral primary。第一個命中的 mode 即為唯一輸出，不再評估後續條件。
- C permanent gate 同時要求：至少三個去重且獨立的 outcome-qualified groups、至少兩種 outcome kinds、至少兩個 `evidence_period_key`，以及 maturity 至少為 `growing`。C 只決定是否可提交 A 形成的永久重塑、maturity 與 marks；沒有合格 A profile 時，即使 C gate 通過也維持 neutral primary。
- B 只從 allowlisted `motion_rhythm_accent` 選最多一項；domain 只從 allowlisted surface／material family 選最多一項。兩者不得加入 units、改 A mode、通過 C gate 或增加 visual slots。
- stable axis order 固定為 `making < inquiry < stewardship < exchange`，只用於 canonical serialization、hybrid ID 與 deterministic display order；同分不會被偷偷解讀成單一 dominant axis。
- 每個 fixture 使用 injected clock。新合格 in-scope event 可產生最長 `24h` 的 memory-only hint；process restart、scope disable／revoke、correction、forget 或 TTL 到期立即回 neutral，且不寫入 DB。

這些數字的作用是讓 prototype 可測，不代表使用者已核准 production thresholds。正式 connector 前必須以新的 production mapping ID 經 privacy review、profile matrix 與視覺 signoff；未知 mapping version fail closed。

### Permanent recipe change

- 跨過門檻後，以 transaction 原子產生新的 `EvolutionPathProfile`、genome revision、`MorphologyRecipe`、`RecipeLineageMap` 與 `EvolutionBridge`。
- 外型可大幅改變，但每個相鄰 recipe 必須保留一條生命／感知連續線與一條結構連續線。
- 近期 hint 可以預告可能長出的方向，但不能承諾一定進化；永久結果仍以 canonical evidence graph 為準。
- 若新傾向後來獲得更多成果，它逐步成為主要 profile；若沒有，暫時反應到期即可，不留下幽靈形態。

### Forget／correction／source disable

- 修正分類、停用來源或忘記資料後，必須先清除受影響的 `EphemeralActivityHint`，再只用仍有效的事件原子重算 contributions、activity profile、domain influence、maturity、genome、recipe、`RecipeLineageMap`、marks、bridges、journal、explanation 與 render caches。
- 部分來源仍支持同一 canonical group 時，只更新 lineage／confidence；最後支持歸零才移除 contribution。
- 遺忘前形態只可作短暫、content-free 的 transition buffer，不得持久化或留在日誌。
- 若重算後主要傾向改變，使用「重新成形」語言，不描述為退化、受傷、死亡或人格改變。

## 非目標

- 不建立固定職業、人格、MBTI、能力、道德或情緒分類。
- 不讓內容領域直接變成「程式寵物」「投資寵物」等固定物種。
- 不用近期一兩筆紀錄、使用時間或資料量快速換皮。
- 不讓每筆記憶增加一個零件。
- 不做背景自動掃描或未授權 process／session 監看。
- 不用 runtime AI 生圖或自由文字模型決定永久形態。

## 成功指標（可驗證）

- [ ] 在黑色 silhouette 與 220 px 實際尺寸下，每張只觀看 2 秒、不得看文字或配色提示；至少 12 位未參與設計的測試者，四種 pure synthetic activity profiles 的每類正確率皆需 ≥ 70%，任兩類單向 confusion 皆需 < 25%。混合 profile 不要求一眼說出精確比例；結果必須保留 confusion matrix。
- [ ] 同一 activity profile、不同 coarse domains 保持相同主要 silhouette，只在受限 surface／material／pattern 區域形成差異。
- [ ] 同一 identity 的近期 activity hint 只改 ephemeral render state；重啟或 TTL 到期不改 persisted recipe。
- [ ] 一筆或大量重複紀錄不能觸發永久重塑；符合門檻的多組成果證據可以確定性改變 recipe。
- [ ] 相同已授權 canonical event 集合、identity seed 與版本 tuple 在不同匯入順序、locale 與重啟後產生相同 recipe hash。
- [ ] 使用者能在兩步內看到主要變化的 aggregate 活動／成果原因，並可修正、停用或忘記來源。
- [ ] 忘記最後支持來源後，相關 recipe decisions、marks、bridges、journal 與 explanation 全部移除或由剩餘證據重算。
- [ ] pet surface、logs、screenshots 與 render DTO 不顯示來源名稱、專案名稱、原始文字或內容領域標籤。
- [ ] 未授權來源、unknown／sensitive domain、conflicting tags 與 future versions 全部 fail closed。

### 最小 synthetic acceptance matrix

| Case | 輸入 | 必須得到的結果 |
|---|---|---|
| A／C／B 衝突 | 三層都要求不相容 module | A 主形保留，C 改 maturity／mark，B 只留一項 motion accent |
| 純 A | 多組單一 activity 成果 | 一眼可辨的 dominant silhouette，不靠顏色或文字 |
| 無 A fallback | 只有歷程／合作證據 | neutral 主形；C 可更新 maturity／marks，B 可更新 motion，但兩者都不越權選物種 |
| 同分 | 兩個 activity axes 同 support | 依版本化 stable order／balanced rule，匯入順序不影響 hash |
| 近期一次 | 一筆合格新紀錄 | 只有 TTL hint，不改 persisted recipe |
| 持續無成果 | 多次活動但沒有 outcome qualifier | 不形成永久 contribution |
| 單次成果 | 一項完成成果 | 可顯示 hint／bounded mark，但不能重塑主形 |
| 持續且有成果 | 多個去重、獨立、合格成果 groups | 過 gate 後只提交一次 canonical recipe revision 與 bridge |
| 領域切換 | activity profile 不變、coarse domain 改變 | 主剪影不變，最多一項 surface／material accent 改變 |
| duplicate import | 同成果重複 1,000 次或跨來源出現 | 單一 capped contribution，保留多來源 lineage |
| partial／last-source forget | 先移除部分支持，再移除最後支持 | 前者只更新 lineage；後者完整移除或重算相關形態 |
| mapping migration | 同事件集切換 mapping version | 明示 migration、確定性重算；未知 future version fail closed |
| initial scope consent | 一個 source、adapter/version、data categories、purposes、mapping/consent versions | 建立一個 canonical scope revision；未同意前零 persistence／contribution |
| later in-scope record | scope 啟用後出現同 source、category、purpose、version 的新紀錄 | 不逐筆詢問；本機自動 normalize／derive，保留 scope lineage |
| new source expansion | 嘗試把第二個 source 放進既有 scope | fail closed；建立另一個 scope 並重新 preview／同意前零貢獻 |
| category／purpose expansion | 同 source 出現未同意 category 或新用途 | fail closed；新 scope revision 同意前零貢獻 |
| correction | 使用者修正 activity／domain／outcome category | 清 hint 並原子重算全部 downstream state，輸出唯一新 revision |
| scope disable／revoke | 使用者停用或撤回 scope | 立即停止新衍生、清 hint，以剩餘 scopes 原子重算；不修改來源 |
| transaction rollback | 任一重算／commit 步驟故障 | 保留完整舊 canonical revision，UI 不宣稱成功，不留部分 row／ghost module |

## 關鍵實體

| 名稱 | 類型 | 關鍵屬性 | 關係 |
|---|---|---|---|
| `SourceConsentScope` | core | one source ID、adapter ID／version、canonical sorted data-category／purpose sets、enabled、consent-schema／mapping version、revision、consented-at、canonical scope fingerprint | 控制哪些事件可自動衍生；新增 source 另建 scope，category／purpose／mapping 語意擴張建立新 revision 並重新同意 |
| `AgentActivityPattern` | core | axis weights、mapping version、lineage | 主導 silhouette 與主要 motion |
| `AccumulatedJourney` | core | maturity evidence、milestones、stage | 主導生命階段與重要印記 |
| `CollaborationStyle` | supporting | bounded motion／pose modifier | 只做次要動態修飾 |
| `DomainInfluence` | supporting | coarse domain、confidence、surface module | 只影響受限表面語彙 |
| `EphemeralActivityHint` | supporting | allowlisted axis、TTL、neutral fallback | 只進 render state |
| `MorphologyRecipe` | core | module IDs、quantized parameters、versions、hash | 永久可重算外形 |
| `RecipeLineageMap` | core | decision → contribution／signal IDs | 支援 explain／forget，不進 pet DTO |
| `EvolutionBridge` | supporting | before／after recipe、transform grammar | 保持相鄰變化連續 |
| `CorrectionControl` | supporting | override、disable、forget | 觸發完整重算 |

## 假設與釐清

| 原假設 | 如何被挑戰 | 最終決定 |
|---|---|---|
| 寵物可以同時平均呈現所有 Agent 使用資訊 | 會造成視覺與語意無主次 | A 使用行為 > C 歷程成果 > B 合作方式 |
| 外觀只要每隻不同即可 | 無法形成可讀的使用連動 | 主傾向一眼可懂，混合與細節進詳細頁 |
| 內容領域可以直接決定外型 | 容易變成職業標籤或洩露內容 | 行為主形、領域只做次要表面語彙 |
| 每次衍生都需要核准 | 會打斷桌面生命感 | 來源與用途一次同意，範圍內自動衍生 |
| 本機抽象化等於完全沒有風險 | 截圖、誤分類與用途擴張仍存在 | 不逐筆核准，但保留範圍、修正、停用、忘記與 fail-closed |
| 最近使用應立刻改變永久外型 | 會頻繁換皮且鼓勵資料量 | 長期基底；近期先 ephemeral；持續且有成果才永久重塑 |

## 實體穩定度追蹤

| 輪次 | 實體數 | 維持 | 變動 | 新增 |
|---|---:|---:|---:|---:|
| 1 | 3 | 0 | 0 | 3 |
| 2 | 6 | 3 | 0 | 3 |
| 3 | 9 | 6 | 0 | 3 |
| 4 | 12 | 9 | 0 | 3 |
| 5 | 12 | 12 | 0 | 0 |

## 訪談全文

<details>
<summary>展開查看五輪問答</summary>

### 第 1 輪（目標清晰度）

**Q：** 寵物應優先反映經常用 Agent 做什麼、共同歷程成果，或合作方式？

**A：** 三者都要，順序 A 使用行為、C 歷程成果、B 合作方式。

**評分：**目標 82%／約束 70%／指標 42% → 模糊度 33%。

### 第 2 輪（成功指標）

**Q：** 外觀應讓使用者一眼看出活動傾向，還是只感覺每隻不同？

**A：** 主傾向一眼可懂；混合經歷與細節需進詳細頁。

**評分：**目標 88%／約束 58%／指標 80% → 模糊度 23.4%。

### 第 3 輪（約束清晰度）

**Q：** 分類應依行為成果、內容領域，或採雙層制？

**A：** 雙層制；內容領域不需要逐筆核准，因為本機且抽象呈現。

**評分：**目標 92%／約束 58%／指標 80% → 模糊度 21.8%。

### 第 4 輪（反面人模式）

**Q：** 不需核准是一次來源／用途同意後自動衍生，還是可未經詢問自動掃描？

**A：** 選 A：來源與用途同意一次，之後範圍內自動本機衍生。

**評分：**目標 92%／約束 80%／指標 52% → 模糊度 23.6%。

### 第 5 輪（成功指標）

**Q：** 長期歷程與最近一兩週使用傾向不同時，永久外型應聽誰的？

**A：** 選 C：長期基底；短期先改暫時反應；持續一段並累積足夠成果後才逐步重塑永久外型。

**評分：**目標 95%／約束 86%／指標 60% → 模糊度 18.2%。

</details>

## 與既有設計的關係

本文件細化現有[可成長寵物系統設計稿](deep-interview-evolving-creature-system-2026-08-11.md)與 proposed [ADR-0004](../adr/0004-deterministic-content-derived-evolution-paths.md)，不取代它們。若兩者尚未同步本輪決策，應以本文件記錄的使用者答案作為後續提案輸入，但 ADR 在 synthetic fixtures、privacy review 與視覺驗收完成前仍維持 Proposed。

## 建議下一步

1. 以本文件的 19-case acceptance matrix 建立 synthetic event／activity／domain／consent／transaction fixtures，不接真實記憶；
2. 驗證四個 activity axes 是否涵蓋足夠常見的 Agent 使用行為；
3. 為每個 axis 設計 silhouette／motion vocabulary，而不是固定寵物；
4. 用 module compatibility matrix 生成 pure、mixed、balanced 與 domain-modified recipe examples；
5. 完成小尺寸、相鄰 EvolutionBridge、explain、correction 與 forget/rederive 驗收後，才決定 Accepted schema。
