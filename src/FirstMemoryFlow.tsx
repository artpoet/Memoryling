import { useEffect, useMemo, useState } from "react";
import "./FirstMemoryFlow.css";
import type {
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
}

const flowCopy = {
  en: {
    eyebrow: "FIRST REAL MEMORY · FIXTURE-ONLY PILOT",
    title: "From approved source to explainable mark",
    body:
      "This complete local pipeline reads one bundled synthetic Codex record. It never scans your Codex files or contacts a network.",
    localBadge: "LOCAL SQLITE · NO NETWORK",
    booting: "Opening the local memory store…",
    unavailableTitle: "Desktop runtime required",
    unavailableBody:
      "Memory access remains off in the browser preview. Open the Tauri desktop app to use the read-only adapter and local SQLite store.",
    stepOne: "1 · Select source",
    chooseSource: "Choose the only source this pilot is allowed to read.",
    fixtureBadge: "SYNTHETIC FIXTURE",
    adapter: "Adapter",
    scopeTitle: "Exact access scope",
    scopeItems: [
      "Reads one bundled synthetic JSON fixture only",
      "Cannot scan arbitrary paths or your Codex tool-home",
      "Cannot write to the source and makes no network request",
      "Preview stays in memory until you explicitly approve it",
    ],
    nothingStored: "Nothing has been stored yet.",
    previewButton: "Preview selected source",
    stepTwo: "2 · Review preview",
    previewTitle: "Review before anything is stored",
    recordCount: (count: number) => `${count} synthetic record${count === 1 ? "" : "s"}`,
    timeRange: "Time range",
    completion: "Completion",
    hash: "SHA-256",
    selectRecord: "Include this record",
    consent:
      "I approve storing the selected synthetic record in Memoryling's local SQLite database and deriving one explainable mark.",
    approveButton: (count: number) => `Approve & store ${count} memory locally`,
    cancelPreview: "Cancel preview",
    stepThree: "3 · Stored and derived",
    storedTitle: "One approved memory left a completion star",
    storedBody:
      "The mark below was returned from persisted Rust state. It is not optimistic UI or browser storage.",
    sourceMetric: "Approved sources",
    eventMetric: "Memory events",
    signalMetric: "Derived signals",
    markMetric: "World effects",
    whyShow: "Why did this happen?",
    whyHide: "Hide lineage",
    lineageTitle: "Machine-readable lineage",
    sourceNode: "Approved source record",
    eventNode: "Normalized memory event",
    signalNode: "Deterministic signal",
    effectNode: "Creature world effect",
    schema: "schema",
    derivation: "derivation",
    confidence: "confidence",
    forgetStart: "Forget this source",
    forgetTitle: "Complete forgetting",
    forgetBody:
      "This removes 1 local source, its normalized event, its signal, and the completion star in one transaction. The bundled source fixture is not modified.",
    forgetConsent: "I understand that the local imported memory and its mark will be removed.",
    forgetConfirm: "Forget source and remove mark",
    keepMemory: "Keep memory",
    genericError:
      "The local operation did not complete. No UI state was changed; try again from the current step.",
    approvedNotice: "Approved locally. The persisted mark is now active.",
    forgottenNotice: "Forgotten completely. Memory access is off again.",
    working: "Working locally…",
  },
  "zh-TW": {
    eyebrow: "第一筆真實記憶流程 · 僅限合成 FIXTURE 試行",
    title: "從核准來源，到可解釋的印記",
    body:
      "這條完整本機流程只讀取一筆隨 App 打包的合成 Codex 紀錄；不會掃描你的 Codex 檔案，也不會連上網路。",
    localBadge: "本機 SQLITE · 無網路傳輸",
    booting: "正在開啟本機記憶資料庫…",
    unavailableTitle: "需要桌面版執行環境",
    unavailableBody:
      "瀏覽器預覽仍維持「記憶存取關閉」。請開啟 Tauri 桌面 App，才能使用唯讀 adapter 與本機 SQLite。",
    stepOne: "1 · 選擇來源",
    chooseSource: "請明確選擇這次試行唯一獲准讀取的來源。",
    fixtureBadge: "合成 FIXTURE",
    adapter: "Adapter",
    scopeTitle: "精確存取範圍",
    scopeItems: [
      "只讀取一個隨 App 打包的合成 JSON fixture",
      "不能掃描任意路徑，也不能讀取你的 Codex tool-home",
      "不能修改來源，且不會送出任何網路請求",
      "預覽只暫存在記憶體，直到你明確核准",
    ],
    nothingStored: "目前尚未儲存任何內容。",
    previewButton: "預覽所選來源",
    stepTwo: "2 · 檢查預覽",
    previewTitle: "儲存前先完整確認",
    recordCount: (count: number) => `${count} 筆合成紀錄`,
    timeRange: "時間範圍",
    completion: "完成事件",
    hash: "SHA-256",
    selectRecord: "納入這筆紀錄",
    consent:
      "我核准將所選合成紀錄存入 Memoryling 的本機 SQLite，並衍生一個可解釋的印記。",
    approveButton: (count: number) => `核准並在本機儲存 ${count} 筆記憶`,
    cancelPreview: "取消預覽",
    stepThree: "3 · 已儲存並衍生",
    storedTitle: "一筆核准記憶留下了完成之星",
    storedBody:
      "下方印記來自 Rust 回傳的已持久化狀態，不是預先樂觀顯示，也不是瀏覽器儲存。",
    sourceMetric: "核准來源",
    eventMetric: "記憶事件",
    signalMetric: "衍生訊號",
    markMetric: "世界影響",
    whyShow: "為什麼會發生？",
    whyHide: "收起來源鏈",
    lineageTitle: "機器可讀來源鏈",
    sourceNode: "已核准來源紀錄",
    eventNode: "正規化記憶事件",
    signalNode: "確定性衍生訊號",
    effectNode: "記憶獸世界影響",
    schema: "schema",
    derivation: "derivation",
    confidence: "信心值",
    forgetStart: "遺忘這個來源",
    forgetTitle: "完整遺忘",
    forgetBody:
      "這會在同一個 transaction 中移除 1 個本機來源、正規化事件、衍生訊號與完成之星；打包的來源 fixture 不會被修改。",
    forgetConsent: "我了解本機匯入的記憶與其印記都會被移除。",
    forgetConfirm: "遺忘來源並移除印記",
    keepMemory: "保留記憶",
    genericError: "本機操作未完成，畫面狀態沒有變更；請從目前步驟再試一次。",
    approvedNotice: "已在本機核准；持久化印記現在已生效。",
    forgottenNotice: "已完整遺忘；記憶存取再次關閉。",
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
}: FirstMemoryFlowProps) {
  const t = flowCopy[locale];
  const [sources, setSources] = useState<SourceOption[]>([]);
  const [selectedSourceId, setSelectedSourceId] = useState("");
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

  useEffect(() => {
    let active = true;
    if (!client.available) return () => undefined;

    setInitializing(true);
    Promise.all([client.listSources(), client.getState()])
      .then(([availableSources, persistedState]) => {
        if (!active) return;
        setSources(availableSources);
        onMemoryStateChange(persistedState);
        setError(false);
      })
      .catch(() => {
        if (active) setError(true);
      })
      .finally(() => {
        if (active) setInitializing(false);
      });

    return () => {
      active = false;
    };
  }, [client, onMemoryStateChange]);

  const selectedSource = useMemo(
    () => sources.find((source) => source.id === selectedSourceId),
    [selectedSourceId, sources],
  );
  const mark = memoryState.marks[0];
  const lineage = mark?.lineage[0];

  async function handlePreview() {
    if (!selectedSource) return;
    setWorking(true);
    setError(false);
    setNotice(null);
    try {
      const nextPreview = await client.previewSource(selectedSource.id);
      setPreview(nextPreview);
      setSelectedRecordIds(nextPreview.records.map((record) => record.id));
      setConsent(false);
    } catch {
      setError(true);
      setPreview(null);
      setSelectedRecordIds([]);
      setConsent(false);
    } finally {
      setWorking(false);
    }
  }

  async function handleCancelPreview() {
    const previewId = preview?.previewId;
    setWorking(true);
    setError(false);
    try {
      if (previewId) await client.cancelPreview(previewId);
      setPreview(null);
      setSelectedRecordIds([]);
      setConsent(false);
    } catch {
      setError(true);
    } finally {
      setWorking(false);
    }
  }

  async function handleApprove() {
    if (!preview || !consent || selectedRecordIds.length === 0) return;
    setWorking(true);
    setError(false);
    try {
      const persistedState = await client.approveImport({
        previewId: preview.previewId,
        sourceId: preview.source.id,
        selectedRecordIds,
      });
      onMemoryStateChange(persistedState);
      setPreview(null);
      setConsent(false);
      setNotice("approved");
    } catch {
      setError(true);
    } finally {
      setWorking(false);
    }
  }

  async function handleForget() {
    if (!lineage || !forgetConsent) return;
    setWorking(true);
    setError(false);
    try {
      const persistedState = await client.forgetSource(lineage.sourceId);
      onMemoryStateChange(persistedState);
      setShowWhy(false);
      setShowForget(false);
      setForgetConsent(false);
      setSelectedSourceId("");
      setNotice("forgotten");
    } catch {
      setError(true);
    } finally {
      setWorking(false);
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
          <div>
            <h3>{t.unavailableTitle}</h3>
            <p>{t.unavailableBody}</p>
          </div>
        </div>
      )}

      {client.available && initializing && <p className="flow-loading">{t.booting}</p>}

      {client.available && !initializing && memoryState.sourceCount === 0 && !preview && (
        <div className="source-step">
          <div className="step-heading">
            <span>{t.stepOne}</span>
            <p>{t.chooseSource}</p>
          </div>
          <div className="source-grid">
            <fieldset className="source-options">
              <legend className="sr-only">{t.chooseSource}</legend>
              {sources.map((source) => (
                <label
                  className={`source-option ${
                    selectedSourceId === source.id ? "selected" : ""
                  }`}
                  key={source.id}
                >
                  <input
                    checked={selectedSourceId === source.id}
                    name="memory-source"
                    onChange={() => {
                      setSelectedSourceId(source.id);
                      setNotice(null);
                    }}
                    type="radio"
                    value={source.id}
                  />
                  <span className="source-copy">
                    <span>
                      <strong>{source.displayName}</strong>
                      <small>{t.fixtureBadge}</small>
                    </span>
                    <code>{source.locator}</code>
                    <em>
                      {t.adapter}: {source.adapterId} v{source.adapterVersion}
                    </em>
                  </span>
                </label>
              ))}
            </fieldset>

            <aside className="scope-card">
              <h3>{t.scopeTitle}</h3>
              <ul>
                {t.scopeItems.map((item) => (
                  <li key={item}>{item}</li>
                ))}
              </ul>
              <strong>{t.nothingStored}</strong>
            </aside>
          </div>
          <button
            className="primary-button"
            disabled={!selectedSource || working}
            onClick={handlePreview}
            type="button"
          >
            {t.previewButton}
          </button>
        </div>
      )}

      {client.available && !initializing && preview && (
        <div className="preview-step">
          <div className="step-heading">
            <span>{t.stepTwo}</span>
            <h3>{t.previewTitle}</h3>
          </div>
          <div className="preview-summary">
            <strong>{t.recordCount(preview.recordCount)}</strong>
            <span>
              {t.timeRange}: {formatTimestamp(preview.timeRange.start, locale)}
            </span>
          </div>
          <div className="preview-records">
            {preview.records.map((record) => (
              <label className="preview-record" key={record.id}>
                <input
                  checked={selectedRecordIds.includes(record.id)}
                  onChange={() => toggleRecord(record.id)}
                  type="checkbox"
                />
                <span>
                  <span className="record-meta">
                    <strong>{t.completion}</strong>
                    <time dateTime={record.sourceTimestamp}>
                      {formatTimestamp(record.sourceTimestamp, locale)}
                    </time>
                  </span>
                  <q>{record.textPreview}</q>
                  <small>
                    {t.hash}: {shortHash(record.contentHash)} · {t.selectRecord}
                  </small>
                </span>
              </label>
            ))}
          </div>
          <label className="consent-check">
            <input
              checked={consent}
              disabled={selectedRecordIds.length === 0}
              onChange={(event) => setConsent(event.target.checked)}
              type="checkbox"
            />
            <span>{t.consent}</span>
          </label>
          <div className="flow-actions">
            <button
              className="primary-button"
              disabled={!consent || selectedRecordIds.length === 0 || working}
              onClick={handleApprove}
              type="button"
            >
              {t.approveButton(selectedRecordIds.length)}
            </button>
            <button
              className="secondary-button"
              disabled={working}
              onClick={handleCancelPreview}
              type="button"
            >
              {t.cancelPreview}
            </button>
          </div>
        </div>
      )}

      {client.available && !initializing && memoryState.sourceCount > 0 && mark && lineage && (
        <div className="stored-step">
          <div className="stored-heading">
            <div className="persisted-star" aria-hidden="true">✦</div>
            <div>
              <span>{t.stepThree}</span>
              <h3>{t.storedTitle}</h3>
              <p>{t.storedBody}</p>
            </div>
          </div>
          <dl className="memory-metrics">
            <div><dt>{t.sourceMetric}</dt><dd>{memoryState.sourceCount}</dd></div>
            <div><dt>{t.eventMetric}</dt><dd>{memoryState.eventCount}</dd></div>
            <div><dt>{t.signalMetric}</dt><dd>{memoryState.signalCount}</dd></div>
            <div><dt>{t.markMetric}</dt><dd>{memoryState.marks.length}</dd></div>
          </dl>

          <button
            aria-expanded={showWhy}
            className="lineage-toggle"
            onClick={() => setShowWhy((value) => !value)}
            type="button"
          >
            {showWhy ? t.whyHide : t.whyShow}
          </button>

          {showWhy && (
            <div className="lineage-inspector">
              <h4>{t.lineageTitle}</h4>
              <ol>
                <li>
                  <span>01</span>
                  <div>
                    <strong>{t.sourceNode}</strong>
                    <p>{lineage.sourceLabel}</p>
                    <code>{lineage.adapterId} v{lineage.adapterVersion} · {lineage.sourceRecordId}</code>
                  </div>
                </li>
                <li>
                  <span>02</span>
                  <div>
                    <strong>{t.eventNode}</strong>
                    <p>{lineage.memoryText}</p>
                    <code>{t.schema} v{lineage.memoryEventSchemaVersion} · {lineage.memoryEventId}</code>
                  </div>
                </li>
                <li>
                  <span>03</span>
                  <div>
                    <strong>{t.signalNode}</strong>
                    <p>{mark.signalType}</p>
                    <code>{t.derivation} v{mark.derivationVersion} · {t.confidence} {Math.round(mark.confidence * 100)}%</code>
                  </div>
                </li>
                <li>
                  <span>04</span>
                  <div>
                    <strong>{t.effectNode}</strong>
                    <p>{mark.style}</p>
                    <code>{mark.id}</code>
                  </div>
                </li>
              </ol>
            </div>
          )}

          {!showForget ? (
            <button
              className="danger-text-button"
              onClick={() => setShowForget(true)}
              type="button"
            >
              {t.forgetStart}
            </button>
          ) : (
            <div className="forget-card">
              <h4>{t.forgetTitle}</h4>
              <p>{t.forgetBody}</p>
              <label>
                <input
                  checked={forgetConsent}
                  onChange={(event) => setForgetConsent(event.target.checked)}
                  type="checkbox"
                />
                <span>{t.forgetConsent}</span>
              </label>
              <div className="flow-actions">
                <button
                  className="danger-button"
                  disabled={!forgetConsent || working}
                  onClick={handleForget}
                  type="button"
                >
                  {t.forgetConfirm}
                </button>
                <button
                  className="secondary-button"
                  disabled={working}
                  onClick={() => {
                    setShowForget(false);
                    setForgetConsent(false);
                  }}
                  type="button"
                >
                  {t.keepMemory}
                </button>
              </div>
            </div>
          )}
        </div>
      )}
    </section>
  );
}

export default FirstMemoryFlow;
