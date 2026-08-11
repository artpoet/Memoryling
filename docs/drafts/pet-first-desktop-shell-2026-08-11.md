# Memoryling Pet-First Desktop Shell 設計稿

> Status: User-confirmed interaction direction; proposed UX and technical details; not implemented
> AS_OF: 2026-08-11 (Asia/Taipei)
> Scope: Windows floating pet, context menu, detail window, tray recovery, lifecycle, and accessibility
> Truth boundary: the current app still opens one 1180 × 780 standard window; no floating pet window, tray, or two-window lifecycle exists yet

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

尺寸需在選定寵物視覺後量測，初始 prototype 可從約 220–260 logical px 的緊密工作區開始，不把此數字視為最終規格。透明 window 仍會以整個矩形攔截滑鼠；Tauri 的 click-through 是整窗切換，不會依透明像素自動穿透，因此縮小 hit box 是 P0 正確性要求，不只是視覺 polish。Phase 0.5 shell 只以 route-agnostic 的 compact／wide／tall／long synthetic envelope fixtures 驗證 bounds，不等待或假裝已鎖定 Phase 2 taxonomy。若採固定 envelope，不能為巨大弧線、光環或尾端粒子留下大面積透明攔截區；若 render state 需要動態 resize，尺寸變更必須由 Rust 管理，完成後重新 clamp，而不是讓 pet WebView 任意改窗。

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
| Left drag | 目標是在 4–8 DIP gesture threshold 後拖曳 pet window；先以 packaged Windows spike 驗證延後呼叫 `startDragging` 是否穩定 | 未通過 spike 前不承諾 click／drag 已正確分流 |
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

完成第一次成功開啟 detail 後不再自動顯示教學。選擇 Skip 會將本機 `onboardingDismissed` 永久設為 true；教學必須尊重 reduced motion，也不能每次更新後重新出現。Privacy & Settings 保留 Controls／How to use 入口，讓使用者能再次查看。

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
  ├─ Hide pet → DETAIL_MINIMIZED + TRAY_ONLY
  └─ Quit → EXITED

TRAY_ONLY
  ├─ Show pet → PET_VISIBLE
  ├─ Open → DASHBOARD_OPEN
  └─ Quit → EXITED

launch / pet show / drag end / scale change / single-instance recovery
  → validate + clamp pet position → previous running state
```

P0 不承諾尚未設計的 in-process WebView crash reconstruction；若整個 process crash，下次啟動必須把 pet 移回可見 work area。WebView2 `ProcessFailed` recovery 與 packaged crash injection 屬 P1。

## 9. Position、monitor 與 DPI

- 初次位置放在 primary monitor work area 右下角，保留安全邊距，不蓋住 taskbar。
- 儲存 logical／DIP position、monitor identity 與 work-area normalized position，不只存 physical pixels。
- 至少在 launch、pet show／recovery、drag end、scale-factor change 與 single-instance callback 時重新驗證位置。Tauri 沒有完整的 monitor-topology／taskbar-change WindowEvent；若要求變更當下即時處理，實作 slice 必須選擇並驗證 Windows `WM_DISPLAYCHANGE`／`WM_SETTINGCHANGE` hook 或受控 polling，而不是假設 framework 已自動通知。
- 任何 render-state transition 若改變 pet logical bounds，Rust 必須先計算安全尺寸、保留可見 anchor、套用 resize，再依目前 monitor work area clamp；失敗時保留上一個可操作 bounds，不可讓唯一入口消失。
- 若原 monitor 不存在，移到 primary work area 的安全位置。
- 至少保留足夠可點擊／可拖曳區域在畫面內，禁止整隻落在螢幕外。
- 驗收 100%、125%、150%、200% 與混合 DPI 多螢幕。
- v1 不做 click-through；它容易讓寵物無法重新被點擊或拖曳。

官方 window-state plugin 可協助保存位置，但只允許 `StateFlags::POSITION` 且 filter／allowlist 僅追蹤 `pet`；`main` 必須排除。仍要補 monitor identity、normalized position、removal 與 work-area clamp，不能把 plugin restore 視為完整驗收。

## 10. Accessibility

- pet 視覺可 `aria-hidden`，外層使用原生 button／可聚焦控制提供名稱與簡短狀態。
- App 啟動時不搶走目前工作的 focus。
- pet window 已取得 focus 時，Enter／Space／`Shift+F10`／Menu key 開啟錨定於 pet 的原生選單；原生系統處理 Esc、方向鍵與 Enter。未聚焦時用 `Win+B` 進入系統匣或從 Start Menu／installed shortcut 找回，v1 不以未驗證的 global shortcut 假裝補足可及性。
- tray 與 Start Menu／packaged UAT 證實存在的 installed shortcut 是右鍵以外的完整 recovery path。
- screen reader 只播報狀態改變一次，不逐幀朗讀動畫。
- reduced-motion 模式直接顯示穩定姿態，不播放強烈進出場或 morph。
- 高對比、Narrator／NVDA、200% zoom 與鍵盤-only 都是 P0 驗收，不延後到視覺 polish。
- detail window 保留語義化 heading、landmark、focus order 與文字版「Why did this happen?」。

## 11. Proposed Tauri architecture

### Window configuration

建議保留現有 `main` label 作 detail，新增 `pet`：

| Setting | `pet` | `main` |
|---|---|---|
| route | `index.html?surface=pet` | `index.html?surface=detail` |
| initial visibility | true | false |
| transparent | true | false |
| decorations | false | true |
| resizable | false | true |
| always on top | true by default | false |
| skip taskbar | true | false |
| initial focus | false | only when explicitly opened |
| size | tight fixed bounds proven with P0 compact／wide／tall／long envelope fixtures, or Rust-owned resize＋re-clamp | current 1180 × 780 constraints |

Tauri 官方 configuration 支援 `transparent`、`decorations`、`alwaysOnTop`、`skipTaskbar` 與多個 unique window labels。`pet` 應明確設 `shadow: false`；Windows 透明視窗仍需實測白色 flash，`noRedirectionBitmap` 只是可評估的 workaround，不應未驗證就宣稱必要。`visibleOnAllWorkspaces` 不支援 Windows，因此產品不可承諾跨所有虛擬桌面永遠顯示。

### Surface routing

`main.tsx` 依 `surface` query／window label 渲染：

- `PetSurface`：只載入 render-safe `CreatureRenderState`、互動與 access-off tag；
- `DetailSurface`：沿用完整 App 與 FirstMemoryFlow；
- browser：維持 detail preview，明說 floating pet／native menu 不可用，不偽造多視窗行為。

目前全域 `html`／`body` 有最小寬度與深色背景；`PetSurface` 必須採獨立的透明 surface reset，否則 native window 透明也只會顯示成黑色矩形。

### Rust-owned lifecycle

視窗生命週期宜由 Rust 管理，而不是讓每個 WebView 取得廣泛 window mutation 權限：

- `open_detail_window`：取得預建的 `main`，依序 unminimize、show、focus，然後 hide `pet`；任一步失敗都要保留 pet／tray recovery，不能先把唯一入口藏掉；
- `return_to_pet`：hide `main`，validate position，show `pet`；
- `main` minimize／restore：minimize 顯示 pet，restore 隱藏 pet；
- `set_pet_visibility`／`set_pet_always_on_top`；
- `quit_memoryling`；
- 只攔 `main` 的 `CloseRequested`，把 X／Alt+F4 改為 cancel pending preview + hide；不可 blanket 阻擋 `ExitRequested`，tray Quit 與 Windows sign-out／shutdown 必須能結束；
- tray 與 single-instance callback 呼叫相同 Rust functions。

目前 capability 只有綁 `main` 的 `core:default`，而且 `invoke_handler` 註冊的 app commands 預設可被所有 windows／webviews 呼叫。實作時必須同時收窄兩層：

1. 以 `tauri_build::AppManifest::commands` 產生每個 app command 的 permissions；`main` 才能呼叫 `list_memory_sources`、`preview_memory_source`、`cancel_memory_preview`、`get_memory_state`、`approve_memory_import`、`forget_memory_source`，`pet` 只取得 `get_creature_render_state` 與必要的 menu／interaction command；
2. 每個敏感 Rust command 再檢查 caller window label 必須是 `main`，作為 defense in depth；
3. `pet` 與 `main` 都不複製 `core:default`，而是逐項列出確實使用的 core permissions。

若 pet frontend 直接呼叫 `startDragging`，只授予 `core:window:allow-start-dragging`；若接收 render-state revision，再只加 listen／unlisten event 權限。不要開放任意 create／destroy／resize、show／focus、menu 或 tray 權限。

### Native menu、tray 與 single instance

- Rust setup 建立並持有單一原生 context menu。`PetSurface` 的 `contextmenu` event 只呼叫狹窄 command；Rust 驗證 caller window label 必須是 `pet`，右鍵以 `Window::popup_menu` 顯示，鍵盤則以 `popup_menu_at` 錨定 pet。menu event 直接呼叫 Rust lifecycle，pet frontend 不取得 menu 或跨窗權限。
- tray 使用 Tauri desktop tray API；Rust dependency 需開啟 `tray-icon` feature。Rust 建立的 menu／tray 不需要 frontend capability。
- `tauri-plugin-single-instance` 應作為第一個 plugin 註冊；第二次啟動只喚醒既有 process 並聚焦 detail，不記錄或轉發 argv、cwd 或可能含私密路徑的值。
- `tauri-plugin-window-state` 可作位置保存候選，優先由 Rust 使用，避免為 frontend 擴大 plugin permissions；使用 `StateFlags::POSITION` 並以 filter／allowlist 只追蹤 `pet`，排除 `main`，不還原 ALL／VISIBLE。

不新增網路、telemetry、global shortcut、autostart 或任意 filesystem capability。

## 12. Cross-window state

Rust／SQLite 仍是唯一 canonical state。現有完整 `MemoryState` 的 lineage 含核准後文字，不能交給 pet surface；需新增內容最小化的 typed DTO：

1. detail 啟動時呼叫既有完整 state command；pet 的 app-command capability 只允許 `get_creature_render_state`，其中不含 normalized text、locator、path 或 explanation content；
2. approve、forget 或未來 genome revision commit 成功後，Rust 發送只含 opaque revision／counts 的 `creature-state-changed` event；
3. 兩個 surfaces 收到事件後分別重新讀取其 scope 允許的 typed state；
4. event 不攜帶 normalized text、source path 或 memory payload；
5. pet renderer 只取得最終 render-safe mark IDs、visual-module IDs、受限 geometry／motion parameters 與 opaque revision，不取得 route profile ID／權重、來源細節、原始 activity labels 或人格摘要。

detail 以 X／Alt+F4 關閉時，Rust close-request path 必須先在 backend 明確取消 pending preview，再 hide；`hide(main)` 不會 unmount WebView，不能依賴 React cleanup。Minimize 不取消 preview。若取消或後續 hide 失敗，回傳狀態必須維持一致並保留 tray recovery。

locale、always-on-top、onboarding completion 與 pet position 屬本機 UI settings，不可混入 memory-derived genome 或 lineage。Always-on-top 的 Rust canonical setting 同步驅動 pet context menu 與 tray 的 checked state，不能各自保存兩份值。

## 13. Privacy and presentation

- pet、menu、tray 與 OS window title 不顯示來源內容、人物、專案名稱或敏感 trait。
- pet-only state 只表達 render-safe 外觀與中性狀態。
- screen sharing privacy mode 屬後續 P1，但公開測試前必須完成。
- detail hidden 不代表資料從記憶體消失；pending preview lifecycle 必須有測試與清楚文案。
- 關閉 detail、隱藏 pet、quit 與 forget source 是四個不同操作；不可混淆。

## 14. Delivery plan

此功能不是 CSS 改版，而是新的 Windows resident-app lifecycle。建議在 real-source connector 前，用 synthetic fixture 完成以下垂直切片：

### Slice A — Two-window shell

- `pet`／`main` window configuration；
- surface routing；
- 預建隱藏的 `main` 與 Rust show／hide／focus lifecycle；
- main close／minimize／restore → pet return／hide；
- app-command permissions、caller-label defense 與從 pet invoke 敏感 command 的 fail-closed tests；
- browser truth boundary；
- unit tests + native desktop smoke。

### Slice B — Native access and recovery

- native right-click menu；
- caller-label validation、最小 pet／main core capabilities 與 app-command permissions；
- keyboard equivalents；
- tray Show／Hide／Open／Quit；
- single-instance Start／installed shortcut relaunch；
- access-off status與首次 onboarding。

### Slice C — Position and state continuity

- packaged Windows drag-vs-click spike and fallback interaction；
- position persistence；
- launch／show／drag-end／scale-change clamp 與 Windows display／taskbar-change trigger spike；
- Rust revision event + pet refetch；
- approve／forget 後 pet mark 即時同步；
- 以 route-agnostic 的 compact／wide／tall／long synthetic envelope states 驗證固定 envelope 或 Rust-owned resize，並在每次 bounds 變更後重新 clamp；實際 route matrix 留到 Phase 2；
- restart persistence；WebView process-failure recovery remains a separately tested P1。

### Slice D — Detail information architecture

- 把現有長頁整理為 Overview／Memories／Growth／Habitat／Privacy；
- 不改動既有 fixture lineage 與 forgetting truth；
- 英文／繁中 parity、keyboard、Narrator／NVDA 與 200% zoom。

### Slice E — Windows packaging UAT

- current-user install；
- Start Menu／實際安裝的 app shortcut relaunch；
- pet launch、menu、dashboard、close-return、tray、quit；
- explicit Quit and Windows sign-out／shutdown；
- 100–200% DPI、多螢幕移除、taskbar relocation；
- uninstall data-retain／delete choices；
- rebuild NSIS and regenerate checksum。

每個 slice 必須完成產品路徑、Rust／React tests、native desktop smoke、文件與真實狀態更新，不能只留下 window connector 骨架。

## 15. Acceptance criteria

- fresh launch 只顯示一個 pet surface，內含必要的 compact access-off badge，不自動開 dashboard。
- 右鍵能在游標處開 menu；pet 已 focus 時 Enter／Space／`Shift+F10` 能在 pet 旁開 menu；Open Memoryling 是第一個 enabled item。
- `Win+B` tray、Start Menu 與 packaged UAT 證實存在的 installed shortcut 都能找回 app。
- Open 只顯示／聚焦一個 detail window，pet 同時收起。
- detail minimize 顯示 pet，restore 再收起 pet；X／Alt+F4 取消 backend pending preview 並回到原位置的 pet。
- tray Quit 才結束 process；resident close logic 不阻擋 Windows sign-out／shutdown。
- 再次啟動不產生第二個 process、pet 或 SQLite writer。
- 從 `pet` 逐一 invoke 既有 memory list／preview／cancel／state／approve／forget commands 全部 fail closed；只檢查正常 DTO 不算通過。
- pet window 的外框不得在 visible pet＋honesty badge 的聯集外多出超過 12 DIP 的透明 margin；packaged smoke 要點擊外框四周相鄰桌面，確認沒有更大的隱形攔截區。
- route-agnostic 的 compact／wide／tall／long envelope fixtures 都要在 100–200% DPI 驗證 bounds、拖曳、右鍵 menu anchor 與 work-area clamp；不得只測最緊湊的 baseline。實際 route／hybrid matrix 由 Phase 2 acceptance 負責。
- 混合 DPI、monitor removal 與 taskbar relocation 後 pet 仍可見可操作。
- fixture approve／forget 後 pet mark 與 detail explanation 一致；失敗交易不改 UI。
- pet IPC／events 不含核准記憶文字、來源 locator、路徑或 lineage explanation。
- browser 不假裝支援 native floating pet。
- runtime 無網路請求、telemetry 或圖片／LLM API。
- memory access off 在 real connector 前保持可見。
- 開啟 detail 任一步驟失敗時，pet 或 tray 仍可見且可重試。

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
- 不在沒有選定視覺目標前重畫 production pet。

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
