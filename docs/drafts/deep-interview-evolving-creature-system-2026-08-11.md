# Memoryling 可成長寵物系統設計稿

> Status: Product design draft — user-confirmed product direction; proposed implementation details; not implemented
> AS_OF: 2026-08-12 (Asia/Taipei)
> Scope: creature growth, evolution, rendering, explainability, and forgetting
> Truth boundary: the shipping app still has only the fixture-derived completion star and CSS creature

## 1. 一句話定義

Memoryling 不是每次啟動時重新生成的一張寵物圖片，而是一個由使用者核准記憶所塑造、能大幅進化、讓相鄰階段保持可理解聯繫，並可從來源完整重算的本機生命系統。

## 2. 已確認的產品決策

本輪訪談已確認下列方向：

1. **混合型生命邏輯。**穩定身份與記憶造成的長期變化可以持續存在；表情、活力、晝夜與季節狀態可以變動。
2. **永久成長只由核准記憶與其本機衍生資料驅動。**日曆時間本身不增加等級，也不會讓閒置中的寵物憑空進化。
3. **時間只驅動可逆狀態。**時間可以影響晝夜、季節、紀念日呈現與當下動作，但不能直接改寫永久基因。
4. **變化自動套用。**記憶已經通過來源核准後，使用者不必逐項批准每次外觀改變；每項變化仍必須可以解釋與撤銷來源。
5. **允許大幅進化，以相鄰階段維持聯繫。**第一階與最後一階不必一眼看出是同一種外型；但每次 `stage N → N+1` 都必須能看懂哪些特徵被保留、成長、分裂、合併、移位或退場，形成不跳接的演化鏈。
6. **成長核心不依賴 runtime AI API。**AI 或 Skill 可以協助開發期的造型探索；實際執行時使用本機規則、本機狀態與隨 EXE 打包的資產。未來即使加入對話模型，其輸出也不能直接回寫永久 genome、stage 或 lineage。
7. **共享視覺 DNA，許多受限的內容衍生變種。**目前概念圖只提供家族語彙、剪影範圍、材質與相鄰橋接參考，不代表預先畫好的固定寵物清單。永久形態不鎖成單一五階直線，而是由核准記憶形成的版本化 signals、有限本機模組與量化參數確定性組成。變種不得直接由原始文字、敏感人格標籤、runtime 模型或未保存亂數決定。

## 3. 產品承諾

### 使用者應感受到

- 這是同一個生命逐步成長，不是每次隨機換皮。
- 重要記憶會留下看得見、能被理解的影響。
- 大幅進化帶來驚喜；遙遠階段可以差異很大，但相鄰變化不會像毫無原因地換成另一隻生物。
- 遺忘來源後，相關變化確實消失或由剩餘記憶重新形成。
- 幾天沒有開啟 Memoryling 不會受到懲罰，也不會錯過不可逆成長。

### 不應發生

- 不把記憶筆數直接當成 XP，避免鼓勵匯入更多私密資料。
- 不因負面內容把寵物變醜、生病或用罪惡感逼使用者回來。
- 不從記憶擅自推論醫療、心理、政治、宗教或其他敏感人格標籤。
- 不讓視覺隨機數成為永久狀態的真相來源。
- 不為了產生新外觀而上傳記憶或呼叫遠端圖片模型。

## 4. 核心資料流

```text
approved MemoryEvent
    → versioned DerivedSignal
        ├─→ PathContribution → deterministic EvolutionPathProfile ─┐
        ├─→ GrowthContribution → structural genome axes ──────────┴─→ CreatureGenome
        └─→ WorldEffect → marks / habitat / story projections

IdentityCore + stage + CreatureGenome + versioned local module catalog
    → bounded deterministic MorphologyRecipe + EvolutionBridge

MorphologyRecipe + active WorldEffects + EphemeralState
    → render-safe CreatureState
    → local renderer + Growth Journal / Why did this happen?
```

SQLite 中的核准事件與來源鏈是事實來源。`CreatureGenome` 是可以重新產生的衍生快照，不可取代事件與 lineage。

`PathContribution`、`GrowthContribution` 與既有 `WorldEffect` 是同一個 `DerivedSignal` 的平行投影：第一個只形成有限的 profile influence，第二個只處理其他結構性 genome 軸，第三個延續現有 completion-star 並處理印記、棲地、故事等離散效果。三者共同組合成 `CreatureState`，不可形成 `WorldEffect → Genome → WorldEffect` 的循環依賴；recipe accent 也不能冒充具獨立 lineage 的 memory mark。

`EvolutionPathProfile` 是有限、版本化且可重算的多軸影響向量，不是固定支系選擇器，也不是把使用者永久分成某種人格或職業。它只接受具 lineage 的安全活動 signals；各軸經量化後影響 `MorphologyRecipe`，不與某一具身體一對一綁定。具來源意義的可見印記仍只能來自 `WorldEffect`。忘記來源後，profile、genome、recipe 與相關 bridge 必須一起重算。

### 四種狀態必須分離

| State | 來源與責任 | 永久狀態規則 |
|---|---|---|
| `IdentityCore` | 本機初次建立的 creature ID、identity seed、穩定名稱與演化根節點 | 不由記憶決定；刪除全部記憶後仍回到同一 identity baseline，但不要求所有階段共享固定外觀 |
| `DerivedGrowth` | 目前核准事件、identity seed 與版本化規則 | 可持續但非不可逆；忘記來源後必須完整重算 |
| `EphemeralState` | 本機時鐘、季節、當下 UI 狀態與未來可選的 content-minimized `LiveAgentPresence` | 不得推動永久成長，也不得成為 genome 的隱藏輸入；預設中性，必須到期或在狀態消失時清除 |
| `HistoryAndAudit` | 成長揭露與解釋索引 | 忘記來源後須刪除相依內容；不得殘留可反推出來源的文字、hash 或特徵 |

只有核准、遺忘、明示的人工修正或版本 migration 能改變永久狀態。啟動 App、時鐘、locale、動畫完成或未來模型輸出都不能直接或間接改變它。若未來模型提出成長候選，它必須先被當成新的不可信來源，經獨立產品決策、明確使用者核准與 machine-readable lineage 後才可能成為輸入；在此之前一律不可進入永久成長圖。

### Agent 使用狀態的兩層邊界

- `ApprovedAgentActivityEvent`：來源、records 與可用活動類別都必須經使用者明確選取、preview 與核准；tag 再由核准範圍內的 connector-declared 或 user-confirmed 有限枚舉提供，並保存 source／event／derivation lineage。範圍外 tag 一律為 `0`。經去重、門檻與上限後，證據才可影響永久 profile、genome 與 morphology。
- `LiveAgentPresence`：目前未實作，預設為 neutral。未來若另作產品／隱私決策並取得明確同意，只能由 source-specific read-only adapter 提供 allowlisted、content-free enum。狀態只留在記憶體、必須有 TTL，不寫 SQLite／logs／telemetry；停用、失聯或 TTL 到期就立即清除。它只能影響表情、姿態、呼吸、移動速度與光感，不寫入 contribution，也不改 stage、genome 或永久 recipe。

禁止把原始 prompt、記憶全文、Agent／專案名稱、路徑、token 數、開啟時數、情緒或人格推論當成 morphology 輸入。若未來要把某種即時狀態轉成永久證據，必須先正規化為可預覽、可選取、可核准且具 lineage 的 durable event，不能靠背景監看偷偷累積。

### GrowthContribution

每一項永久成長貢獻至少包含：

- opaque contribution ID；
- derivation ID 與 derivation version；
- 一個或多個來源 signal ID；
- 影響的 genome axis；
- direction、magnitude 與 confidence；
- explanation key；
- deterministic merge rule；
- forgetting 時的重算行為。

不可把原始記憶文字或可識別內容直接塞進渲染層。

### PathContribution

每一項路線貢獻至少包含：

- opaque contribution ID；
- derivation ID／version 與 path-mapping version；
- 一個或多個來源 signal ID；
- 有限枚舉的 path axis ID；
- 整數 support units、confidence bucket 與 per-group cap；
- tag provenance（例如 connector-declared 或 user-confirmed）；
- canonical group key 與 deterministic merge rule；
- forgetting 時的重算行為。

`PathContribution` 不得攜帶原始文字，不得直接產生 memory mark，也不得和其他 genome axis 共用未版本化的自由字串。

### CreatureGenome

建議先使用少量、可測試的軸，而不是無限制自由生成：

| Genome axis | 影響 | 邊界 |
|---|---|---|
| maturity | 生命階段與整體複雜度 | 由記憶意義與穩定訊號決定，不看日曆天數 |
| morphology | 體型、比例、附肢、姿態 | 可大幅改變，但每次 stage transition 必須產生可解釋的 EvolutionBridge |
| surface | 色盤、紋理、發光、材質感 | 顏色不可是唯一資訊載體 |
| baseline_motion_style | 呼吸節奏、探索動作、待機姿態 | 只描述可觀察的動作風格，不下敏感人格診斷 |
| memory marks | 星點、符號、飾物、疤紋式故事印記 | 每一項都要有 effect lineage 與可見數量上限 |
| habitat affinity | 與棲地物件、光線、植物或收藏的關係 | 屬後續 slice，不阻擋核心寵物渲染 |

## 5. 生命階段、內容變種與大幅進化

以下五段是供 prototype 驗證的**提案**，不是已核准 schema；名稱與段數可在視覺測試後調整。若採用，資料層應使用穩定 enum：

| Internal stage | 工作名稱 | 可見變化 |
|---|---|---|
| `seed` | 記憶種 | 幾乎沒有歷史，輪廓最簡潔，建立 identity root 與第一組可演化特徵 |
| `awakened` | 初醒 | 形成第一個完整身體與基礎動作性格 |
| `growing` | 成形 | 比例、附肢、紋理與移動方式開始分化 |
| `evolved` | 蛻變 | 允許明顯剪影與能力感變化，棲地開始回應牠 |
| `legacy` | 守憶 | 高度個人化但不雜亂，重要印記形成有秩序的整體設計 |

階段門檻由版本化規則判定，考慮去重後訊號的意義、受上限約束的證據強度、語義多樣性與穩定度。來源數量本身不得提供加成；單純連接更多來源或匯入大量相似紀錄不能快速升階。

### 共享視覺 DNA 與多變種空間

目前已確認的高層視覺方向是：具有眼神與生命感的有機角色，結合神聖、精緻但不宗教化的高級折面／材質語言。用來驗證這個方向的 proposed family grammar 是：

- 幼體以有虹膜深度、眼瞼重量與穩定眨眼節奏的生物眼建立生命感；遠端階段可以轉換眼部形態，但相鄰 bridge 必須保留或承接視線／眨眼／感知節奏，而不是突然改成第三眼或無關符號；
- 薄荷色 memory seed 是初期候選 anchor，可被保留、成長、分裂、合併、移位或由明確 successor 承接，不是永遠固定在同一解剖位置的 schema 欄位；
- 會呼吸、收縮與舒展的有機膜／葉片結構；
- 以珍珠母、瓷質折面、內嵌光縫與克制金屬邊表達高級感，不使用宗教符號、巨大光環、皇冠或神諭式文案；
- 紫丁香、薄荷與深靛藍 ancestry，但任何變種都不能只靠顏色識別。

目前 ImageGen 概念形態只用來抽取 shared family grammar、剪影差異與相鄰 `EvolutionBridge` 規則；它們不是固定職業、固定終局或 production sprite roster，也不限制正式系統只能產生圖中幾種外型。

`EvolutionPathProfile` 採 **weighted influence and bounded composition**，不是一次選定後永遠鎖死的 branch。P0 可用的安全 activity axes 仍是提案，初始候選為 `craft`（製作／交付）、`inquiry`（學習／研究）、`stewardship`（修復／整理／維護）與 `exchange`（協作／教學／分享）。這些軸只能描述核准內容中可觀察的活動，不得推論「你是創造者／智者／守護者」、情緒、醫療、心理、政治、宗教或其他敏感人格特徵；任何 axis 都不能直接等於一具固定外型。

P0 的 signal 來源優先順序是 connector 提供的明確結構化標籤，其次是 import preview 中由使用者確認的有限枚舉標籤。沒有可信標籤的事件仍可留下受上限約束的 mark，但不能改變 morphology。自由文字分類、embedding、sentiment branch、本機 LLM 與遠端模型都不屬 P0。

每個 canonical memory group 的 path contribution 有上限；同內容跨來源先去重但保留多來源 lineage。P0 提案使用 `canonical_group_key = SHA-256(event_schema_version || event_kind || canonical_content_hash)`。同 group 只貢獻一次並保存所有支持 event／signal IDs、tag provenance 與 mapping version；相同 tag 可合併 lineage，不同有效 tag 衝突時 morphology 必須 fail closed，待使用者在 detail 內解決，不能依來源順序挑一個。忘記部分支持來源時 group 保留並更新 lineage；最後一個支持來源消失時才移除 contribution 並重算 profile。

### Proposed PathMappingV1 for synthetic prototyping

以下數字是可實作的 synthetic prototype contract，仍需測試與使用者 signoff 才能成為 Accepted schema：

- 只在 synthetic `PathMappingV1` fixtures 內先固定 `craft`、`inquiry`、`stewardship`、`exchange` 四個 axes；未知 tag fail closed，axis 與外型不是一對一；
- 每個 canonical group 對一個 axis 產生整數 support units，不使用浮點；相同 tag 的多個 supports 取最高 bucket、保留全部 lineage，但 group 仍只貢獻一次；單 group 上限 `1000`；
- 每個 axis 聚合上限 `3000`；先依 canonical group key 排序、去重、cap，再評估 profile；
- morphology 只有在 maturity 至少 `growing`、有效 canonical groups 至少 `3`、最高 axis 至少 `1500` 時啟用，確保一筆記憶不能決定永久 recipe；
- 若至少三個 axes 都達 `1500` 且最高與最低差不超過 `500`，mode 為 `balanced-confluence`；
- 否則若前兩軸都達 `1500` 且差小於 `750`，mode 為雙軸 hybrid；
- 其餘情況以最高軸為 dominant；第二軸達 `1000` 時最多提供一個不含 lineage mark 的材質／動作 accent；
- 同分只用明文 axis ID 排序決定輸出順序，不把同分硬解讀成單一勝者。

PathMappingV1 的 support table：

| Tag provenance／confidence | Support units | 規則 |
|---|---:|---|
| `user-confirmed` | 1000 | 使用者在 import preview 明確選定有限枚舉 tag |
| `connector-declared`, confidence `9000–10000` bps | 750 | connector 必須提供版本化結構化 tag 與 confidence |
| `connector-declared`, confidence `7500–8999` bps | 500 | 同上 |
| `connector-declared`, confidence `6000–7499` bps | 250 | 同上 |
| missing confidence or below `6000` bps | 0 | 不形成 path contribution；可在 detail 提示使用者確認 |

本機 lexicon 只能提出候選；候選在使用者確認前是 `0`，確認後走 `user-confirmed = 1000`。同一 canonical group 若出現不同有效 tags，即使 confidence 不同也整組 fail closed，不能用高分蓋掉衝突。

這讓相同 stage 可以由多軸 profile 編譯出許多不同剪影，也允許來源改變後確定性重算，而不造成無限組合或一筆記憶瞬間換物種。正式 mapping 表、bucket 分配與 axis 名稱仍可在 ADR-0004 接受前調整；任何調整都要提升 mapping version 並重跑 fixtures。

### 相鄰階段 EvolutionBridge

身份連續性是一條**演化路徑**，不是要求所有階段永遠掛著同一組眼睛、耳朵或顏色。每次 `stage N → N+1` 都要產生版本化 `EvolutionBridge`，記錄一個或多個可見／可感知特徵如何轉換：

- `preserved`：保留但比例或細節改變；
- `grown`：從小型特徵長成主要結構；
- `split`／`merged`：一個特徵分化，或多個特徵融合；
- `relocated`：例如胸口光點轉移成尾端星環；
- `retired`：舊特徵退場，但由新形態或行為承接其功能與原因。

可用來建立橋接的元素包括臉部節奏、核心記憶種、色彩、高光、剪影、附肢、材質、移動方式與招牌動作。遠距階段並排時不一定能直接認出來；但沿著相鄰階段查看時，不能出現沒有 lineage 或形態轉換說明的「瞬間換物種」。

`EvolutionBridge` 由前後兩個 deterministic stage snapshot 與版本化規則計算，不是另一個不可回算的歷史真相。Growth Journal 保存可理解的演化路徑；忘記來源而改道或退階時，相關 bridge 也必須由剩餘事件重算。reduced-motion 模式以 before／after 圖與文字列出轉換，不強迫播放 morph 動畫。

recipe 改變同樣必須產生 bridge。生命感可以由視線、眨眼、呼吸或感知動作中的至少一項承接，不要求永遠保留同一雙眼；memory seed 與既有護殼、翼片、感知帆或披膜都可 `preserved`／`grown`／`split`／`merged`／`relocated`／`retired`，但退場時必須記錄可理解的 successor。產品不可在兩個成熟變種間瞬間換皮，也不可把改變描述成受傷、退化或人格改變。

## 6. 永久、持續與暫時狀態

| 狀態類型 | 例子 | 是否寫入衍生狀態 | 如何移除 |
|---|---|---:|---|
| structural | stage、morphology、核心色盤 | 是 | 刪除來源後重算 genome |
| persistent effect | 記憶印記、動作傾向、棲地物件 | 是 | 刪除相依 effect 或由剩餘 signal 重算 |
| ephemeral | 晝夜表情、季節光線、暫時活力 | 否，或只存使用者設定 | 重新計算當下 render state |
| presentation | 新進化揭露動畫、已讀狀態 | 只存 UI revision／ack | 動畫結束或使用者略過 |

本機時鐘異常、時區切換或長時間未開啟不可造成永久成長、倒退、資料損失或懲罰。

若「紀念日」日期來自核准記憶，日期資格本身必須先成為具 lineage 的 persistent effect；本機時鐘只能在符合日期時啟用 ephemeral overlay。刪除該來源後，資格與 overlay 都要消失。

## 7. 自動變化與解釋體驗

核准匯入完成後，推導可自動套用。為避免突然變形造成困惑：

1. store 先完成事件、signal、effect、genome、lineage-bearing `MorphologyRecipe`、EvolutionBridge、journal／explanation projection 與 revision 的原子交易；
2. UI 取得新的 revision；
3. 一般變化以短而柔和的轉場呈現；
4. stage 改變以可略過的「蛻變時刻」呈現；略過的只有動畫，canonical genome 與文字摘要已經完成更新；
5. Growth Journal 增加一筆不含原始私密文字的摘要；
6. 使用者可從寵物或日誌開啟「Why did this happen?」。

若 revision 改變 `EvolutionPathProfile`，揭露摘要只能描述「哪些核准活動 signals 讓形態重新形成」，不能宣稱使用者屬於某種人格、命運或價值階級。

解釋最少要顯示：

- 發生了什麼變化；
- 哪一類核准訊號造成；
- 預設只顯示非敏感的 aggregate 支持數與 signal 類別；來源名稱只有在未來另行設計並驗證的本機 source-detail unlock gate 內才可顯示；
- derivation version；
- 刪除哪個 Memoryling 本機匯入來源會重新計算它。

自動套用不等於不可控制。後續人工修正應成為版本化的 `LocalGrowthOverride`：與來源衍生判斷綁定的修正，在相關來源被忘記後預設一起失效；純美術偏好則屬 `IdentityCore` 或 presentation 設定，不能冒充記憶因果。任何「隱藏動畫」或 presentation mode 都只能改變呈現，不能讓畫面長期停在與 canonical genome 不同的舊形態。

## 8. 遺忘與回算

遺忘不能只把印記從畫面隱藏。正確流程是：

1. 開啟單一本機 transaction，讀取目前 canonical `beforeRecipe` 的 render-safe projection，僅供成功 commit 後的短暫轉場；
2. 移除或失效指定 source／record／event，清除其相依 signals、effects、path profile、genome snapshot、recipe decisions、`MorphologyRecipe`、`RecipeLineageMap`、EvolutionBridge、journal／explanation projections 與 render caches；
3. 從仍存在的核准事件重新推導上述全部永久狀態與 lineage，得到 canonical `afterRecipe`；
4. 只用目前仍核准的事件重建持久 EvolutionBridge graph，原子寫入 `afterRecipe`、`RecipeLineageMap` 與新的 canonical genome／recipe revision；
5. commit 後才更新 UI；交易失敗則保留前一個有效 canonical revision；
6. renderer 只接受新 revision，不保留幽靈 layer。若遺忘前後需要視覺轉場，只能從 before／after 的 render-safe projection 產生 memory-only、TTL-bound 的 `BridgeFrameRecipe[]`；不得寫入 DB／log／journal，動畫結束、略過、重啟或 TTL 到期就清除。

核准或 migration 的一般 recipe 變化，也必須在同一 transaction 內先取得 canonical `beforeRecipe`，再推導 `afterRecipe`，依固定 `bridge_rule_version` 計算差異後原子寫入 recipe、lineage map、bridge／journal projections 與 revision。只有兩端都能由目前核准圖重建的 bridge 才可持久化；忘記來源後，不得以歷史 bridge 或 before recipe 留下被刪來源的形態證據。

若重算造成階段降低，產品文案應描述為「重新成形」或「記憶已重新整理」，不可用失敗、死亡或懲罰語言。來源工具的檔案永遠不被修改。

多來源效果必須是完整圖，而不是假設一個 effect 只有一個來源。移除其中一個支持來源時，如果剩餘 signals 仍符合規則，effect 可以保留，但 explanation 與 confidence 必須改寫；支持降到門檻以下或歸零時，形態、階段、棲地、故事、對話事實、提醒候選、快取、日誌與解釋索引都要一起刪除或重算。兩個 connector 匯入相同內容時，去重與 canonical ordering 不得造成重複升階。

## 9. 視覺密度與防雜亂規則

不是每一段記憶都增加一個配件。建議：

- 同類訊號先聚合成紋理、光澤或形態強度；
- 畫面同時只顯示有限的 hero marks，初始建議上限 5 個；
- 其餘有效 effects 保留在 Growth Journal，renderer 依穩定規則選擇代表項；
- 同一來源或同一主題不能壟斷全部可見 slots；
- stage 改變時重新編排印記，而不是把舊裝飾無限堆疊；
- tie-breaking 使用穩定 ID 與版本化排序，不能使用未保存的亂數。

## 10. 本機渲染架構

### 建議 v1：分層 React SVG

```text
IdentityCore + stage + CreatureGenome + quantized EvolutionPathProfile
    + versioned local module catalog
    → MorphologyRecipe

MemorylingCreature
├─ SignatureLayer
├─ BodyMorphLayer
├─ StructuralModuleLayer
├─ SurfacePatternLayer
├─ SeamLayer
├─ MemoryMarkLayer
├─ ExpressionLayer
└─ AmbientLayer
```

採用原因：

- 不需要 runtime AI API 或網路資產；
- 可沿用目前 React／Tauri 架構；
- SVG 向量能支援大幅形變、縮放與分層印記；
- effect layer 能攜帶 opaque effect ID，容易測試新增與遺忘；
- CSS 或 Web Animations API 足以完成呼吸、眨眼、耳尾微動與轉場；
- DOM 外層可保留鍵盤、螢幕閱讀器與 reduced-motion 控制。

內部 `MorphologyRecipe` 至少包含 `recipe_schema_version`、compiler／catalog versions、stage、genome revision、canonical `recipe_hash`，以及版本化 module instances。每個 instance 具有穩定 `instance_key`、`family_id`、`semantic_role`、量化幾何／材質／持續動作參數與 bounds；recipe 全體最多一個非 mark accent。另有只存在 Rust／授權 detail boundary 的 `RecipeLineageMap`，把每個 instance／parameter bucket 映射到排序後的 supporting contribution／signal IDs、derivation version 與 explanation key。module catalog 必須有 compatibility allowlist、visual-slot caps 與 fail-closed baseline。這可產生許多但有限、可枚舉測試的組合，不是固定幾張圖，也不是每個零件無限制自由排列。

未知 recipe／catalog／module version 或不相容組合必須讓 compilation／transaction 失敗並保留上一個有效 canonical recipe，不能靜默把 baseline 寫成新的永久外型。只有從未有過有效 recipe 時，UI 才可暫時顯示 reviewed baseline 與錯誤狀態；這個 fallback 不持久化，也不可宣稱衍生成功。

renderer 只接收 `CreatureRenderState`，不接收原始記憶內容、來源路徑或自由文字。相同 approved event set、identity seed、所有 derivation／mapping／genome／catalog 版本與明示 override 必須得到相同的 recipe、結構與外觀參數。

`SignatureLayer` 保存目前 stage snapshot 已經算好的家族視覺文法與生命節奏，不要求遠端階段共享固定器官。profile axes、權重與 activity labels 留在 Rust／aggregate explanation boundary。Agent identity 不是 profile／recipe input；若作為 source metadata 保留，只能在未來另行驗證的 source-detail unlock gate 內顯示，忘記來源後必須刪除或更新。pet renderer 只接收從 recipe 投影出的最終 visual-module IDs、受限 quantized parameters 與 revision，不接收 recipe lineage、原始文字、人物／專案名稱或人格摘要。

### 後續選項

- **Rive：**若角色骨架與形變品質超過 SVG 動畫的維護能力，可用本機 `.riv`、本機 WASM 與 data binding 接同一份 `CreatureRenderState`。採用前需確認 runtime export 授權、CSP、離線載入、二進位資產 review 與測試策略。
- **PixiJS：**只有棲地成為大量粒子、shader、多角色與複雜 scene graph 時才評估；單一寵物階段不值得先承擔 Canvas／WebGL 與 accessibility 成本。
- **PNG 主體＋SVG overlay：**可作短期視覺原型，但不適合作為大幅 morphology evolution 的最終架構。

## 11. AI、Skill 與外掛的正確角色

Skill 是開發工作流，不是 Memoryling 的 runtime 依賴：

- OpenAI 官方 `hatch-pet` 可協助製作固定 8 × 9 動畫圖集，但其核心仍是開發期 ImageGen，且固定 atlas 無法自行承擔連續 genome evolution。
- Product Design／ImageGen 可探索共享 family grammar、相鄰 transition 與 profile-matrix 極端值；通過人工 signoff 後再轉成可維護的本機 SVG 模組或資產。探索圖不直接定義 production route roster。
- 動畫設計 Skill 可協助 timing、easing、anticipation 與 reduced motion，但不能替代 lineage、genome 與 forgetting engine。
- 第三方 SVG／Pixi Skill 在安裝前必須檢查來源、license、程式碼與網路行為；目前設計不需要先安裝任何第三方 Skill 才能開始。

開發期產生的參考圖若進入產品，必須保存可重生來源、授權資訊、透明背景與小尺寸可讀性驗收。成品 App 不得為了寵物變化而呼叫圖片生成 API。

## 12. Accessibility 與舒適度

- SVG 視覺本體可設 `aria-hidden`，由外層原生控制提供角色狀態摘要與 Growth Journal 入口。
- 所有永久變化提供文字摘要；顏色、閃爍或位置不可是唯一辨識方式。
- 尊重 `prefers-reduced-motion`；大幅進化需可略過，reduced-motion 模式直接切換到穩定 end state。
- 禁止快速閃爍、不可停止的強烈粒子與用動作懲罰閒置。
- 鍵盤使用者必須能查看原因、日誌與忘記來源的結果。
- 自動大幅進化不可搶走 focus；螢幕閱讀器只做一次適當的 live announcement。
- 高對比、色覺差異與 200% zoom 需要獨立視覺驗收。

## 13. 隱私與安全邊界

- renderer、動畫與視覺測試只使用 synthetic genome fixtures。
- 不在 DOM attribute、CSS class、console、截圖或測試 snapshot 放入真實記憶文字。
- 不新增遠端字型、CDN、Rive cloud asset、遠端 WASM、遙測或圖片 API。
- Growth Journal 預設只顯示衍生摘要。來源細節需要未來另行設計的本機解鎖與隱私控制；目前產品沒有可供沿用的獨立 source-detail gate。
- 忘記一個來源後，相關 journal／audit 內容也必須刪除；仍由其他來源支持的項目只能保留更新後的非敏感摘要。
- screenshot／streaming privacy mode 屬公開測試前的必要設計議題，不得假裝已存在。

## 14. 版本、遷移與確定性

下列版本必須分開：

- memory-event schema version；
- adapter version；
- derivation version；
- evolution-path mapping version；
- creature-genome version；
- morphology-recipe schema version；
- morphology-recipe compiler version；
- morphology-module catalog version；
- EvolutionBridge-rule version；
- renderer version。

升級 derivation 或 genome 時，應能從核准事件重新建立結果。未知的 future version 必須 fail closed，不可以靜默用舊 renderer 猜測。

相同核准事件集合、identity seed 與所有 derivation／path-mapping／genome／recipe-compiler／module-catalog／bridge-rule 版本必須產生相同 path profile、genome、`MorphologyRecipe` 與 bridge，且不受匯入順序、來源列舉順序或重啟影響。一般 presentation settings 與 `LiveAgentPresence` 不得成為 genome 輸入；未來若有會改變成長的設定，必須建模成明確、版本化且可追溯的 `LocalGrowthOverride`。

profile 分數依 PathMappingV1 support table 轉成整數並套用 `6000` bps eligibility floor，避免浮點差異造成抖動；接近同分依明文規格形成 hybrid 或 `balanced-confluence`，只有受視覺 slot 上限約束時才使用 stable ID tie-break。同一事件集合必須回到同一 stage 與 profile，不使用會造成匯入順序差異的隱藏 hysteresis；若使用者反覆匯入／遺忘使 canonical state 確實改變，presentation layer 可以合併揭露或避免重播動畫，但不能拒絕更新 canonical state。

規則更新不可在背景靜默讓寵物變身，必須 pin 舊版或經明示 migration 產生新的 revision 與可查看說明。若新版 derivation 把既有記憶用於原核准範圍之外的新用途，必須先重新說明用途並取得同意，不能只靠 app update 自動擴張 consent。

## 15. 建議交付切片

本設計屬 Phase 2 方向，不取代目前既定的 installer gate → pet-first synthetic shell → real-source Phase 1 順序：

1. **Visual contract prototype** — 把目前概念形態只當作 family grammar 與 bridge reference，再用 synthetic profile matrix 驗證同一 identity／stage 能編譯出許多明顯不同、受限且仍可沿相鄰 bridge 追蹤的 variants。當前 ImageGen 合併稿不是 route roster、production asset 或完成驗收。
2. **Genome foundation** — Rust 純函式推導、schema／migration、revision、forget／rederive 測試，不先做漂亮動畫。
3. **SVG vertical slice** — 一個 synthetic signal 從 genome axis 造成 stage 內形變與一個 lineage mark，含 explain／forget UI。
4. **Major evolution slice** — 至少兩個 stage、可略過轉場、重啟一致、reduced-motion 與雙語摘要。
5. **Density and journal slice** — marks 聚合、slot 上限、Growth Journal 與來源刪除後完整重算。
6. **Time presentation slice** — 晝夜／季節／紀念狀態，只影響 ephemeral render state。
7. **Habitat expansion** — 在核心寵物模型穩定後才擴張棲地、粒子與多物件互動。

每個 slice 仍需 UI、Rust、persistence、lineage、forgetting、bilingual copy、accessibility 與 desktop smoke 的完整垂直驗證。

## 16. 驗收條件

### Product

- 使用者逐對查看相鄰階段時，能指出至少一條明確的保留或轉換關係；從第一階到最後一階則能透過完整演化鏈理解它們的關聯，不要求單看兩端就立即認出。
- 大幅進化有驚喜，但不依賴看不懂的隨機造型。
- 相同 identity 與 stage 在 synthetic activity profile matrix 下能形成許多可辨識、受限且可重算的 variants；每個 recipe 仍呈現可沿相鄰 bridge 追蹤的生命感、有機節奏與節制的神聖高級語言，不要求固定器官永遠存在。
- 使用者能在兩步操作內找到永久變化的原因。
- 忘記來源後，相關外觀與日誌結果一致，不出現幽靈印記。
- 不匯入更多資料也不會遭到懲罰或情緒施壓。

### Technical

- 同一組核准事件、identity seed、版本與明示 `LocalGrowthOverride` 得到相同 genome revision 與 `MorphologyRecipe`；一般 presentation settings 與 `LiveAgentPresence` 不影響永久結果。
- synthetic profile matrix 的 pure／mixed／balanced profiles 都編譯成 allowlisted modules 與量化參數；忘記來源後可確定性重算，並更新 bridge、journal 與 explanation。
- 每個 recipe module instance 與 parameter bucket 都有穩定 key 與機器可讀 lineage；pet DTO 不含 `RecipeLineageMap`。
- renderer 無網路請求，不需要 API key。
- preview／derivation／render logs 不含 source content。
- stage、marks 與 forget／rederive 有 Rust 與 UI 自動測試。
- 重啟後外觀與解釋一致；交易失敗時 UI 不宣稱變化已完成。
- reduced-motion、鍵盤、螢幕閱讀摘要與英／繁中語義通過驗收。
- 多來源共同支持一項效果時，移除部分支持會正確更新 confidence／explanation；歸零才移除效果。
- PathMappingV1 的 `5999／6000／7499／7500／8999／9000` bps 邊界、user-confirmed bucket、tag conflict fail-closed、partial-source 與 last-source forgetting 都有固定 fixtures。
- 1,000 筆同類 synthetic records 仍只形成受控 composite trait，不突破 visual slots 或重複升階。
- DST、時區切換、手動倒轉時鐘與離線數月後重開只改變 ephemeral state。
- 未來 presence adapter 的 cold start 是 neutral；不得產生 DB row／contribution／journal，TTL 到期與 clock skew 不累積狀態，logs／pet DTO 不含 Agent／專案／路徑／session identity。

## 17. 尚未鎖定但不阻擋架構的事項

- 共通視覺語言已確認；精確融合造型、正式 SVG、module catalog 規模、parameter levels、compatibility matrix 與 profile-matrix coverage 仍待 synthetic prototype signoff；
- EvolutionBridge 可使用的完整形態文法、轉換類型與視覺表達；
- 五階段的正式英文／繁中名稱；
- 正式 approved-activity taxonomy、每個 memory signal 的 profile mapping、量化門檻與 visual module 上限；
- hero mark slot 是否維持 5 個；
- 何時需要 Motion 套件、Rive 或 PixiJS；
- 手動修正、presentation controls 與 screenshot privacy mode 的詳細 UX。

這些事項應以共享視覺基準、多組 synthetic profile matrix 與人工 signoff 決定，不應把參考圖中的路線數或造型硬編進資料模型。

## 18. 訪談收斂紀錄

| 維度 | 收斂結果 |
|---|---|
| Goal | 從資料驅動的一次性印記，擴張為可持續、可大幅進化的生命 |
| Constraints | local-first、無 runtime AI API、lineage、forget/rederive、no time-based permanent growth |
| Visual DNA | 生物眼神與有機生命感，結合節制、神聖但不宗教化的高級折面／材質語言 |
| Path model | 同一 identity 依核准 signals 形成版本化的多軸 `EvolutionPathProfile`，再編譯成許多受限 `MorphologyRecipe` variants |
| Success | 相鄰階段與 recipe 改變都有清楚聯繫、遙遠形態可大幅分化、自動變化仍可理解、刪除後不留幽靈效果 |
| Remaining ambiguity | 集中在正式 evidence taxonomy、signal mapping、recipe catalog、參數門檻與 production SVG；適合用 synthetic profile matrix 驗證 |

## 19. 研究來源

- [OpenAI curated `hatch-pet` Skill](https://github.com/openai/skills/blob/main/skills/.curated/hatch-pet/SKILL.md)
- [Motion for React](https://motion.dev/docs/react)
- [PixiJS Graphics](https://pixijs.com/8.x/guides/components/scene-objects/graphics)
- [PixiJS accessibility](https://pixijs.com/8.x/guides/components/accessibility)
- [PixiJS Agent Skills](https://github.com/pixijs/pixijs-skills)
- [Rive data binding for Web](https://rive.app/docs/runtimes/web/data-binding)
- [Rive state machines](https://rive.app/docs/runtimes/state-machines)
- [Rive Web WASM preloading and self-hosting](https://rive.app/docs/runtimes/web/preloading-wasm)
- [Rive runtime sizes](https://rive.app/docs/runtimes/runtime-sizes)
