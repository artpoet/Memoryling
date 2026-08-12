# Memoryling Windows x64 測試版使用指南

[English](../USER_GUIDE.md) · [專案 README](../../README.zh-TW.md) · [隱私原則](../PRIVACY_PRINCIPLES.md)

## 請先閱讀

唯一完成安裝 UAT 的 packaged Memoryling build，仍是 **pet-first、僅使用 fixture 的 Windows x64 v0.2.0 測試 artifact**。它的原生浮動寵物外殼，以及針對一筆隨 App 打包之虛構紀錄的本機預覽、核准、SQLite 持久化、來源解釋與遺忘流程，都已可運作。

它**不會**讀取真實 Codex 記憶、不會掃描 Codex tool-home、不接受任意檔案，也沒有連接正式記憶來源。即使合成 fixture 試行正在運作，App 仍必須顯示「持久記憶存取關閉」。

另外，Repo source tree 已進入 v0.3.0，並為開發者評估實作了綁定特定版本的實驗性 **Codex 工作紀錄／thread history** 試行。它不在已測試的 v0.2.0 安裝器內、不是持久記憶 connector，也尚未執行獲授權的私人 thread UAT。詳見[僅供開發版 source 使用：實驗性 Codex 工作紀錄試行](#僅供開發版-source-使用實驗性-codex-工作紀錄試行)。

這個測試版尚未簽章，也不是已可公開發布的正式安裝包。

## 測試使用者的正式入口

一般測試請使用目前使用者（current-user）的 NSIS 安裝程式：

    Memoryling_0.2.0_x64-setup.exe

開發機本機產出的檔案位於：

    src-tauri\target\release\bundle\nsis\Memoryling_0.2.0_x64-setup.exe

不要把這個 Repo 路徑當成公開下載網址。`src-tauri/target/` 底下是未 commit 的本機 build output。

### Raw exe 不是 portable 版本

`src-tauri\target\release\memoryling.exe` 是 build output，不是建議給測試者使用的入口，也不是單檔 portable 安裝包。fixture-only runtime 需要 raw exe 旁保留這個 sidecar 資源：

    src-tauri\target\release\fixtures\codex-first-memory-v1.json

若只移動或分享 `memoryling.exe`，fixture 可能無法使用。除非正在除錯 build tree，否則請使用 NSIS 安裝程式；若要執行 raw exe，必須保留產出時的 sidecar 目錄結構。

## 安裝前的安全提醒

- 目前測試安裝程式沒有程式碼簽章。Windows 可能顯示 **Unknown publisher（未知的發行者）** 或 Microsoft Defender SmartScreen 警告。
- 不要為了 Memoryling 關閉 SmartScreen、防毒軟體或其他 Windows 防護；也不要只因為看過這份指南就略過安全警告。
- 只安裝從你信任的專案管道取得的 artifact。若無法確認檔案來源或身分，請取消安裝。
- 2026-08-12 完成的本機 v0.2.0 測試安裝器大小為 2,875,965 bytes，SHA-256 是 `BFB2A08D272CDEF64C59C84D30389D99E2EB6A74EC45E97209EFDD906CF6DFCD`。每次重建後都必須重新核對 hash。
- 此 artifact 針對 Windows x64 建置；本測試包不涵蓋其他 Windows 架構。

## 安裝與開啟

1. 在 Windows 檔案總管雙擊 `Memoryling_0.2.0_x64-setup.exe`。
2. 閱讀每一個 Windows 安全提示。若無法確認 artifact 來源，請停止；本指南不建議繞過 Windows 防護。
3. 以英文或繁體中文完成目前使用者（current-user）安裝流程。
4. 若電腦缺少 Microsoft Edge WebView2，安裝器已設定為下載並安裝 Microsoft 的 WebView2 bootstrapper；這個前置步驟可能需要網路連線。
5. 完成安裝後，從一般 Windows 開始功能表或桌面捷徑開啟 **Memoryling**。

可能發生的 WebView2 下載屬於安裝前置需求。fixture 記憶流程本身沒有網路 client，也不會把記憶內容送出網路。

2026-08-12 已用這個完全相同的 v0.2.0 artifact，完成由檔案總管啟動的 current-user 安裝，並安裝到實際的目前使用者 LocalAppData 位置。一般開始功能表與桌面捷徑都存在，HKCU 解除安裝登錄顯示版本 0.2.0。從捷徑 cold launch 時先顯示 pet；程式已常駐時再次啟動仍維持 single-instance；右鍵 → **開啟 Memoryling** 會開啟詳細視窗，關閉詳細視窗會回到 pet，明確選擇 **結束 Memoryling** 後 process 不再存在。解除安裝時不勾選 **刪除應用程式數據**，程式、HKCU 解除安裝登錄、捷徑與 process 都已移除，而 `%LOCALAPPDATA%\app.memoryling.desktop` 仍保留；全程只檢查檔案系統與登錄 metadata，沒有讀取資料庫內容。

較早一次由代理直接啟動安裝器的嘗試受到 Windows virtualization 影響；其殘影已移除，且不列入產品證據。WebView2 缺失分支仍需安全的拋棄式 Windows x64 環境才能補測，不得為了測試而移除目前主機已安裝的 runtime。這些證據不代表未簽章版本已達公開發布品質。

完全相同的 v0.2.0 artifact、size、hash，以及已完成的安裝／lifecycle／保留資料解除安裝證據，都是禁止重做的基準線。除非 artifact 或相關 packaging 行為改變，否則不要重新 build 或重複該項驗收。

## 使用 pet-first 外殼

1. 一般啟動會先顯示浮動 pet，不會直接開啟完整詳細視窗。符合條件的第一次啟動也可能顯示一次性的雙語指南。
2. 若要開啟詳細內容，先在 pet 上按右鍵，再選擇 **開啟 Memoryling**。pet 已有鍵盤焦點時，Enter、Space、Menu key 或 `Shift+F10` 會開啟同一個原生選單。
3. 拖曳 pet 可移動位置。關閉或最小化詳細視窗會回到 pet；開啟或還原詳細視窗會隱藏 pet，因此一般狀態只會看到一個 Memoryling 表面。
4. 可透過原生選單或系統匣顯示、隱藏或開啟 Memoryling。`Win+B` 可前往 Windows 系統匣，但這條精確的鍵盤救援路徑仍需專門的 packaged acceptance。
5. 選擇 **結束 Memoryling** 才會結束常駐 process。隱藏 pet 或關閉詳細內容都不等於 Quit。

## 執行 fixture-only 記憶流程

下方來源與紀錄都是 Repo 內的虛構 fixture。

### 1. 預覽

1. 若詳細視窗尚未開啟，先在 pet 上按右鍵，再選擇 **開啟 Memoryling**。
2. 先確認狀態仍顯示「真實記憶存取關閉」。
3. 選取 **Codex · First memory fixture**。
4. 檢查精確存取範圍：它應說明只讀取一個隨 App 打包的合成 JSON fixture、不能掃描任意路徑或 Codex tool-home、不能修改來源，也不會發出網路請求。
5. 選取 **預覽所選來源**。

預覽內容會在 Rust 記憶體中準備。預覽不等於核准，也不會把 fixture 的來源內容持久化。

### 2. 核准並儲存在本機

1. 檢查虛構紀錄、時間、類型與縮短顯示的 content hash。
2. 保持 **納入這筆紀錄** 為已選取。
3. 明確勾選同意選項。
4. 選取 **核准並在本機儲存 1 筆記憶**。

Memoryling 會把核准後的正規化 fixture 紀錄與來源鏈存入 App 本機 SQLite。這是真正的本機持久化，但輸入仍只有合成 fixture；它不代表 App 已存取使用者的 Codex 記憶。

### 3. 解釋印記

1. 確認記憶獸身上出現完成之星。
2. 選取 **為什麼會發生？**
3. 檢查「來源 → 正規化事件 → 確定性訊號 → 記憶獸影響」來源鏈。

畫面狀態必須持續把 fixture 試行與真實記憶存取分開顯示。

### 4. 遺忘本機匯入副本

1. 選取 **遺忘這個來源**。
2. 閱讀並勾選刪除確認。
3. 選取 **遺忘來源並移除印記**。
4. 確認匯入來源、正規化事件、訊號與完成之星都從 Memoryling 狀態消失。

遺忘流程會移除 App 的本機匯入副本與目前支援的下游來源鏈；它不會修改或刪除隨安裝包附帶的唯讀 fixture，也不是實體儲存媒體不可恢復抹除的保證。

## 僅供開發版 source 使用：實驗性 Codex 工作紀錄試行

OpenAI 目前並未針對這項整合公開穩定的 Codex 持久記憶匯出 API，或具有相容性保證的記憶檔案 schema。因此，v0.3.0 source implementation 透過狹窄的本機 App Server stdio 邊界評估的是 **Codex 工作紀錄／thread history**，不是「Codex 記憶」。App Server host 本身仍屬實驗性，也不支援 production workload。

這項試行採 fail-closed，並要求標準本機 Codex Desktop CLI 必須完全回報 `codex-cli 0.134.0`；它不宣稱支援更早或更晚的 CLI 版本。開發版預期流程如下：

1. 啟動目前 source build，並由使用者主動選擇 **瀏覽本機 Codex 工作紀錄**。App 不會在啟動時或背景列出任何項目。
2. 檢查內容最小化的 catalog。候選項目只有中性標籤與有限的來源／時間 metadata，不會揭露 thread 標題、摘要、Repo 路徑、原始識別碼、提示詞、回覆或工具輸出。
3. 明確選取一個候選項目。只有此時，backend 才能透過本機 stdio，針對這一個 thread 呼叫 `thread/read`。
4. 若 thread 已完成，backend 只考慮最後一個 completed turn 中，phase 為 `final_answer` 的最終 `agentMessage`。使用者提示詞、commentary、reasoning、指令、工具輸出與其他 items 全部排除。
5. 檢查遮蔽內容後的 preview 與精確同意範圍。preview 只回報有限的數量、來源／時間 metadata、預計 completion event 數量與排除的資料類別，不會顯示所選 thread 文字。
6. 確認所選工作已完成，並給予精確同意。只有此時，該 final answer 內容才能被正規化，並持久化在 Memoryling 的 App 本機 SQLite；它不會透過 frontend IPC 回傳，也不會顯示在 UI。
7. 使用 **遺忘**，移除 Memoryling 的本機匯入副本、來源鏈與目前支援的下游影響。這不會編輯、歸檔、刪除或以其他方式改變原始 Codex thread。

adapter 為唯讀，不接受任意路徑、不掃描 Codex 持久記憶檔案、不會呼叫模型或外部網路、不會寫入來源，而且同一時間只允許一個核准來源。若要換來源，必須先遺忘目前的 Memoryling 副本，再重新完成瀏覽、選取、預覽、同意與匯入。

持久記憶存取必須持續顯示為關閉。截至 2026-08-12，私人 thread UAT 尚未獲得授權，也尚未執行。在使用者明確指定來源／thread 範圍前，不得為驗收而瀏覽或讀取私人 thread；即使日後 UAT 通過，也只代表這個固定版本的實驗性試行獲得驗證，不代表正式 connector 或 Phase 1 已完成。

## 本機資料與解除安裝

Memoryling 的目前使用者 App 資料位於：

    %LOCALAPPDATA%\app.memoryling.desktop

這個資料夾可能包含：

- `memoryling.sqlite3`：核准後的正規化 fixture 文字、hash、來源鏈與衍生狀態；v0.3.0 開發版 source build 也可能保留唯一選取的 final answer 正規化文字，但只會發生在完成上述明確同意之後；
- `desktop-shell-v1.json` 與可能存在的 `desktop-shell-v1.json.bak`：只包含 onboarding、always-on-top 與安全 pet 位置等本機 shell 設定；
- `EBWebView` 等 WebView runtime 資料。

一般的 **遺忘這個來源** 流程會移除匯入 fixture 紀錄與目前支援的下游影響，但可能留下空的資料庫與 WebView 資料夾。

產生出的解除安裝程式包含 **刪除應用程式數據（Delete the application data）** 選項。v0.2.0 原生 UAT 已驗證：不勾選時會保留 App-data tree，同時移除已安裝程式、HKCU 登錄、捷徑與 process；全程只檢查 metadata，沒有讀取資料庫內容。歷史 v0.1.0 的兩輪 UAT 也曾驗證勾選此選項會移除整個 App-data tree，但這項舊結果不代表目前 v0.2.0 的刪除資料路徑已重新測過。若你在意資料是否刪除，解除安裝後仍應檢查 `%LOCALAPPDATA%\app.memoryling.desktop`。這是應用程式層級的清理證據，不是實體安全抹除保證。

不得分享、附加、印出或 commit 真實本機資料庫。雖然目前 fixture 是虛構資料，同一位置仍是未來敏感本機狀態的保留區。

## 疑難排解邊界

- **App 顯示「持久記憶存取關閉」：**這是正確狀態；已測試的 v0.2.0 安裝器沒有真實來源 connector，而 v0.3.0 source 試行讀取的是使用者明確選取的工作紀錄，不是 Codex 持久記憶。
- **Codex 工作紀錄瀏覽器顯示 CLI 版本不受支援：**除非標準本機 Codex Desktop CLI 完全回報 `codex-cli 0.134.0`，否則這是預期的 fail-closed 行為。不要繞過 pin，也不要把 App 指向任意執行檔。
- **瀏覽器預覽維持詳細版面：**這是正確狀態；browser mode 不會假裝有原生浮動 pet、context menu、系統匣、single-instance lifecycle、SQLite 或持久化。
- **WebView2 安裝失敗：**只透過可信任安裝器與可信任網路重試，或從 Microsoft 官方管道取得 WebView2；不要使用不明第三方 runtime 下載。
- **Windows 阻擋未簽章安裝器：**不要降低系統防護。請向專案擁有者確認 artifact，或等待已簽章、達公開發布品質的版本。
- **複製 raw exe 後找不到 fixture：**請還原產出時的 `fixtures` sidecar 結構，或改用 NSIS 安裝程式；raw exe 不是 portable 發布包。

## 開發者建置

在 Repo 根目錄執行：

    npm install
    npm run check
    npm run tauri dev

唯一完成原生 UAT 的安裝器仍是既有 v0.2.0 artifact：

    src-tauri\target\release\bundle\nsis\Memoryling_0.2.0_x64-setup.exe

`npm run build:windows` 會使用目前 source version 建置，產生新的、跟隨 source version 且尚未驗證的 artifact；它不會重現上方已測試的精確 v0.2.0 檔案。任何 rebuilt artifact 都必須先針對其精確 bytes 完成必要的原生安裝器 click-through，並記錄 size、checksum 與 CI 證據，才能宣稱受支援。不得以 raw release exe 取代安裝器。

## 測試版圖像狀態

這個測試版的 icon 與 App 內品牌圖像由 Codex 內建 ImageGen 產出。已檢查來源 PNG 與產生的 PNG icon assets，確認包含 alpha channel 與透明像素。

這只是透明背景的技術檢查，不等於已核准公開發布。圖像、未簽章安裝器、商店呈現、授權審查與散布流程都尚未被宣稱為 public release-ready。
