import { useEffect, useMemo, useRef, useState } from "react";
import "./FirstMemoryFlow.css";
import type {
  CodexThreadCatalog,
  ImportPreview,
  MemoryClient,
  MemoryState,
  SourceOption,
} from "./memoryClient";

export type MemoryLocale = "en" | "zh-TW";

interface FirstMemoryFlowProps {
  locale: MemoryLocale;
  client: MemoryClient;
  memoryState: MemoryState;
  onMemoryStateChange: (state: MemoryState) => void;
  resetRevision?: number;
}

const THREAD_ADAPTER_ID = "codex-app-server-thread";

const flowCopy = {
  en: {
    eyebrow: "FIRST SOURCE · EXPLICIT LOCAL CONSENT",
    title: "From approved work to an explainable mark",
    body:
      "Try the bundled synthetic fixture, or explicitly choose one local Codex work record. Codex durable memories are not connected.",
    localBadge: "LOCAL SQLITE · LOCAL IMPORT",
    booting: "Opening the local memory store…",
    unavailableTitle: "Desktop runtime required",
    unavailableBody:
      "Memory access remains off in the browser preview. Browsing Codex work records is unavailable here.",
    stepOne: "1 · Select source",
    chooseSource: "Nothing is read until you choose an action.",
    fixtureBadge: "SYNTHETIC FIXTURE",
    fixtureDescription: "Bundled fictional record for a safe end-to-end demonstration.",
    adapter: "Adapter",
    workTitle: "Codex work records",
    experimental: "EXPERIMENTAL",
    workDescription:
      "These are thread work records, not Codex durable memories. This pilot requires the standard Windows Codex Desktop install with Codex CLI exactly 0.134.0 and fails closed on any other version. Browse returns at most 25 neutral candidates without titles, summaries, paths, raw IDs, or transcript content.",
    browse: "Browse local Codex work records",
    browseAgain: "Refresh work-record list",
    browseDisclosure:
      "After you select one record, Rust uses thread/read locally to inspect that thread's full history and extract only its last completed final answer. Other turns and items are not retained or sent to the WebView.",
    catalogTitle: "Choose one work record",
    candidateKind: "Local Codex thread",
    candidatePreview: "Review this record",
    cancelBrowse: "Cancel browse",
    noCandidates: "No eligible local work records were returned.",
    scopeTitle: "Exact access boundary",
    scopeItems: [
      "No Codex durable-memory access or arbitrary file scanning",
      "No source writes; this import makes no network request",
      "Only an explicitly selected record may be inspected",
      "Preview remains temporary until exact consent is approved",
    ],
    nothingStored: "Nothing has been stored yet.",
    previewFixture: "Preview synthetic fixture",
    stepTwo: "2 · Review preview",
    fixturePreviewTitle: "Review the synthetic content before storage",
    workPreviewTitle: "Review the redacted work-record preview",
    fixtureRecords: (count: number) =>
      `${count} synthetic record${count === 1 ? "" : "s"}`,
    workRecords: (count: number) => `${count} selected work record${count === 1 ? "" : "s"}`,
    timeRange: "Time",
    completion: "Completion",
    category: "Category",
    characterCount: "Characters",
    hash: "SHA-256",
    purpose: "Purpose",
    scopeRevision: "Consent scope",
    readOnly: "Read-only source",
    selectRecord: "Include this record",
    redacted: "CONTENT HIDDEN FROM THE WEBVIEW",
    fixtureConsent:
      "I approve storing the selected synthetic record in Memoryling's local SQLite database and deriving a completion star with lineage.",
    workConsent:
      "I confirm this work record represents a completed outcome and approve storing its extracted final answer in Memoryling's local SQLite database, deriving a completion star, and retaining source lineage until I forget it.",
    approveButton: (count: number) => `Approve & store ${count} record locally`,
    cancelPreview: "Cancel preview",
    stepThree: "3 · Stored and derived",
    fixtureStoredTitle: "One approved synthetic memory left a completion star",
    workStoredTitle: "One approved Codex work record left a completion star",
    fixtureStoredBody:
      "The mark below came from persisted Rust state, not browser storage.",
    workStoredBody:
      "One selected final answer is stored locally. Codex durable-memory access remains off, and raw content stays hidden from this interface.",
    sourceMetric: "Approved sources",
    eventMetric: "Memory events",
    signalMetric: "Derived signals",
    markMetric: "World effects",
    whyShow: "Why did this happen?",
    whyHide: "Hide lineage",
    lineageTitle: "Privacy-safe lineage",
    sourceNode: "Approved source record",
    eventNode: "Normalized memory event",
    signalNode: "Deterministic signal",
    effectNode: "Creature world effect",
    redactedLineage: "Stored content hidden",
    schema: "schema",
    derivation: "derivation",
    confidence: "confidence",
    forgetStart: "Forget this source",
    forgetTitle: "Complete forgetting",
    fixtureForgetBody:
      "This removes the local source, event, signal, and star in one transaction. The bundled fixture is unchanged.",
    workForgetBody:
      "This removes the selected work record, event, signal, completion star, and lineage from Memoryling in one transaction. It does not modify Codex.",
    forgetConsent: "I understand that the local imported record and its mark will be removed.",
    forgetConfirm: "Forget source and remove mark",
    keepMemory: "Keep record",
    genericError:
      "The local operation did not complete. No private error details were shown and no UI state was approved.",
    approvedNotice: "Approved locally. The persisted mark is now active.",
    forgottenNotice: "Forgotten completely. Source access is off again.",
    working: "Working locally…",
  },
  "zh-TW": {
    eyebrow: "第一個來源 · 明確本機同意",
    title: "從核准工作，到可解釋的印記",
    body:
      "你可以試用打包的合成 fixture，或明確選擇一筆本機 Codex 工作紀錄；Codex durable memories 尚未連線。",
    localBadge: "本機 SQLITE · 本機匯入",
    booting: "正在開啟本機記憶資料庫…",
    unavailableTitle: "需要桌面版執行環境",
    unavailableBody: "瀏覽器預覽維持記憶存取關閉，也不能瀏覽 Codex 工作紀錄。",
    stepOne: "1 · 選擇來源",
    chooseSource: "在你主動選擇操作前，不會讀取任何內容。",
    fixtureBadge: "合成 FIXTURE",
    fixtureDescription: "隨 App 打包的虛構紀錄，用於安全展示完整流程。",
    adapter: "Adapter",
    workTitle: "Codex 工作紀錄",
    experimental: "實驗性",
    workDescription:
      "這是 thread 工作紀錄，不是 Codex durable memories。此 pilot 只支援標準 Windows Codex Desktop 安裝中的 Codex CLI 0.134.0 exact version；其他版本一律拒絕。按下瀏覽後，此介面最多只收到 25 筆中性候選，不顯示標題、摘要、路徑、原始 ID 或 transcript 內容。",
    browse: "瀏覽本機 Codex 工作紀錄",
    browseAgain: "重新整理工作紀錄清單",
    browseDisclosure:
      "選定一筆後，Rust 才會在本機用 thread/read 檢查該 thread 的完整歷史，只擷取最後一個已完成的 final answer；其他 turn 與 item 不會保留，也不會送進 WebView。",
    catalogTitle: "選擇一筆工作紀錄",
    candidateKind: "本機 Codex thread",
    candidatePreview: "檢查這筆紀錄",
    cancelBrowse: "取消瀏覽",
    noCandidates: "未取得可用的本機工作紀錄。",
    scopeTitle: "精確存取邊界",
    scopeItems: [
      "不存取 Codex durable memory，也不掃描任意檔案",
      "不修改來源；這個匯入流程不發出網路請求",
      "只檢查你明確選定的單一紀錄",
      "精確核准前，預覽只會暫存在記憶體",
    ],
    nothingStored: "目前尚未儲存任何內容。",
    previewFixture: "預覽合成 fixture",
    stepTwo: "2 · 檢查預覽",
    fixturePreviewTitle: "儲存前檢查合成內容",
    workPreviewTitle: "檢查已遮罩的工作紀錄預覽",
    fixtureRecords: (count: number) => `${count} 筆合成紀錄`,
    workRecords: (count: number) => `${count} 筆所選工作紀錄`,
    timeRange: "時間",
    completion: "完成事件",
    category: "類別",
    characterCount: "字元數",
    hash: "SHA-256",
    purpose: "用途",
    scopeRevision: "同意範圍",
    readOnly: "唯讀來源",
    selectRecord: "納入這筆紀錄",
    redacted: "內容不會顯示於 WEBVIEW",
    fixtureConsent:
      "我核准將所選合成紀錄存入 Memoryling 本機 SQLite，並建立帶有來源鏈的完成之星。",
    workConsent:
      "我確認這筆工作紀錄代表已完成成果，並核准將擷取的 final answer 存入 Memoryling 本機 SQLite、衍生完成之星，且保留來源鏈直到我執行遺忘。",
    approveButton: (count: number) => `核准並在本機儲存 ${count} 筆紀錄`,
    cancelPreview: "取消預覽",
    stepThree: "3 · 已儲存並衍生",
    fixtureStoredTitle: "一筆核准的合成記憶留下了完成之星",
    workStoredTitle: "一筆核准的 Codex 工作紀錄留下了完成之星",
    fixtureStoredBody: "下方印記來自 Rust 回傳的持久化狀態，不是瀏覽器儲存。",
    workStoredBody:
      "一個所選 final answer 已儲存在本機。Codex durable-memory 存取仍關閉，原始內容也不會顯示在此介面。",
    sourceMetric: "核准來源",
    eventMetric: "記憶事件",
    signalMetric: "衍生訊號",
    markMetric: "世界影響",
    whyShow: "為什麼會發生？",
    whyHide: "收起來源鏈",
    lineageTitle: "保護隱私的來源鏈",
    sourceNode: "已核准來源紀錄",
    eventNode: "正規化記憶事件",
    signalNode: "確定性衍生訊號",
    effectNode: "記憶獸世界影響",
    redactedLineage: "已儲存內容維持遮罩",
    schema: "schema",
    derivation: "derivation",
    confidence: "信心值",
    forgetStart: "遺忘這個來源",
    forgetTitle: "完整遺忘",
    fixtureForgetBody:
      "這會在同一個 transaction 中移除本機來源、事件、訊號與星星；打包的 fixture 不會被修改。",
    workForgetBody:
      "這會在同一個 transaction 中移除 Memoryling 內的工作紀錄、事件、訊號、完成之星與來源鏈，不會修改 Codex。",
    forgetConsent: "我了解本機匯入的紀錄與其印記都會被移除。",
    forgetConfirm: "遺忘來源並移除印記",
    keepMemory: "保留紀錄",
    genericError: "本機操作未完成；未顯示私密錯誤細節，也沒有核准任何畫面狀態。",
    approvedNotice: "已在本機核准；持久化印記現在已生效。",
    forgottenNotice: "已完整遺忘；來源存取再次關閉。",
    working: "正在本機處理…",
  },
} as const;

function formatTimestamp(value: string, locale: MemoryLocale) {
  const timestamp = new Date(value);
  if (Number.isNaN(timestamp.getTime())) {
    return locale === "en" ? "Invalid source timestamp" : "無效的來源時間";
  }
  return new Intl.DateTimeFormat(locale, {
    dateStyle: "medium",
    timeStyle: "short",
    timeZone: "UTC",
  }).format(timestamp);
}

function shortHash(value: string) {
  return `${value.slice(0, 12)}…`;
}

function FirstMemoryFlow({
  locale,
  client,
  memoryState,
  onMemoryStateChange,
  resetRevision = 0,
}: FirstMemoryFlowProps) {
  const t = flowCopy[locale];
  const [sources, setSources] = useState<SourceOption[]>([]);
  const [selectedSourceId, setSelectedSourceId] = useState("");
  const [catalog, setCatalog] = useState<CodexThreadCatalog | null>(null);
  const [selectedCandidateId, setSelectedCandidateId] = useState("");
  const [preview, setPreview] = useState<ImportPreview | null>(null);
  const [selectedRecordIds, setSelectedRecordIds] = useState<string[]>([]);
  const [consent, setConsent] = useState(false);
  const [forgetConsent, setForgetConsent] = useState(false);
  const [showForget, setShowForget] = useState(false);
  const [showWhy, setShowWhy] = useState(false);
  const [initializing, setInitializing] = useState(client.available);
  const [working, setWorking] = useState(false);
  const [error, setError] = useState(false);
  const [notice, setNotice] = useState<"approved" | "forgotten" | null>(null);
  const operationGeneration = useRef(0);

  useEffect(() => {
    let active = true;
    const generation = operationGeneration.current;
    if (!client.available) return () => undefined;
    setInitializing(true);
    Promise.all([client.listSources(), client.getState()])
      .then(([availableSources, persistedState]) => {
        if (!active || generation !== operationGeneration.current) return;
        setSources(availableSources);
        onMemoryStateChange(persistedState);
        setError(false);
      })
      .catch(() => {
        if (active && generation === operationGeneration.current) setError(true);
      })
      .finally(() => {
        if (active && generation === operationGeneration.current) setInitializing(false);
      });
    return () => {
      active = false;
    };
  }, [client, onMemoryStateChange]);

  useEffect(() => {
    if (resetRevision === 0) return;
    operationGeneration.current += 1;
    setSelectedSourceId("");
    setCatalog(null);
    setSelectedCandidateId("");
    setPreview(null);
    setSelectedRecordIds([]);
    setConsent(false);
    setForgetConsent(false);
    setShowForget(false);
    setShowWhy(false);
    setInitializing(false);
    setWorking(false);
    setError(false);
    setNotice(null);
  }, [resetRevision]);

  const selectedSource = useMemo(
    () => sources.find((source) => source.id === selectedSourceId),
    [selectedSourceId, sources],
  );
  const mark = memoryState.marks[0];
  const lineage = mark?.lineage[0];
  const previewIsThread = preview?.source.adapterId === THREAD_ADAPTER_ID;
  const storedIsThread = lineage?.adapterId === THREAD_ADAPTER_ID;

  async function handleFixturePreview() {
    if (!selectedSource) return;
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    setNotice(null);
    try {
      const nextPreview = await client.previewSource(selectedSource.id);
      if (generation !== operationGeneration.current) return;
      setPreview(nextPreview);
      setCatalog(null);
      setSelectedRecordIds(nextPreview.records.map((record) => record.id));
      setConsent(false);
    } catch {
      if (generation !== operationGeneration.current) return;
      setError(true);
      setPreview(null);
      setSelectedRecordIds([]);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleFixtureSelection(sourceId: string) {
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    try {
      if (catalog) await client.cancelPreview(catalog.catalogId);
      if (generation !== operationGeneration.current) return;
      setSelectedSourceId(sourceId);
      setCatalog(null);
      setSelectedCandidateId("");
      setNotice(null);
    } catch {
      if (generation === operationGeneration.current) setError(true);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleBrowse() {
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    setNotice(null);
    setSelectedSourceId("");
    setSelectedCandidateId("");
    try {
      if (catalog) await client.cancelPreview(catalog.catalogId);
      if (generation !== operationGeneration.current) return;
      const nextCatalog = await client.listCodexThreads();
      if (generation !== operationGeneration.current) return;
      setCatalog(nextCatalog);
    } catch {
      if (generation !== operationGeneration.current) return;
      setCatalog(null);
      setError(true);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleCancelBrowse() {
    if (!catalog) return;
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    try {
      await client.cancelPreview(catalog.catalogId);
      if (generation !== operationGeneration.current) return;
      setCatalog(null);
      setSelectedCandidateId("");
    } catch {
      if (generation !== operationGeneration.current) return;
      setError(true);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleThreadPreview() {
    if (!catalog || !selectedCandidateId) return;
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    setNotice(null);
    try {
      const nextPreview = await client.previewCodexThread(
        catalog.catalogId,
        selectedCandidateId,
      );
      if (generation !== operationGeneration.current) return;
      setPreview(nextPreview);
      setCatalog(null);
      setSelectedRecordIds(nextPreview.records.map((record) => record.id));
      setConsent(false);
    } catch {
      if (generation !== operationGeneration.current) return;
      setCatalog(null);
      setSelectedCandidateId("");
      setError(true);
      setPreview(null);
      setSelectedRecordIds([]);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleCancelPreview() {
    const previewId = preview?.previewId;
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    try {
      if (previewId) await client.cancelPreview(previewId);
      if (generation !== operationGeneration.current) return;
      setPreview(null);
      setSelectedRecordIds([]);
      setConsent(false);
    } catch {
      if (generation !== operationGeneration.current) return;
      setError(true);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleApprove() {
    if (!preview || !consent || selectedRecordIds.length === 0) return;
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    try {
      const persistedState = await client.approveImport({
        previewId: preview.previewId,
        sourceId: preview.source.id,
        selectedRecordIds,
        consentScopeHash: preview.consentScopeHash,
      });
      if (generation !== operationGeneration.current) return;
      onMemoryStateChange(persistedState);
      setPreview(null);
      setCatalog(null);
      setConsent(false);
      setNotice("approved");
    } catch {
      if (generation !== operationGeneration.current) return;
      setError(true);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  async function handleForget() {
    if (!lineage || !forgetConsent) return;
    const generation = ++operationGeneration.current;
    setWorking(true);
    setError(false);
    try {
      const persistedState = await client.forgetSource(lineage.sourceId);
      if (generation !== operationGeneration.current) return;
      onMemoryStateChange(persistedState);
      setShowWhy(false);
      setShowForget(false);
      setForgetConsent(false);
      setSelectedSourceId("");
      setSelectedCandidateId("");
      setNotice("forgotten");
    } catch {
      if (generation !== operationGeneration.current) return;
      setError(true);
    } finally {
      if (generation === operationGeneration.current) setWorking(false);
    }
  }

  function toggleRecord(recordId: string) {
    setSelectedRecordIds((current) =>
      current.includes(recordId)
        ? current.filter((id) => id !== recordId)
        : [...current, recordId],
    );
    setConsent(false);
  }

  return (
    <section className="first-memory-flow" aria-labelledby="first-memory-title">
      <header className="flow-header">
        <div>
          <p className="flow-eyebrow">{t.eyebrow}</p>
          <h2 id="first-memory-title">{t.title}</h2>
          <p>{t.body}</p>
        </div>
        <span className="local-badge">{t.localBadge}</span>
      </header>

      <div className="flow-announcer" aria-live="polite" role="status">
        {working
          ? t.working
          : error
            ? t.genericError
            : notice === "approved"
              ? t.approvedNotice
              : notice === "forgotten"
                ? t.forgottenNotice
                : ""}
      </div>

      {!client.available && (
        <div className="runtime-boundary" data-testid="runtime-boundary">
          <span aria-hidden="true">◇</span>
          <div><h3>{t.unavailableTitle}</h3><p>{t.unavailableBody}</p></div>
        </div>
      )}

      {client.available && initializing && <p className="flow-loading">{t.booting}</p>}

      {client.available && !initializing && memoryState.sourceCount === 0 && !preview && (
        <div className="source-step">
          <div className="step-heading"><span>{t.stepOne}</span><p>{t.chooseSource}</p></div>
          <div className="source-grid">
            <div className="source-options">
              {sources.map((source) => (
                <label className={`source-option ${selectedSourceId === source.id ? "selected" : ""}`} key={source.id}>
                   <input
                      checked={selectedSourceId === source.id}
                      disabled={working}
                      name="memory-source"
                      onChange={() => void handleFixtureSelection(source.id)}
                    type="radio"
                    value={source.id}
                  />
                  <span className="source-copy">
                    <span><strong>{source.displayName}</strong><small>{t.fixtureBadge}</small></span>
                    <span className="source-description">{t.fixtureDescription}</span>
                    <code>{source.locator}</code>
                    <em>{t.adapter}: {source.adapterId} v{source.adapterVersion}</em>
                  </span>
                </label>
              ))}
              <article className="work-source-card">
                <div className="source-copy">
                  <span><strong>{t.workTitle}</strong><small>{t.experimental}</small></span>
                  <p>{t.workDescription}</p>
                  <p className="browse-disclosure">{t.browseDisclosure}</p>
                  <button className="secondary-button" disabled={working} onClick={handleBrowse} type="button">
                    {catalog ? t.browseAgain : t.browse}
                  </button>
                </div>
              </article>
            </div>

            <aside className="scope-card">
              <h3>{t.scopeTitle}</h3>
              <ul>{t.scopeItems.map((item) => <li key={item}>{item}</li>)}</ul>
              <strong>{t.nothingStored}</strong>
            </aside>
          </div>

          {selectedSource && (
            <button className="primary-button" disabled={working} onClick={handleFixturePreview} type="button">
              {t.previewFixture}
            </button>
          )}

          {catalog && (
            <section className="thread-catalog" aria-labelledby="thread-catalog-title">
              <h3 id="thread-catalog-title">{t.catalogTitle}</h3>
              {catalog.candidates.length === 0 ? (
                <p>{t.noCandidates}</p>
              ) : (
                <fieldset>
                  <legend className="sr-only">{t.catalogTitle}</legend>
                  {catalog.candidates.map((candidate) => (
                    <label className={`thread-candidate ${selectedCandidateId === candidate.candidateId ? "selected" : ""}`} key={candidate.candidateId}>
                      <input
                        checked={selectedCandidateId === candidate.candidateId}
                        disabled={working}
                        name="codex-thread-candidate"
                        onChange={() => {
                          setSelectedCandidateId(candidate.candidateId);
                          setConsent(false);
                        }}
                        type="radio"
                      />
                      <span><strong>{candidate.displayName}</strong><small>{candidate.sourceKind || t.candidateKind} · {formatTimestamp(candidate.updatedAt, locale)}</small></span>
                    </label>
                  ))}
                </fieldset>
              )}
              <div className="flow-actions">
                <button className="primary-button" disabled={!selectedCandidateId || working} onClick={handleThreadPreview} type="button">
                  {t.candidatePreview}
                </button>
                <button className="secondary-button" disabled={working} onClick={handleCancelBrowse} type="button">
                  {t.cancelBrowse}
                </button>
              </div>
            </section>
          )}
        </div>
      )}

      {client.available && !initializing && preview && (
        <div className="preview-step">
          <div className="step-heading"><span>{t.stepTwo}</span><h3>{previewIsThread ? t.workPreviewTitle : t.fixturePreviewTitle}</h3></div>
          <div className="preview-summary">
            <strong>{previewIsThread ? t.workRecords(preview.recordCount) : t.fixtureRecords(preview.recordCount)}</strong>
            <span>{t.timeRange}: {formatTimestamp(preview.timeRange.start, locale)}</span>
          </div>
          <div className="preview-records">
            {preview.records.map((record) => (
              <label className="preview-record" key={record.id}>
                <input checked={selectedRecordIds.includes(record.id)} onChange={() => toggleRecord(record.id)} type="checkbox" />
                <span>
                  <span className="record-meta"><strong>{t.completion}</strong><time dateTime={record.sourceTimestamp}>{formatTimestamp(record.sourceTimestamp, locale)}</time></span>
                  {previewIsThread ? <span className="redacted-content">{t.redacted}</span> : <q>{record.textPreview}</q>}
                  <small>{t.category}: {record.kind} · {t.characterCount}: {record.characterCount} · {t.hash}: {shortHash(record.contentHash)} · {t.selectRecord}</small>
                </span>
              </label>
            ))}
          </div>
          <label className="consent-check">
            <input checked={consent} disabled={selectedRecordIds.length === 0} onChange={(event) => setConsent(event.target.checked)} type="checkbox" />
            <span>{previewIsThread ? t.workConsent : t.fixtureConsent}</span>
          </label>
          <dl className="consent-scope-details">
            <div><dt>{t.adapter}</dt><dd><code>{preview.consentScope.adapterId} v{preview.consentScope.adapterVersion}</code></dd></div>
            <div><dt>{t.category}</dt><dd><code>{preview.consentScope.dataCategories.join(", ")}</code></dd></div>
            <div><dt>{t.purpose}</dt><dd><code>{preview.consentScope.purposes.join(", ")}</code></dd></div>
            <div><dt>{t.scopeRevision}</dt><dd>v{preview.consentScope.schemaVersion} · r{preview.consentScope.revision} · {t.readOnly}</dd></div>
          </dl>
          <div className="consent-hash">{t.hash}: <code>{preview.consentScopeHash}</code></div>
          <div className="flow-actions">
            <button className="primary-button" disabled={!consent || selectedRecordIds.length === 0 || working} onClick={handleApprove} type="button">
              {t.approveButton(selectedRecordIds.length)}
            </button>
            <button className="secondary-button" disabled={working} onClick={handleCancelPreview} type="button">{t.cancelPreview}</button>
          </div>
        </div>
      )}

      {client.available && !initializing && memoryState.sourceCount > 0 && mark && lineage && (
        <div className="stored-step">
          <div className="stored-heading">
            <div className="persisted-star" aria-hidden="true">✦</div>
            <div><span>{t.stepThree}</span><h3>{storedIsThread ? t.workStoredTitle : t.fixtureStoredTitle}</h3><p>{storedIsThread ? t.workStoredBody : t.fixtureStoredBody}</p></div>
          </div>
          <dl className="memory-metrics">
            <div><dt>{t.sourceMetric}</dt><dd>{memoryState.sourceCount}</dd></div>
            <div><dt>{t.eventMetric}</dt><dd>{memoryState.eventCount}</dd></div>
            <div><dt>{t.signalMetric}</dt><dd>{memoryState.signalCount}</dd></div>
            <div><dt>{t.markMetric}</dt><dd>{memoryState.marks.length}</dd></div>
          </dl>
          <button aria-expanded={showWhy} className="lineage-toggle" onClick={() => setShowWhy((value) => !value)} type="button">
            {showWhy ? t.whyHide : t.whyShow}
          </button>
          {showWhy && (
            <div className="lineage-inspector">
              <h4>{t.lineageTitle}</h4>
              <ol>
                <li><span>01</span><div><strong>{t.sourceNode}</strong><p>{lineage.sourceLabel}</p><code>{lineage.adapterId} v{lineage.adapterVersion}{storedIsThread && lineage.consentScopeHash ? ` · scope r${lineage.consentRevision} ${shortHash(lineage.consentScopeHash)}` : ""}</code></div></li>
                <li><span>02</span><div><strong>{t.eventNode}</strong><p>{storedIsThread ? t.redactedLineage : lineage.memoryText}</p><code>{t.schema} v{lineage.memoryEventSchemaVersion} · {storedIsThread ? `${t.hash} ${shortHash(lineage.contentHash)}` : lineage.memoryEventId}</code></div></li>
                <li><span>03</span><div><strong>{t.signalNode}</strong><p>{mark.signalType}</p><code>{t.derivation} v{mark.derivationVersion} · {t.confidence} {Math.round(mark.confidence * 100)}%</code></div></li>
                <li><span>04</span><div><strong>{t.effectNode}</strong><p>{mark.style}</p><code>{mark.id}</code></div></li>
              </ol>
            </div>
          )}
          {!showForget ? (
            <button className="danger-text-button" onClick={() => setShowForget(true)} type="button">{t.forgetStart}</button>
          ) : (
            <div className="forget-card">
              <h4>{t.forgetTitle}</h4><p>{storedIsThread ? t.workForgetBody : t.fixtureForgetBody}</p>
              <label><input checked={forgetConsent} onChange={(event) => setForgetConsent(event.target.checked)} type="checkbox" /><span>{t.forgetConsent}</span></label>
              <div className="flow-actions">
                <button className="danger-button" disabled={!forgetConsent || working} onClick={handleForget} type="button">{t.forgetConfirm}</button>
                <button className="secondary-button" disabled={working} onClick={() => { setShowForget(false); setForgetConsent(false); }} type="button">{t.keepMemory}</button>
              </div>
            </div>
          )}
        </div>
      )}
    </section>
  );
}

export default FirstMemoryFlow;
