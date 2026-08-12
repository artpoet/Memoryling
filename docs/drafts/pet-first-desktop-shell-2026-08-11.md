# Memoryling Pet-First Desktop Shell 設計稿

> Status: User-confirmed direction; 0.2.0 core vertical slice implemented and verified; ADR-0003 remains Proposed pending the full Windows acceptance matrix
> AS_OF: 2026-08-12 (Asia/Taipei)
> Scope: Windows floating pet, context menu, detail window, tray recovery, lifecycle, and accessibility
> Truth boundary: pet-first shell and fixture continuity are live; real-source and creature-growth implementation have not started, and only the compact baseline envelope exists

## 1. 結論

Memoryling 應採用「兩個表面、一個生命」：平常只顯示一隻安靜的透明浮動寵物；需要記憶、成長日誌、來源管理或設定時，使用者以右鍵選單開啟標準詳細視窗。

這個方向比把完整 dashboard 長期留在桌面上更符合產品願景，也能讓寵物成為主要體驗，而不是 dashboard 裡的一張插圖。

## 2. 已確認方向與提案細節

### 使用者已確認

- 常態呈現是一隻單純浮動的寵物。
- 右鍵寵物才顯示「開啟進階內容」的入口。
- 選擇入口後才顯示完整詳細內容。

### 本設計稿提案

- 右鍵是**主要入口，但不是唯一救援入口**；系統匣、Start Menu 與安裝時實際建立的 app shortcut 必須能找回視窗。本機 repository root 的 `.lnk` 只屬開發／測試便利，不是 packaged product contract。
- v1 使用 Windows／Tauri 原生 context menu，不先做自繪泡泡，以降低鍵盤、DPI、螢幕邊界與 screen reader 風險；可靠性仍需 packaged Narrator／NVDA UAT 證明。
- 浮動寵物與詳細內容使用兩個 Tauri windows，但共享同一個 Rust backend、SQLite store 與 creature state。
- 開啟詳細內容時先收起浮動寵物；關閉詳細視窗後寵物回到原位置，避免同時出現兩隻 Memoryling。
- 寵物預設 always-on-top，以符合「浮動」感；使用者可從右鍵選單、系統匣或設定取消置頂／隱藏。
- 不預設登入時自動啟動；autostart 必須留到未來明確 opt-in。

### 0.2.0 live evidence

- 23 個 frontend tests 與 29 個 Rust tests 通過；Rust 包含首次並行開庫、lifecycle 補償、position／anchor、privacy DTO、exact capabilities 與 settings recovery。
- `pet` 逐一 invoke list／preview／cancel／full-state／approve／forget 六個敏感 commands 時，production ACL 與 caller-label defense 兩層各自 fail closed，handler side-effect counter 保持 0；`main` list 是正向控制。
- 透明 pet、一次性 onboarding、原生 pointer／focused-keyboard menu、single-instance、close／minimize／restore、raw movement／second-monitor observation、核心 pet／main smoke 與 explicit native Quit 通過。Tray actions 與 position recovery 有 automated evidence，不冒充完整 live acceptance。
- raw bundled fixture 的 preview／approve、restart persistence、source → event → signal → completion-star lineage 與 forget 通過；未使用真實記憶。
- 正常由 Explorer 啟動的 current-user NSIS install、實際 Start shortcut 的 cold launch 與 resident single-instance relaunch、以及保留資料的 uninstall 通過。
- `Memoryling_0.2.0_x64-setup.exe` 為 2,875,965 bytes，SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`，版本 0.2.0，`NotSigned`。
- 早期 agent-direct installer launch 觸發 Windows virtualization，屬無效 harness artifact，不是產品失敗；packaged 證據只採正常 Explorer／installed-shortcut 路徑。
- 尚未通過：live 125–200%／mixed DPI、monitor hot-unplug、taskbar relocation、desktop adjacent-hitbox、`Win+B`、Narrator／NVDA、sign-out／shutdown、compact／wide／tall／long envelope。WebView2-missing bootstrapper 仍 deferred。

## 3. 體驗原則

1. **Pet first.** 啟動後第一眼是生命，不是面板。
2. **Quiet by default.** 寵物不搶 focus、不彈出完整內容、不自行遮住工作。
3. **One life, two surfaces.** 浮動寵物與詳細視窗是同一狀態，不是兩份角色或兩個資料庫。
4. **Right-click primary, recovery always available.** 滑鼠使用者以右鍵進入；`Win+B` 系統匣、Start Menu 與再次啟動都能找回程式。Pet 已取得 focus 時，Enter／Space／Menu key／`Shift+F10` 提供等效操作，但不把它誤稱為未聚焦時的獨立 recovery。
5. **Hide is not quit.** 隱藏寵物、關閉詳細內容與結束程式必須有不同語意。
6. **Honest boundary.** 在真實 connector 完成前，pet-only surface 仍要看得到「real memory access is off」。

## 4. 三個產品表面

### A. Floating pet window — `pet`

常態唯一可見的 app surface：

- 透明、無框、固定小尺寸；
- Windows 上關閉 undecorated-window shadow，避免透明浮窗周圍出現白邊或系統圓角；
- 緊貼角色輪廓，避免大面積透明矩形吞掉桌面點擊；
- 預設 always-on-top、skip taskbar、不可 resize；
- 啟動時不主動搶 focus，但保持可聚焦，讓鍵盤操作成立；
- 左鍵單擊只觸發短反應或一句話；
- 左鍵拖曳移動位置；
- 右鍵開啟游標位置的原生選單；pet 已取得 focus 時，Enter／Space／Menu key／`Shift+F10` 開啟錨定於 pet 的同一選單；
- 不顯示 dashboard、來源文字或持續展開的 speech panel。

在 real connector 完成前，這個 pet surface 內含一個 compact `Memory access off` honesty badge；它不構成第二個產品面板。

0.2.0 使用 360 × 430 logical px 的 first-run onboarding envelope，dismiss 後由 Rust 原子地保存並縮為 320 × 320 compact envelope，同時保留 bottom-right／center anchor、clamp 並保存新位置；任一步失敗會回復舊 geometry 且不提交 onboarding flag。透明 window 仍會以整個矩形攔截滑鼠；Tauri 的 click-through 是整窗切換，不會依透明像素自動穿透，因此縮小 hit box 是 P0 正確性要求，不只是視覺 polish。相鄰桌面 hitbox 實測與 route-agnostic wide／tall／long envelope 尚未完成，不能把 compact baseline 說成 Phase 2 growth coverage。

### B. Detail window — `main`

沿用目前完整 App 的標準 Windows surface：

- 有標準標題列、工作列項目、Alt+Tab、resize、minimize 與 screen reader 結構；
- 初始 `visible: false`，由 context menu、tray、Start Menu 或 installed app shortcut 的第二次啟動顯示／聚焦；
- app 啟動時就預先建立並隱藏，不在同步 menu／command handler 中臨時建立 WebView；Tauri 的 Windows 文件警告後者可能造成 deadlock；
- 永遠只存在一個實例，不重複開多個 dashboard；
- `X` 或 `Alt+F4` 關閉詳細內容時採 hide，不退出 process；
- 關閉後恢復 `pet` 到先前位置；
- minimize `main` 時顯示 `pet`；從工作列或任何 Open 路徑 restore `main` 時再次收起 `pet`；
- 真正退出只能由明確的「結束 Memoryling」或作業系統 session shutdown 發生。

詳細內容建議依資訊架構逐步分成：

1. Overview／現在的 Memoryling；
2. Memories & Sources／記憶與來源；
3. Growth Journal／成長與 EvolutionBridge；
4. Habitat & Events／棲地與事件；
5. Privacy & Settings／隱私與設定。

本設計稿不先決定 sidebar、tabs、色彩或 spacing；進入 UI 實作前必須依 Product Design 流程提出三個視覺方向並選定一個目標。

### C. System tray — recovery surface

系統匣不是主要體驗，而是必備的救援與 lifecycle surface：

- Open Memoryling；
- Show／Hide pet；
- Always on top；
- Quit Memoryling。

寵物被隱藏、掉到移除的螢幕、詳細視窗關閉或使用者忘記右鍵操作時，tray 必須仍能找回程式。

## 5. Pet context menu

v1 採用原生 context menu，首項是清楚的主要動作：

```text
Open Memoryling / 開啟 Memoryling
────────────────────────────
Real memory access: Off / 真實記憶存取：關閉   (disabled status)
Show pet / 顯示寵物
Always on top / 永遠顯示在最上層              (checked)
Hide pet / 隱藏寵物
Quit Memoryling / 結束 Memoryling
```

「Open Memoryling」應是第一個 enabled item。右鍵路徑顯示於游標附近；鍵盤路徑以 `popup_menu_at` 錨定在 pet 可見範圍。第一項鍵盤 focus 不先靠文件宣稱，必須由 packaged Windows UAT 驗證。狀態列不可顯示任何來源文字、記憶摘要或可反推內容的 trait。

品牌化自繪按鈕／泡泡可在原生選單通過 Windows UAT 後評估；在那之前不要為了視覺造型承擔 popover window、clipping、DPI 與 keyboard behavior 的額外風險。

## 6. 基本互動

| Input | 行為 | 不應發生 |
|---|---|---|
| Left click | 短反應、眨眼、姿態或一行本機文案 | 不開 dashboard、不發網路請求 |
| Left drag | frontend 在 gesture threshold 後呼叫 `start_pet_dragging`；Rust `PetCaller` 只拖曳 caller 自己的 pet window | 不開放可指定任意 window label 的 generic core drag capability；相鄰桌面 hitbox 仍待 live UAT |
| Right click | 開啟原生 context menu | 不直接開完整視窗 |
| Enter／Space／Menu key／Shift+F10（pet 已 focus） | 以固定 pet anchor 開啟同一原生選單 | 不把游標可能位於別處的 `popup_menu` 當鍵盤定位 |
| `Win+B` → tray | 鍵盤找回 Open／Show／Hide／Quit | 不要求先聚焦 skip-taskbar pet |
| Open Memoryling | 收起 pet，show／unminimize／focus `main` | 不建立第二個 dashboard |
| Close／Alt+F4 on `main` | hide `main`，恢復 pet | 不退出整個程式 |
| Minimize／restore `main` | minimize 顯示 pet；restore 收起 pet | 不讓日常 presence 消失或同時顯示兩隻 |
| Hide pet | 進入 tray-only | 不等於 quit |
| Relaunch Start／installed shortcut | single-instance callback 顯示／聚焦既有 `main` | 不產生第二隻 pet，也不記錄 argv／cwd／可能含路徑的資料 |

不在 v1 定義雙擊。雙擊開啟 detail 只有在 usability test 證明需要後才增加，避免與單擊反應和拖曳競爭。

## 7. First-run discoverability

只依賴右鍵會有發現性問題，因此第一次啟動需有一次簡短 onboarding：

- 「拖曳我來移動位置」；
- 「右鍵點我開啟 Memoryling」；
- 「找不到我時，可以從系統匣叫我回來」；
- 「真實記憶存取目前關閉」。

0.2.0 已實作：完成第一次成功開啟 detail 後不再自動顯示教學；選擇 Skip 會將本機 `onboardingDismissed` 保存為 true。geometry 調整與 JSON 設定採 transaction-like 順序，resize／reposition／save 任一步失敗都回復舊 bounds 且前端繼續顯示 onboarding。教學尊重 reduced motion，也不會每次更新後重新出現；Privacy & Settings 保留 Controls／How to use 入口供重設。

在 real connector 尚未完成前，pet 下方保留一個極小、單行、低干擾但可讀的狀態 tag：`Memory access off`／`記憶存取關閉`。這是目前誠實性邊界，不可只藏在右鍵選單或 detail 裡。

## 8. Window state machine

```text
FIRST_RUN
  → PET_VISIBLE

PET_VISIBLE
  ├─ right-click / keyboard menu → PET_MENU
  ├─ drag → PET_VISIBLE + save position
  └─ tray Hide → TRAY_ONLY

PET_MENU
  ├─ Open → DASHBOARD_OPEN (pet hidden)
  ├─ Hide → TRAY_ONLY
  └─ dismiss → PET_VISIBLE

DASHBOARD_OPEN
  ├─ close / Alt+F4 → PET_VISIBLE (cancel pending preview in Rust)
  ├─ minimize → DETAIL_MINIMIZED + PET_VISIBLE
  └─ Quit → EXITED

DETAIL_MINIMIZED + PET_VISIBLE
  ├─ taskbar restore / Open → DASHBOARD_OPEN (pet hidden)
  ├─ Hide pet → TRAY_ONLY (pet 與 minimized main 都 hidden；main 仍預建供 Open 恢復)
  └─ Quit → EXITED

TRAY_ONLY
  ├─ Show pet → PET_VISIBLE
  ├─ Open → DASHBOARD_OPEN
  └─ Quit → EXITED

launch / pet show / settled move or scale change / single-instance recovery
  → validate + clamp pet position → previous running state
```

P0 不承諾尚未設計的 in-process WebView crash reconstruction；若整個 process crash，下次啟動必須把 pet 移回可見 work area。WebView2 `ProcessFailed` recovery 與 packaged crash injection 屬 P1。

## 9. Position、monitor 與 DPI

- 0.2.0 初次位置放在 primary monitor work area 右下角並保留安全邊距；獨立的 content-free shell JSON 儲存 logical／DIP position、monitor identity、work-area dimensions、normalized position 與 scale，不只存 physical pixels。
- Rust 在 launch、pet show／recovery、settled move／scale change 與 single-instance callback 重新選 monitor 並 clamp；pet 可見時另以受控 polling 偵測 monitor／work-area topology 變化，因為 Tauri 沒有完整的 taskbar-change WindowEvent。
- onboarding resize 會先保留 bottom-right／center anchor，再依目前 work area reposition、clamp 與 persist；resize／reposition／save 任一步失敗都回復舊 geometry 與 setting。
- 純 geometry tests 已涵蓋 taskbar work-area offset／縮小、oversized window、monitor removal fallback，以及 100%、125%、150%、200% 下 320 logical px 對應的不同 physical size。
- live 125–200%／mixed DPI、monitor hot-unplug、taskbar relocation 與相鄰桌面 hitbox 仍待 packaged Windows UAT，不能用純 geometry tests 代替。
- v1 不做 click-through；它容易讓寵物無法重新被點擊或拖曳。

0.2.0 沒有採用 generic window-state restore；shell settings 由 Rust 以 temp sibling、atomic replace 與 recovery 處理，且與 memory SQLite／forget lifecycle 分離。這避免還原 `main` visibility，也保留 invalid／truncated settings fail-safe recovery。

## 10. Accessibility

- pet 視覺可 `aria-hidden`，外層使用原生 button／可聚焦控制提供名稱與簡短狀態。
- App 啟動時不搶走目前工作的 focus。
- pet window 已取得 focus 時，Enter／Space／`Shift+F10`／Menu key 開啟錨定於 pet 的原生選單；frontend tests 已覆蓋 focused-keyboard dispatch，原生系統處理 Esc、方向鍵與 Enter。
- 正常 Explorer 安裝後的實際 Start shortcut 已通過 cold／resident recovery smoke；direct tray actions 與 `Win+B` 鍵盤遍歷仍待 live UAT，v1 不以未驗證的 global shortcut 假裝補足可及性。
- screen reader 只播報狀態改變一次，不逐幀朗讀動畫。
- reduced-motion 模式直接顯示穩定姿態，不播放強烈進出場或 morph；此分支已有 frontend test。
- 高對比、Narrator／NVDA、live 200% 與完整 keyboard-only 路徑仍是 P0 驗收，不延後到視覺 polish。
- detail window 保留語義化 heading、landmark、focus order 與文字版「Why did this happen?」。

## 11. Implemented Tauri architecture（ADR remains Proposed）

### Window configuration

0.2.0 保留 `main` label 作 detail，並新增 `pet`：

| Setting | `pet` | `main` |
|---|---|---|
| route | `index.html?surface=pet` | `index.html?surface=detail` |
| initial visibility | pre-created hidden；Rust setup 顯示 | pre-created hidden |
| transparent | true | false |
| decorations | false | true |
| resizable | false | true |
| always on top | true by default | false |
| skip taskbar | true | false |
| initial focus | false | only when explicitly opened |
| size | 360 × 430 onboarding → 320 × 320 compact；Rust-owned resize／anchor／re-clamp；wide／tall／long pending | current 1180 × 780 constraints |

0.2.0 已以 Tauri configuration 預建兩個 unique window labels，並為 `pet` 設定 transparent、decorations false、shadow false、always-on-top、skip-taskbar、不可 resize 與不可 close。Windows 透明視窗仍需在完整 DPI／GPU matrix 留意白色 flash；`visibleOnAllWorkspaces` 不支援 Windows，因此產品不承諾跨所有虛擬桌面永遠顯示。

### Surface routing

`main.tsx` 已依 `surface` query／window label 渲染：

- `PetSurface`：只載入 render-safe `CreatureRenderState`、互動與 access-off tag；
- `DetailSurface`：沿用完整 App 與 FirstMemoryFlow；
- browser：維持 detail preview，明說 floating pet／native menu 不可用，不偽造多視窗行為。

`PetSurface` 已採獨立透明 surface reset，不繼承 detail 的最小寬度與深色背景；native pet smoke 證實不再是 dashboard／黑色矩形。

### Rust-owned lifecycle

0.2.0 由 Rust 管理視窗生命週期，WebView 不取得廣泛 window mutation 權限：

- Open 取得預建的 `main`，依序 unminimize、show、focus，再 hide `pet`；Return 先恢復／clamp `pet`，再 hide `main`。兩條路徑都有 compensating rollback，逐步 failure tests 驗證不留下雙窗或無 recovery surface。
- `main` minimize 顯示 pet，restore／focus 收起 pet；Tray Hide 會把 pet 與仍在 taskbar 的 minimized main 一起藏進一致的 tray-only mode。
- 只攔 `main` 的 `CloseRequested`，先在 Rust 清除 pending preview，再 hide main、恢復 pet；Minimize 不清 preview。
- explicit Quit 不被 resident close interception 吞掉；tray、menu 與 single-instance callback 重用同一組 Rust lifecycle functions。正常 Quit 已 smoke 通過；Windows sign-out／shutdown 仍待 live UAT。

0.2.0 沒有 `core:default`、remote origin 或 wildcard；app commands 同時收窄兩層：

1. `tauri_build::AppManifest::commands` 產生 exact permissions；只有 `main` 能呼叫 `list_memory_sources`、`preview_memory_source`、`cancel_memory_preview`、`get_memory_state`、`approve_memory_import`、`forget_memory_source`；
2. 每個敏感 command 的 `MainCaller` 同時核對 message webview label 與 owning window label；`PetCaller`／`RenderCaller` 也只接受其明確 surface；
3. `pet` 只取得窄的 render／shell／menu／onboarding commands 與 caller-bound `start_pet_dragging`；它不能傳入其他 window label，也沒有 generic core window-drag permission；
4. production-context ACL invoke harness 與 empty-authority caller harness 都逐一拒絕六個敏感 commands，handler side-effect counter 為 0；`main` list 作正向控制。

兩個 local webviews 只列出實際需要的 event permissions；menu、tray、跨窗 show／hide／focus、resize 與位置存取都由 Rust 擁有。`pet` 沒有任意 create／destroy／resize、show／focus、menu、tray 或指定 label 的 window capability。

### Native menu、tray 與 single instance

- Rust setup 建立並持有單一原生 context menu。`PetSurface` 的 pointer／focused-keyboard event 只呼叫狹窄 command；Rust 驗證 caller 必須是 `pet`，並在正確 anchor popup。menu event 直接呼叫 Rust lifecycle，pet frontend 不取得 menu 或跨窗權限。
- tray 使用 Tauri desktop tray API 建立 Open／Show／Hide／Always on top／Quit；Rust 建立的 menu／tray 不需要 frontend capability。
- `tauri-plugin-single-instance` 是第一個 plugin；第二次啟動只喚醒既有 process 並聚焦 detail，不保存或轉發 argv、cwd 或可能含私密路徑的值。實際 Start shortcut 的 cold／resident 兩路已通過。
- 位置與 onboarding 使用 Rust-owned content-free JSON，不使用 generic window-state plugin，也不還原 `main` visibility。

不新增網路、telemetry、global shortcut、autostart 或任意 filesystem capability。

## 12. Cross-window state

Rust／SQLite 仍是 memory canonical state；shell settings 則是獨立的 content-free JSON。完整 `MemoryState` lineage 含核准後文字，不能交給 pet surface；0.2.0 已加入內容最小化的 typed DTO：

1. detail 以 `MainCaller` 讀完整 state；pet 只取得 `CreatureRenderState`，其中不含 normalized text、display、locator、path、content hash 或 explanation content；
2. approve、forget 或未來 genome revision commit 成功後，Rust 對 `pet` 與 `main` 發送只含 `{revision}` 的 `memoryling://creature-state-changed`；event failure 不回滾已提交的 memory transaction；
3. 兩個 surfaces 收到 content-free event 後分別重新讀取其 scope 允許的 typed state；
4. event 不攜帶 normalized text、source path 或 memory payload；
5. pet renderer 只取得最終 render-safe mark IDs、visual-module IDs、受限 geometry／motion parameters 與 opaque revision，不取得 route profile ID／權重、來源細節、原始 activity labels 或人格摘要。

detail 以 X／Alt+F4 關閉時，Rust close-request path 會先在 backend 明確取消 pending preview，再 hide；`hide(main)` 不會 unmount WebView，因此不依賴 React cleanup。Minimize 不取消 preview；failure compensation 維持一致 mode 並保留一個 recovery surface。

locale、always-on-top、onboarding completion 與 pet position 屬本機 UI settings，不混入 memory-derived genome 或 lineage。Always-on-top 的 Rust canonical setting 同步驅動 pet context menu 與 tray 的 checked state；onboarding completion 只在 geometry transition 成功後原子提交。

## 13. Privacy and presentation

- pet、menu、tray 與 OS window title 不顯示來源內容、人物、專案名稱或敏感 trait。
- pet-only state 只表達 render-safe 外觀與中性狀態。
- screen sharing privacy mode 屬後續 P1，但公開測試前必須完成。
- detail hidden 不代表資料從記憶體消失；pending preview lifecycle 必須有測試與清楚文案。
- 關閉 detail、隱藏 pet、quit 與 forget source 是四個不同操作；不可混淆。

## 14. Delivery plan

此功能不是 CSS 改版，而是新的 Windows resident-app lifecycle。0.2.0 已以 bundled raw fixture 完成 shell core；下列狀態明確區分已交付與仍待驗收的部分。

### Slice A — Two-window shell（0.2.0 completed）

- `pet`／`main` window configuration；
- surface routing；
- 預建隱藏的 `main` 與 Rust show／hide／focus lifecycle；
- main close／minimize／restore → pet return／hide；
- app-command permissions、caller-label defense 與從 pet invoke 敏感 command 的 fail-closed tests；
- browser truth boundary；
- unit tests + native desktop smoke。

### Slice B — Native access and recovery（0.2.0 core completed）

- native right-click menu；
- caller-label validation、最小 pet／main core capabilities 與 app-command permissions；
- focused-keyboard menu equivalents 已有 automated coverage；`Win+B` live traversal pending；
- tray Show／Hide／Open／Quit；
- single-instance Start／installed shortcut relaunch；
- access-off status與首次 onboarding。

### Slice C — Position and state continuity（compact baseline completed；live matrix／growth pending）

- pointer gesture threshold 與 caller-bound native drag 已完成；packaged adjacent-desktop hitbox pending；
- content-free JSON position persistence、launch／show／settled move／scale／single-instance clamp 與 topology polling 已完成 automated coverage；live mixed-DPI／hot-unplug／taskbar relocation pending；
- Rust content-free revision event、pet／main refetch、approve／forget mark sync 與 restart persistence 已完成；
- compact onboarding → baseline resize／anchor／rollback 已完成；route-agnostic wide／tall／long envelopes 與實際 route matrix 未開始；
- WebView process-failure detection／recovery remains a separately tested P1。

### Slice D — Detail information architecture（not started）

- 把現有長頁整理為 Overview／Memories／Growth／Habitat／Privacy；
- 不改動既有 fixture lineage 與 forgetting truth；
- 英文／繁中 parity、keyboard、Narrator／NVDA 與 200% zoom。

### Slice E — Windows packaging UAT（core passed；full matrix pending）

- current-user Explorer install、實際 Start shortcut cold／resident relaunch、pet／main core、menu、close-return、explicit native Quit 與 retained-data uninstall 已通過；
- Windows sign-out／shutdown、live 125–200%／mixed DPI、monitor removal、taskbar relocation 與 `Win+B` pending；
- WebView2-missing bootstrapper deferred；
- 0.2.0 NSIS 與 checksum 已重建：2,875,965 bytes，SHA-256 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`，`NotSigned`。

每個 slice 必須完成產品路徑、Rust／React tests、native desktop smoke、文件與真實狀態更新，不能只留下 window connector 骨架。

## 15. Acceptance criteria

- [x] Fresh launch 只顯示透明 pet surface 與 access-off badge，不自動開 dashboard；first-run onboarding 可 atomic dismiss／resize，失敗會 rollback。
- [x] Pointer 右鍵與 focused-keyboard command 開同一原生 menu；Open Memoryling 是第一個 enabled item。
- [x] 正常 Explorer 安裝後，Start Menu 的實際 shortcut 可 cold launch；resident relaunch 聚焦既有 app，不產生第二個 process、pet 或 SQLite writer。
- [x] Open 只顯示／聚焦一個 detail window；detail minimize／restore、X／Alt+F4 pending-preview reset、tray-only Hide 與 recovery 維持單一一致 surface。
- [x] Explicit native Quit 結束 process；direct tray action acceptance 仍列於下方未完成 gate。
- [x] 從 `pet` 逐一 invoke memory list／preview／cancel／state／approve／forget，在 production ACL 與 caller-label defense 兩層都 fail closed，handler 未執行。
- [x] Raw fixture preview／approve、restart、source → event → signal → completion-star lineage 與 forget 一致；失敗 transaction 不改 committed state。
- [x] Pet DTO／events 不含核准記憶文字、display、locator、path、content hash 或 lineage explanation；event payload only `{revision}`。
- [x] Browser 不假裝支援 native floating pet；runtime 無網路、telemetry、圖片／LLM API，且 real memory access off 保持可見。
- [x] Lifecycle 每一步 failure 的 automated tests 驗證 compensating rollback，保留一個可恢復 surface。
- [ ] Direct tray Open／Show／Hide／always-on-top／Quit、`Win+B` traversal、Narrator／NVDA、高對比與完整 keyboard-only live UAT。
- [ ] Windows sign-out／shutdown 不被 resident close logic 阻擋的 live UAT。
- [ ] Pet visible union 外框不超過 12 DIP，並以 packaged adjacent-desktop clicking 驗證沒有更大透明 hitbox。
- [ ] Compact／wide／tall／long envelopes、live 125–200%／mixed DPI、monitor hot-unplug 與 taskbar relocation。
- [ ] WebView2-missing bootstrapper path；目前 deferred，不阻擋 0.2.0 core evidence，但阻擋 ADR Accepted。

## 16. P1 after the shell is proven

- 全螢幕遊戲／簡報時自動退讓；
- screen sharing／capture privacy mode；
- 依 monitor layout 記住多組位置；
- pet 尺寸 presets 與鍵盤移到角落；
- opt-in autostart；
- 可選 global shortcut；
- pet 進入 detail 的品牌化轉場；
- 原生 menu 通過 UAT 後再評估品牌化 popover；
- WebView2 process-failure detection、pet／tray preservation and packaged crash recovery。

## 17. Non-goals for the first slice

- 不實作 pet 自主在桌面四處走動；
- 不做全畫面 click-through；
- 不加入遠端 AI、即時生圖或雲端同步；
- 不掃描 Codex tool-home；
- 不改寫已完成的 fixture lineage／forgetting 基礎；
- 不把 browser mock 當 native window 驗證；
- 不複製 Codex 的角色造型、資產或品牌識別；只借鑑「桌面上單一浮動生命、進階內容按需開啟」的互動層次；
- 不把尚未實作的 wide／tall／long growth visuals 說成 production pet 已具備。

## 18. Official technical references

- [Tauri configuration and multi-window labels](https://v2.tauri.app/reference/config/)
- [Tauri native menu and `Menu.popup()`](https://v2.tauri.app/reference/javascript/api/namespacemenu/)
- [Tauri Rust `Window::popup_menu`](https://docs.rs/tauri/latest/tauri/window/struct.Window.html#method.popup_menu)
- [Tauri Rust `WebviewWindow::popup_menu_at`](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindow.html#method.popup_menu_at)
- [Tauri Rust `WebviewWindowBuilder` and Windows handler warning](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewWindowBuilder.html)
- [Tauri system tray](https://v2.tauri.app/learn/system-tray/)
- [Tauri window customization and dragging](https://v2.tauri.app/learn/window-customization/)
- [Tauri core permissions](https://v2.tauri.app/reference/acl/core-permissions/)
- [Tauri capabilities](https://v2.tauri.app/security/capabilities/)
- [Tauri single-instance plugin](https://v2.tauri.app/plugin/single-instance/)
- [Tauri window-state plugin](https://v2.tauri.app/plugin/window-state/)
- [Tauri window-state builder flags and filters](https://docs.rs/tauri-plugin-window-state/latest/tauri_plugin_window_state/struct.Builder.html)
- [Tauri runtime `WindowEvent`](https://docs.rs/tauri/latest/tauri/enum.WindowEvent.html)
- [Tauri cross-webview events](https://v2.tauri.app/develop/calling-rust/)
