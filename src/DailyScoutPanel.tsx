import { useEffect, useState } from "react";
import type { Locale } from "./locale";
import {
  emptyDailyScoutState,
  nativeDailyScoutClient,
  type DailyScoutClient,
  type DailyScoutState,
} from "./dailyScoutClient";
import "./DailyScoutPanel.css";

const copy = {
  en: {
    eyebrow: "OPTIONAL ONLINE FEATURE",
    title: "Daily Memory Scout",
    intro:
      "More than a pet: when you opt in, Memoryling can bring back one useful, source-linked insight for your recent work each day.",
    ordinaryPet: "The ordinary local pet does not need an API.",
    browserOnly:
      "API setup is available only in the Windows desktop app. This browser preview makes no request.",
    keyTitle: "Connect your OpenAI API",
    keyHelp:
      "Your key is saved in Windows Credential Manager and is never shown again. API usage may cost money.",
    keyPlaceholder: "Paste an OpenAI API key",
    saveKey: "Save key securely",
    replaceKey: "Replace key",
    connected: "OpenAI key connected",
    getKey: "Get an OpenAI API key ↗",
    quickstart: "OpenAI API guide ↗",
    test: "Test connection",
    testing: "Testing…",
    tested: "Connection works. No Web Search was used.",
    deleteKey: "Delete key and turn off",
    contextTitle: "What may be sent",
    contextEmpty:
      "Approve one supported Codex work record first. Synthetic fixtures are never sent.",
    domains: "Work",
    tools: "Public tools",
    goals: "Goals",
    constraints: "Constraints",
    evidence: "Evidence window",
    enableTitle: "Enable one search per day",
    delivery: "Check after",
    deliveryHelp: "Only while Memoryling is running; missed days are not replayed.",
    consent:
      "I agree to send only the summary above to OpenAI for one daily search and short message. I understand my API account pays the cost; store:false is used, but ordinary API abuse-monitoring retention may still be up to 30 days.",
    enable: "Enable Daily Scout",
    disable: "Turn off",
    scheduled: "Ready to check once today after the chosen time.",
    running: "Memoryling is looking for today's useful thing…",
    complete: "Today's check is complete.",
    failed: "Today's search did not succeed. It will not retry automatically today.",
    needsKey: "The saved key is missing. Daily search is paused.",
    needsMemory: "The approved work context changed. Enable again after reviewing it.",
    ready: "Memoryling brought something back.",
    quiet: "Practical tip · no major update claimed",
    practical: "Useful current update",
    sources: "Sources",
    why: "Why this fits",
    searched: "Searched",
    markRead: "Got it",
    clearHistory: "Clear local insight",
    reset: "Turn off, delete key, and clear local insight",
    privacy:
      "Never sent: prompts, final-answer text, paths, repository URLs, thread IDs, credentials, or arbitrary private phrases.",
    genericError: "That action did not finish. Check the setting and try again.",
    authError: "The API key was rejected. Replace it and check the API account.",
    quotaError: "The API account reported a rate, quota, or billing limit.",
    offlineError: "Memoryling could not reach OpenAI today.",
    invalidResponse: "No safe source-linked result passed validation today.",
  },
  "zh-TW": {
    eyebrow: "選配線上功能",
    title: "每日記憶情報",
    intro:
      "不只是寵物：你選擇開啟後，Memoryling 每天能替近期工作帶回一則實用且附來源的情報。",
    ordinaryPet: "普通的本機寵物不需要 API。",
    browserOnly:
      "API 設定只在 Windows 桌面 App 提供；這個瀏覽器預覽不會發出請求。",
    keyTitle: "連接你的 OpenAI API",
    keyHelp:
      "Key 會存入 Windows Credential Manager，之後不會再次顯示；API 使用可能產生費用。",
    keyPlaceholder: "貼上 OpenAI API key",
    saveKey: "安全儲存 key",
    replaceKey: "更換 key",
    connected: "OpenAI key 已連接",
    getKey: "申請 OpenAI API key ↗",
    quickstart: "OpenAI API 說明 ↗",
    test: "測試連線",
    testing: "測試中…",
    tested: "連線正常；這次沒有使用 Web Search。",
    deleteKey: "刪除 key 並關閉",
    contextTitle: "可能送出的內容",
    contextEmpty:
      "請先核准一筆支援的 Codex 工作紀錄；合成 fixture 永遠不會送出。",
    domains: "工作類型",
    tools: "公開工具",
    goals: "工作目標",
    constraints: "非敏感限制",
    evidence: "證據時間範圍",
    enableTitle: "開啟每天一次搜尋",
    delivery: "這個時間後檢查",
    deliveryHelp: "只在 Memoryling 運行時執行；錯過的日期不補跑。",
    consent:
      "我同意只把上方摘要送往 OpenAI，用於每天一次搜尋與短訊息。我理解費用由自己的 API 帳戶負擔；Memoryling 使用 store:false，但一般 API 的濫用監控資料仍可能保留最長 30 天。",
    enable: "開啟每日記憶情報",
    disable: "關閉功能",
    scheduled: "已排定在所選時間後，今天最多檢查一次。",
    running: "Memoryling 正在找今天可能有用的東西……",
    complete: "今天的檢查已完成。",
    failed: "今天沒有成功帶回情報，而且不會在今天自動重試。",
    needsKey: "已保存的 key 不見了；每日搜尋已暫停。",
    needsMemory: "核准的工作脈絡已改變；請重新檢視後啟用。",
    ready: "Memoryling 帶東西回來了。",
    quiet: "一般實用技巧 · 沒有宣稱重大更新",
    practical: "實用近期更新",
    sources: "資料來源",
    why: "為何適合你",
    searched: "搜尋時間",
    markRead: "知道了",
    clearHistory: "清除本機情報",
    reset: "關閉、刪除 key 並清除本機情報",
    privacy:
      "絕不送出：prompt、final answer 原文、路徑、私有 repo 網址、thread ID、憑證或任意私密句子。",
    genericError: "操作沒有完成，請檢查設定後再試一次。",
    authError: "API key 被拒絕，請更換 key 並檢查 API 帳戶。",
    quotaError: "API 帳戶遇到頻率、額度或帳務限制。",
    offlineError: "Memoryling 今天無法連上 OpenAI。",
    invalidResponse: "今天沒有任何附來源的安全結果通過驗證。",
  },
} as const;

interface DailyScoutPanelProps {
  client?: DailyScoutClient;
  locale: Locale;
  refreshRevision?: number;
}

function errorCopy(error: unknown, t: (typeof copy)[Locale]) {
  const code = typeof error === "string" ? error : "";
  if (code.includes("authentication")) return t.authError;
  if (code.includes("quota-or-rate-limit")) return t.quotaError;
  if (code.includes("offline") || code.includes("timeout")) return t.offlineError;
  if (code.includes("invalid-response")) return t.invalidResponse;
  return t.genericError;
}

export function DailyScoutPanel({
  client = nativeDailyScoutClient,
  locale,
  refreshRevision = 0,
}: DailyScoutPanelProps) {
  const t = copy[locale];
  const [state, setState] = useState<DailyScoutState>(emptyDailyScoutState);
  const [apiKey, setApiKey] = useState("");
  const [deliveryTime, setDeliveryTime] = useState("10:00");
  const [consent, setConsent] = useState(false);
  const [busy, setBusy] = useState<string | null>(null);
  const [notice, setNotice] = useState("");

  useEffect(() => {
    if (!client.available) return;
    let active = true;
    void client
      .getState()
      .then((next) => {
        if (!active) return;
        setState(next);
        setDeliveryTime(next.deliveryTime);
      })
      .catch(() => undefined);
    return () => {
      active = false;
    };
  }, [client, refreshRevision]);

  async function runAction(
    name: string,
    action: () => Promise<DailyScoutState>,
  ) {
    setBusy(name);
    setNotice("");
    try {
      const next = await action();
      setState(next);
      setDeliveryTime(next.deliveryTime);
    } catch (error) {
      setNotice(errorCopy(error, t));
    } finally {
      setBusy(null);
    }
  }

  async function saveKey() {
    const submitted = apiKey;
    setBusy("key");
    setNotice("");
    try {
      const next = await client.saveApiKey(submitted);
      setState(next);
    } catch (error) {
      setNotice(errorCopy(error, t));
    } finally {
      setApiKey("");
      setBusy(null);
    }
  }

  async function testKey() {
    setBusy("test");
    setNotice("");
    try {
      await client.testApiKey();
      setNotice(t.tested);
    } catch (error) {
      setNotice(errorCopy(error, t));
    } finally {
      setBusy(null);
    }
  }

  async function openLink(
    kind: "api-keys" | "quickstart" | "citation",
    url?: string,
  ) {
    setNotice("");
    try {
      await client.openLink(kind, url);
    } catch (error) {
      setNotice(errorCopy(error, t));
    }
  }

  const context = state.contextPreview;
  const insight = state.latestInsight;
  const statusText = {
    off: "",
    "needs-key": t.needsKey,
    "needs-memory": t.needsMemory,
    scheduled: t.scheduled,
    running: t.running,
    ready: t.ready,
    failed: t.failed,
    complete: t.complete,
  }[state.status];

  return (
    <section className="daily-scout" aria-labelledby="daily-scout-title">
      <header className="daily-scout-header">
        <div>
          <p>{t.eyebrow}</p>
          <h2 id="daily-scout-title">{t.title}</h2>
          <span>{t.intro}</span>
        </div>
        <strong data-enabled={state.enabled}>{state.enabled ? "ON" : "OFF"}</strong>
      </header>

      <p className="daily-scout-local-note">{t.ordinaryPet}</p>

      {!client.available ? (
        <div className="daily-scout-boundary">{t.browserOnly}</div>
      ) : (
        <div className="daily-scout-grid">
          <article className="daily-scout-card">
            <h3>{t.keyTitle}</h3>
            <p>{t.keyHelp}</p>
            {state.hasApiKey && <div className="daily-scout-connected">✓ {t.connected}</div>}
            <div className="daily-scout-key-row">
              <input
                aria-label={t.keyPlaceholder}
                autoComplete="off"
                onChange={(event) => setApiKey(event.target.value)}
                placeholder={t.keyPlaceholder}
                type="password"
                value={apiKey}
              />
              <button
                disabled={!apiKey.trim() || busy !== null}
                onClick={() => void saveKey()}
                type="button"
              >
                {state.hasApiKey ? t.replaceKey : t.saveKey}
              </button>
            </div>
            <div className="daily-scout-link-row">
              <a
                href="https://platform.openai.com/api-keys"
                onClick={(event) => {
                  event.preventDefault();
                  void openLink("api-keys");
                }}
                rel="noreferrer"
                target="_blank"
              >
                {t.getKey}
              </a>
              <a
                href="https://developers.openai.com/api/docs/quickstart"
                onClick={(event) => {
                  event.preventDefault();
                  void openLink("quickstart");
                }}
                rel="noreferrer"
                target="_blank"
              >
                {t.quickstart}
              </a>
            </div>
            {state.hasApiKey && (
              <div className="daily-scout-actions">
                <button disabled={busy !== null} onClick={() => void testKey()} type="button">
                  {busy === "test" ? t.testing : t.test}
                </button>
                <button
                  disabled={busy !== null}
                  onClick={() => void runAction("delete", () => client.deleteApiKey())}
                  type="button"
                >
                  {t.deleteKey}
                </button>
              </div>
            )}
          </article>

          <article className="daily-scout-card">
            <h3>{t.contextTitle}</h3>
            {context ? (
              <dl className="daily-context-list">
                <div><dt>{t.domains}</dt><dd>{context.workDomains.join(" · ")}</dd></div>
                <div><dt>{t.tools}</dt><dd>{context.publicToolsAndModels.join(" · ") || "—"}</dd></div>
                <div><dt>{t.goals}</dt><dd>{context.currentGoals.join(" · ")}</dd></div>
                {context.nonSensitiveConstraints.length > 0 && (
                  <div><dt>{t.constraints}</dt><dd>{context.nonSensitiveConstraints.join(" · ")}</dd></div>
                )}
                <div>
                  <dt>{t.evidence}</dt>
                  <dd>{context.evidenceWindow.startDate} → {context.evidenceWindow.endDate}</dd>
                </div>
              </dl>
            ) : (
              <p className="daily-context-empty">{t.contextEmpty}</p>
            )}
            <p className="daily-scout-privacy">{t.privacy}</p>
          </article>

          <article className="daily-scout-card daily-scout-enable-card">
            <h3>{t.enableTitle}</h3>
            <label className="daily-scout-time">
              <span>{t.delivery}</span>
              <input
                max="21:59"
                min="08:00"
                onChange={(event) => setDeliveryTime(event.target.value)}
                type="time"
                value={deliveryTime}
              />
            </label>
            <small>{t.deliveryHelp}</small>
            {!state.enabled && (
              <label className="daily-scout-consent">
                <input
                  checked={consent}
                  onChange={(event) => setConsent(event.target.checked)}
                  type="checkbox"
                />
                <span>{t.consent}</span>
              </label>
            )}
            <div className="daily-scout-actions">
              {state.enabled ? (
                <button
                  disabled={busy !== null}
                  onClick={() => void runAction("disable", () => client.disable())}
                  type="button"
                >
                  {t.disable}
                </button>
              ) : (
                <button
                  disabled={!state.canEnable || !consent || busy !== null}
                  onClick={() =>
                    void runAction("enable", () =>
                      client.configure({
                        locale,
                        deliveryTime,
                        consentAccepted: true,
                      }),
                    )
                  }
                  type="button"
                >
                  {t.enable}
                </button>
              )}
            </div>
            {statusText && <p className="daily-scout-status" role="status">{statusText}</p>}
          </article>
        </div>
      )}

      {client.available && insight && (
        <article className="daily-insight-card">
          <div className="daily-insight-heading">
            <span>{insight.strength === "quiet" ? t.quiet : t.practical}</span>
            <time dateTime={insight.searchedAt}>{insight.localDate}</time>
          </div>
          <blockquote>{insight.petMessage}</blockquote>
          <dl>
            <div><dt>{t.why}</dt><dd>{insight.relevanceReason}</dd></div>
            <div><dt>{t.searched}</dt><dd>{insight.searchedAt}</dd></div>
          </dl>
          <div className="daily-insight-sources">
            <strong>{t.sources}</strong>
            <ol>
              {insight.citations.map((citation) => (
                <li key={citation.url}>
                  <a
                    href={citation.url}
                    onClick={(event) => {
                      event.preventDefault();
                      void openLink("citation", citation.url);
                    }}
                    rel="noreferrer"
                    target="_blank"
                  >
                    {citation.title} ↗
                  </a>
                </li>
              ))}
            </ol>
          </div>
          <div className="daily-scout-actions">
            {!insight.read && (
              <button disabled={busy !== null} onClick={() => void runAction("read", () => client.markRead())} type="button">
                {t.markRead}
              </button>
            )}
            <button disabled={busy !== null} onClick={() => void runAction("clear", () => client.clearHistory())} type="button">
              {t.clearHistory}
            </button>
          </div>
        </article>
      )}

      {client.available && (state.hasApiKey || state.enabled || insight) && (
        <button
          className="daily-scout-reset"
          disabled={busy !== null}
          onClick={() => void runAction("reset", () => client.reset())}
          type="button"
        >
          {t.reset}
        </button>
      )}

      <p className="daily-scout-notice" aria-live="polite" role="status">{notice}</p>
    </section>
  );
}

export default DailyScoutPanel;
