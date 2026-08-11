# Memoryling 可成長寵物系統設計稿

> Status: Product design draft — user-confirmed product direction; proposed implementation details; not implemented
> AS_OF: 2026-08-11 (Asia/Taipei)
> Scope: creature growth, evolution, rendering, explainability, and forgetting
> Truth boundary: the shipping app still has only the fixture-derived completion star and CSS creature

## 1. 一句話定義

Memoryling 不是每次啟動時重新生成的一張寵物圖片，而是一個由使用者核准記憶所塑造、能大幅進化、仍保有可辨識身份，並可從來源完整重算的本機生命系統。

## 2. 已確認的產品決策

本輪訪談已確認下列方向：

1. **混合型生命邏輯。**穩定身份與記憶造成的長期變化可以持續存在；表情、活力、晝夜與季節狀態可以變動。
2. **永久成長只由核准記憶與其本機衍生資料驅動。**日曆時間本身不增加等級，也不會讓閒置中的寵物憑空進化。
3. **時間只驅動可逆狀態。**時間可以影響晝夜、季節、紀念日呈現與當下動作，但不能直接改寫永久基因。
4. **變化自動套用。**記憶已經通過來源核准後，使用者不必逐項批准每次外觀改變；每項變化仍必須可以解釋與撤銷來源。
5. **允許大幅進化。**生命階段可以顯著改變體型、比例、附肢、移動方式與棲地互動，但必須保留跨階段可辨識的核心特質。
6. **成長核心不依賴 runtime AI API。**AI 或 Skill 可以協助開發期的造型探索；實際執行時使用本機規則、本機狀態與隨 EXE 打包的資產。未來即使加入對話模型，其輸出也不能直接回寫永久 genome、stage 或 lineage。

## 3. 產品承諾

### 使用者應感受到

- 這是同一個生命逐步成長，不是每次隨機換皮。
- 重要記憶會留下看得見、能被理解的影響。
- 大幅進化帶來驚喜，但不會抹掉原本認得出的個性。
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
        ├─→ GrowthContribution → deterministic CreatureGenome → structural state
        └─→ WorldEffect → marks / habitat / story projections

IdentityCore + structural state + active WorldEffects + EphemeralState
    → render-safe CreatureState
    → local renderer + Growth Journal / Why did this happen?
```

SQLite 中的核准事件與來源鏈是事實來源。`CreatureGenome` 是可以重新產生的衍生快照，不可取代事件與 lineage。

`GrowthContribution` 與既有 `WorldEffect` 是同一個 `DerivedSignal` 的平行投影：前者只處理結構性 genome 軸，後者延續現有 completion-star 並處理印記、棲地、故事等離散效果。兩者共同組合成 `CreatureState`，不可形成 `WorldEffect → Genome → WorldEffect` 的循環依賴。

### 四種狀態必須分離

| State | 來源與責任 | 永久狀態規則 |
|---|---|---|
| `IdentityCore` | 本機初次建立的 identity seed、穩定名稱與視覺錨點 | 不由記憶決定；刪除全部記憶後仍回到同一 identity baseline |
| `DerivedGrowth` | 目前核准事件、identity seed 與版本化規則 | 可持續但非不可逆；忘記來源後必須完整重算 |
| `EphemeralState` | 本機時鐘、季節、當下 UI 狀態 | 不得推動永久成長，也不得成為 genome 的隱藏輸入 |
| `HistoryAndAudit` | 成長揭露與解釋索引 | 忘記來源後須刪除相依內容；不得殘留可反推出來源的文字、hash 或特徵 |

只有核准、遺忘、明示的人工修正或版本 migration 能改變永久狀態。啟動 App、時鐘、locale、動畫完成或未來模型輸出都不能直接或間接改變它。若未來模型提出成長候選，它必須先被當成新的不可信來源，經獨立產品決策、明確使用者核准與 machine-readable lineage 後才可能成為輸入；在此之前一律不可進入永久成長圖。

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

### CreatureGenome

建議先使用少量、可測試的軸，而不是無限制自由生成：

| Genome axis | 影響 | 邊界 |
|---|---|---|
| maturity | 生命階段與整體複雜度 | 由記憶意義與穩定訊號決定，不看日曆天數 |
| morphology | 體型、比例、附肢、姿態 | 可大幅改變，但必須通過身份錨點規則 |
| surface | 色盤、紋理、發光、材質感 | 顏色不可是唯一資訊載體 |
| temperament | 呼吸節奏、探索動作、待機姿態 | 只描述可觀察的互動風格，不下敏感人格診斷 |
| memory marks | 星點、符號、飾物、疤紋式故事印記 | 每一項都要有 effect lineage 與可見數量上限 |
| habitat affinity | 與棲地物件、光線、植物或收藏的關係 | 屬後續 slice，不阻擋核心寵物渲染 |

## 5. 生命階段與大幅進化

以下五段是供 prototype 驗證的**提案**，不是已核准 schema；名稱與段數可在視覺測試後調整。若採用，資料層應使用穩定 enum：

| Internal stage | 工作名稱 | 可見變化 |
|---|---|---|
| `seed` | 記憶種 | 幾乎沒有歷史，輪廓最簡潔，建立核心身份錨點 |
| `awakened` | 初醒 | 形成第一個完整身體與基礎動作性格 |
| `growing` | 成形 | 比例、附肢、紋理與移動方式開始分化 |
| `evolved` | 蛻變 | 允許明顯剪影與能力感變化，棲地開始回應牠 |
| `legacy` | 守憶 | 高度個人化但不雜亂，重要印記形成有秩序的整體設計 |

階段門檻由版本化規則判定，考慮去重後訊號的意義、受上限約束的證據強度、語義多樣性與穩定度。來源數量本身不得提供加成；單純連接更多來源或匯入大量相似紀錄不能快速升階。

### 可辨識身份錨點

每隻 Memoryling 在 `seed` 階段建立一組 `CreatureSignature`。prototype 可從下列五類選出固定三項，並把其 anchor ID 與核心值寫入 identity baseline；後續所有階段都必須保留**同一組固定錨點**，不能每次換成另外三類來規避身份連續性。具體錨點仍需視覺方向核准：

1. 眼睛或臉部節奏；
2. 核心記憶種／胸口核心；
3. 代表性色彩或高光；
4. 一個剪影特徵，例如耳、角、冠、尾或觸鬚；
5. 一個招牌動作或待機姿勢。

體型、四肢比例、附肢數量、移動方式與材質可以顯著改變，只要仍保留 `CreatureSignature` 指定的固定錨點與通過人工視覺辨識驗收。

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

1. store 先完成事件、signal、effect、genome revision 與 lineage 的原子交易；
2. UI 取得新的 revision；
3. 一般變化以短而柔和的轉場呈現；
4. stage 改變以可略過的「蛻變時刻」呈現；略過的只有動畫，canonical genome 與文字摘要已經完成更新；
5. Growth Journal 增加一筆不含原始私密文字的摘要；
6. 使用者可從寵物或日誌開啟「Why did this happen?」。

解釋最少要顯示：

- 發生了什麼變化；
- 哪一類核准訊號造成；
- 支援它的來源紀錄數與來源名稱；
- derivation version；
- 刪除哪個 Memoryling 本機匯入來源會重新計算它。

自動套用不等於不可控制。後續人工修正應成為版本化的 `LocalGrowthOverride`：與來源衍生判斷綁定的修正，在相關來源被忘記後預設一起失效；純美術偏好則屬 `IdentityCore` 或 presentation 設定，不能冒充記憶因果。任何「隱藏動畫」或 presentation mode 都只能改變呈現，不能讓畫面長期停在與 canonical genome 不同的舊形態。

## 8. 遺忘與回算

遺忘不能只把印記從畫面隱藏。正確流程是：

1. 在單一本機 transaction 中移除或失效指定 source／record／event；
2. 清除其相依 signals、effects 與 genome snapshot；
3. 從仍存在的核准事件重新推導全部永久狀態；
4. 產生新的 genome revision；
5. commit 後才更新 UI；
6. renderer 只接受新 revision，不保留幽靈 layer。

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
MemorylingCreature
├─ BodyMorphLayer
├─ SignatureLayer
├─ SurfacePatternLayer
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

renderer 只接收 `CreatureRenderState`，不接收原始記憶內容、來源路徑或自由文字。相同 genome revision 與 renderer version 必須得到相同的結構與外觀參數。

### 後續選項

- **Rive：**若角色骨架與形變品質超過 SVG 動畫的維護能力，可用本機 `.riv`、本機 WASM 與 data binding 接同一份 `CreatureRenderState`。採用前需確認 runtime export 授權、CSP、離線載入、二進位資產 review 與測試策略。
- **PixiJS：**只有棲地成為大量粒子、shader、多角色與複雜 scene graph 時才評估；單一寵物階段不值得先承擔 Canvas／WebGL 與 accessibility 成本。
- **PNG 主體＋SVG overlay：**可作短期視覺原型，但不適合作為大幅 morphology evolution 的最終架構。

## 11. AI、Skill 與外掛的正確角色

Skill 是開發工作流，不是 Memoryling 的 runtime 依賴：

- OpenAI 官方 `hatch-pet` 可協助製作固定 8 × 9 動畫圖集，但其核心仍是開發期 ImageGen，且固定 atlas 無法自行承擔連續 genome evolution。
- Product Design／ImageGen 可探索三種以上視覺家族，選定方向後再轉成可維護的本機 SVG 元件或資產。
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
- creature-genome version；
- renderer version。

升級 derivation 或 genome 時，應能從核准事件重新建立結果。未知的 future version 必須 fail closed，不可以靜默用舊 renderer 猜測。

相同核准事件集合、identity seed 與所有 derivation／genome 版本必須產生相同 genome，且不受匯入順序、來源列舉順序或重啟影響。一般 presentation settings 不得成為 genome 輸入；未來若有會改變成長的設定，必須建模成明確、版本化且可追溯的 `LocalGrowthOverride`。

分支同分時使用明文規格化的 stable tie-break，分數先經穩定量化與 confidence floor，避免微小浮點差異造成抖動。同一事件集合必須回到同一 stage，不使用會造成匯入順序差異的隱藏 hysteresis；若使用者反覆匯入／遺忘使 canonical stage 確實改變，presentation layer 可以合併揭露或避免重播動畫，但不能拒絕更新 canonical state。

規則更新不可在背景靜默讓寵物變身，必須 pin 舊版或經明示 migration 產生新的 revision 與可查看說明。若新版 derivation 把既有記憶用於原核准範圍之外的新用途，必須先重新說明用途並取得同意，不能只靠 app update 自動擴張 consent。

## 15. 建議交付切片

本設計屬 Phase 2 方向，不取代目前尚未完成的 installer gate 與 real-source Phase 1：

1. **Visual contract prototype** — 僅用 synthetic genome，製作三個跨五階段的角色家族，驗證「大幅變化仍認得出來」。
2. **Genome foundation** — Rust 純函式推導、schema／migration、revision、forget／rederive 測試，不先做漂亮動畫。
3. **SVG vertical slice** — 一個 synthetic signal 從 genome axis 造成 stage 內形變與一個 lineage mark，含 explain／forget UI。
4. **Major evolution slice** — 至少兩個 stage、可略過轉場、重啟一致、reduced-motion 與雙語摘要。
5. **Density and journal slice** — marks 聚合、slot 上限、Growth Journal 與來源刪除後完整重算。
6. **Time presentation slice** — 晝夜／季節／紀念狀態，只影響 ephemeral render state。
7. **Habitat expansion** — 在核心寵物模型穩定後才擴張棲地、粒子與多物件互動。

每個 slice 仍需 UI、Rust、persistence、lineage、forgetting、bilingual copy、accessibility 與 desktop smoke 的完整垂直驗證。

## 16. 驗收條件

### Product

- 使用者看五階段並排圖時，能辨識它們是同一隻 Memoryling。
- 大幅進化有驚喜，但不依賴看不懂的隨機造型。
- 使用者能在兩步操作內找到永久變化的原因。
- 忘記來源後，相關外觀與日誌結果一致，不出現幽靈印記。
- 不匯入更多資料也不會遭到懲罰或情緒施壓。

### Technical

- 同一組核准事件、identity seed、版本與明示 `LocalGrowthOverride` 得到相同 genome revision；一般 presentation settings 不影響結果。
- renderer 無網路請求，不需要 API key。
- preview／derivation／render logs 不含 source content。
- stage、marks 與 forget／rederive 有 Rust 與 UI 自動測試。
- 重啟後外觀與解釋一致；交易失敗時 UI 不宣稱變化已完成。
- reduced-motion、鍵盤、螢幕閱讀摘要與英／繁中語義通過驗收。
- 多來源共同支持一項效果時，移除部分支持會正確更新 confidence／explanation；歸零才移除效果。
- 1,000 筆同類 synthetic records 仍只形成受控 composite trait，不突破 visual slots 或重複升階。
- DST、時區切換、手動倒轉時鐘與離線數月後重開只改變 ephemeral state。

## 17. 尚未鎖定但不阻擋架構的事項

- 核心身份錨點的具體造型與主色；
- 五階段的正式英文／繁中名稱；
- 每個未來 memory signal 對 genome axis 的正式 mapping；
- hero mark slot 是否維持 5 個；
- 何時需要 Motion 套件、Rive 或 PixiJS；
- 手動修正、presentation controls 與 screenshot privacy mode 的詳細 UX。

這些事項應以 synthetic fixtures、三方向視覺比較與人工 signoff 決定，不應在資料模型中硬編未經驗證的美術假設。

## 18. 訪談收斂紀錄

| 維度 | 收斂結果 |
|---|---|
| Goal | 從資料驅動的一次性印記，擴張為可持續、可大幅進化的生命 |
| Constraints | local-first、無 runtime AI API、lineage、forget/rederive、no time-based permanent growth |
| Success | 大幅變形仍可辨識、自動變化仍可理解、刪除後不留幽靈效果 |
| Remaining ambiguity | 約 15%；集中在美術方向與各 signal mapping，適合用 prototype 而非繼續口頭抽象討論 |

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
