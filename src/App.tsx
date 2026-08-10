import { useEffect, useMemo, useState } from "react";
import "./App.css";

type Locale = "en" | "zh-TW";

const copy = {
  en: {
    prototype: "Concept shell · memory access is off",
    tagline: "Your agent memories, alive.",
    intro:
      "A small desktop life that grows from the work, ideas, and promises your AI agents remember.",
    creatureName: "Your first Memoryling",
    creatureState: "Listening for a beginning",
    creatureLines: [
      "I woke up between the things you finished and the things still glowing.",
      "When memory access arrives, every mark on me will have a reason.",
      "I can be curious without becoming noisy. You decide the boundaries.",
    ],
    tapHint: "Tap the Memoryling to hear another thought",
    eventLabel: "Continuing event",
    eventTitle: "The Unclosed Gate",
    eventBody:
      "A promise has appeared at the edge of the habitat. It will remain quiet until the right moment.",
    eventAction: "Keep it for tomorrow",
    memoryLabel: "Memory signals",
    memoryItems: [
      ["A world shipped", "completion · strong"],
      ["An old idea returned", "recurrence · medium"],
      ["A private boundary held", "value · strong"],
    ],
    whyLabel: "Why did this happen?",
    whyBody:
      "Future changes will point back to their source memories, confidence, and derived effects. Nothing is connected in this prototype.",
    hideWhy: "Hide explanation",
    showWhy: "Show explanation",
    initiativeLabel: "Bounded initiative",
    quietHours: "Quiet hours",
    quietValue: "22:00–09:00",
    nudgeBudget: "Daily nudge budget",
    nudgeValue: "1 of 2 available",
    principle:
      "Memoryling may decide when to speak, but it never decides your limits.",
    privacyLabel: "Local-first promise",
    privacyBody:
      "The future connector will be read-only. Raw agent memories, credentials, and private files must never be committed or silently uploaded.",
    roadmap: "Product vision included in the repository",
  },
  "zh-TW": {
    prototype: "概念原型 · 尚未讀取任何記憶",
    tagline: "讓你的 Agent 記憶，長成一個生命。",
    intro:
      "一個住在桌面的微小生命，從 AI Agent 記得的工作、點子與承諾中逐漸成長。",
    creatureName: "你的第一隻記憶獸",
    creatureState: "正在等待故事開始",
    creatureLines: [
      "我在你完成的事，和那些還亮著的事之間醒來。",
      "未來我身上的每個變化，都必須有記憶可以解釋。",
      "我可以主動好奇，但不會變得吵鬧；界線由你決定。",
    ],
    tapHint: "點一下記憶獸，聽聽牠的另一個念頭",
    eventLabel: "連續事件",
    eventTitle: "沒有關上的門",
    eventBody:
      "有一個承諾出現在棲地邊緣。在適合的時機之前，牠會安靜地留在那裡。",
    eventAction: "留到明天",
    memoryLabel: "記憶訊號",
    memoryItems: [
      ["一個世界正式完成", "完成 · 強"],
      ["舊點子重新出現", "反覆 · 中"],
      ["守住了一條私密界線", "價值 · 強"],
    ],
    whyLabel: "為什麼會發生？",
    whyBody:
      "未來每個變化都會指出來源記憶、信心值與衍生影響。目前只是原型，尚未連接任何真實資料。",
    hideWhy: "收起說明",
    showWhy: "查看說明",
    initiativeLabel: "有限主動性",
    quietHours: "安靜時段",
    quietValue: "22:00–09:00",
    nudgeBudget: "每日提醒額度",
    nudgeValue: "剩餘 1／2 次",
    principle: "記憶獸可以決定何時開口，但永遠不能替你決定界線。",
    privacyLabel: "Local-first 承諾",
    privacyBody:
      "未來的 connector 必須唯讀；原始 Agent 記憶、憑證與私密檔案不得被 commit 或在未告知下上傳。",
    roadmap: "產品願景已收錄於 Repo 文件",
  },
} as const;

function getInitialLocale(): Locale {
  const requested = new URLSearchParams(window.location.search).get("lang");
  if (requested === "en" || requested === "zh-TW") return requested;
  const saved = window.localStorage.getItem("memoryling:locale");
  if (saved === "en" || saved === "zh-TW") return saved;
  return navigator.language.toLowerCase().startsWith("zh") ? "zh-TW" : "en";
}

function App() {
  const [locale, setLocale] = useState<Locale>(getInitialLocale);
  const [lineIndex, setLineIndex] = useState(0);
  const [showWhy, setShowWhy] = useState(false);
  const [eventSnoozed, setEventSnoozed] = useState(false);
  const t = copy[locale];

  useEffect(() => {
    document.documentElement.lang = locale;
    window.localStorage.setItem("memoryling:locale", locale);
  }, [locale]);

  const creatureLine = useMemo(
    () => t.creatureLines[lineIndex % t.creatureLines.length],
    [lineIndex, t.creatureLines],
  );

  return (
    <main className="app-shell">
      <header className="topbar">
        <a className="brand" href="#top" aria-label="Memoryling home">
          <span className="brand-mark" aria-hidden="true">
            M
          </span>
          <span>Memoryling</span>
        </a>
        <div className="topbar-actions">
          <span className="prototype-pill">{t.prototype}</span>
          <div className="locale-toggle" aria-label="Language">
            <button
              className={locale === "en" ? "active" : ""}
              onClick={() => setLocale("en")}
              type="button"
            >
              EN
            </button>
            <button
              className={locale === "zh-TW" ? "active" : ""}
              onClick={() => setLocale("zh-TW")}
              type="button"
            >
              繁中
            </button>
          </div>
        </div>
      </header>

      <section className="hero" id="top">
        <div className="hero-copy">
          <p className="eyebrow">LOCAL-FIRST · MEMORY-DRIVEN · OPEN SOURCE</p>
          <h1>{t.tagline}</h1>
          <p className="intro">{t.intro}</p>
        </div>

        <div className="habitat-card">
          <div className="habitat-glow habitat-glow-one" />
          <div className="habitat-glow habitat-glow-two" />
          <button
            className="creature-button"
            onClick={() => setLineIndex((value) => value + 1)}
            type="button"
            aria-label={t.tapHint}
          >
            <span className="orbit orbit-one" aria-hidden="true" />
            <span className="orbit orbit-two" aria-hidden="true" />
            <span className="memoryling" aria-hidden="true">
              <span className="ear ear-left" />
              <span className="ear ear-right" />
              <span className="face">
                <span className="eye eye-left" />
                <span className="eye eye-right" />
                <span className="cheek cheek-left" />
                <span className="cheek cheek-right" />
                <span className="mouth" />
              </span>
              <span className="memory-mark memory-mark-one" />
              <span className="memory-mark memory-mark-two" />
            </span>
          </button>

          <div className="creature-caption">
            <p>{t.creatureName}</p>
            <span>{t.creatureState}</span>
          </div>

          <div className="speech-card" aria-live="polite">
            <span className="speech-spark">✦</span>
            <p>{creatureLine}</p>
          </div>
          <p className="tap-hint">{t.tapHint}</p>
        </div>
      </section>

      <section className="dashboard" aria-label="Memoryling concept dashboard">
        <article className="panel event-panel">
          <div className="panel-heading">
            <span className="panel-icon amber">◌</span>
            <p>{t.eventLabel}</p>
            <span className="live-dot">LIVE</span>
          </div>
          <h2>{t.eventTitle}</h2>
          <p className="panel-copy">{t.eventBody}</p>
          <button
            className="soft-button"
            type="button"
            onClick={() => setEventSnoozed((value) => !value)}
          >
            {eventSnoozed ? "✓ " : ""}
            {t.eventAction}
          </button>
        </article>

        <article className="panel memory-panel">
          <div className="panel-heading">
            <span className="panel-icon violet">✦</span>
            <p>{t.memoryLabel}</p>
          </div>
          <ul className="signal-list">
            {t.memoryItems.map(([label, meta], index) => (
              <li key={label}>
                <span className={`signal-orb signal-${index + 1}`} />
                <span>
                  <strong>{label}</strong>
                  <small>{meta}</small>
                </span>
              </li>
            ))}
          </ul>
          <button
            className="text-button"
            type="button"
            aria-expanded={showWhy}
            onClick={() => setShowWhy((value) => !value)}
          >
            {showWhy ? t.hideWhy : t.showWhy}
          </button>
          {showWhy && <p className="why-copy">{t.whyBody}</p>}
        </article>

        <article className="panel initiative-panel">
          <div className="panel-heading">
            <span className="panel-icon mint">⌁</span>
            <p>{t.initiativeLabel}</p>
          </div>
          <dl>
            <div>
              <dt>{t.quietHours}</dt>
              <dd>{t.quietValue}</dd>
            </div>
            <div>
              <dt>{t.nudgeBudget}</dt>
              <dd>{t.nudgeValue}</dd>
            </div>
          </dl>
          <p className="principle">{t.principle}</p>
        </article>
      </section>

      <footer className="privacy-strip">
        <div>
          <span className="shield" aria-hidden="true">
            ◈
          </span>
          <div>
            <strong>{t.privacyLabel}</strong>
            <p>{t.privacyBody}</p>
          </div>
        </div>
        <span className="roadmap-note">
          {t.roadmap}
        </span>
      </footer>
    </main>
  );
}

export default App;
