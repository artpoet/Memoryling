import { useEffect, useMemo, useRef, useState } from "react";
import "./App.css";
import memorylingIcon from "./assets/memoryling-icon.png";
import CreatureBody from "./CreatureBody";
import DailyScoutPanel from "./DailyScoutPanel";
import FirstMemoryFlow from "./FirstMemoryFlow";
import ProductSetup from "./ProductSetup";
import {
  nativeDetailEventClient,
  nativeDetailShellClient,
  type DetailEventClient,
  type DetailShellClient,
} from "./creatureClient";
import { useStoredLocale } from "./locale";
import {
  nativeDailyScoutClient,
  type DailyScoutClient,
} from "./dailyScoutClient";
import {
  emptyMemoryState,
  nativeMemoryClient,
  type MemoryClient,
} from "./memoryClient";
import {
  completeProductSetupState,
  nativeProductSetupClient,
  type ProductSetupClient,
  type ProductSetupState,
} from "./productSetupClient";

const copy = {
  en: {
    prototype: "Memory access is off · no approved sources",
    prototypeActive: "Fixture pilot active · real memory access is off",
    prototypeThreadActive: "1 Codex work record active · durable memory access is off",
    prototypeBrowser: "Browser preview · memory access is off",
    tagline: "Your agent memories, alive.",
    intro:
      "A small desktop life that grows from what your AI agents remember—and, only if you opt in, helps your current work with one useful daily insight.",
    creatureName: "Your first Memoryling",
    creatureState: "Listening for a beginning",
    creatureStateActive: "One approved memory is shaping me",
    approvedCreatureLine:
      "A completion star appeared because you approved one synthetic Codex memory.",
    approvedThreadCreatureLine:
      "A completion star appeared because you approved one Codex work record.",
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
    conceptBadge: "CONCEPT",
    memoryLabel: "Memory signals",
    noSignals: "No source-backed signals",
    noSignalsBody: "Approve the synthetic fixture above to create the first real lineage path.",
    activeSignal: "A completion left a star",
    activeSignalMeta: "completion · persisted · explainable",
    activeSignalBody: "Its complete source → event → signal → mark lineage is available above.",
    initiativeLabel: "Bounded initiative",
    plannedBadge: "PLANNED",
    quietHours: "Quiet hours",
    quietValue: "22:00–09:00",
    nudgeBudget: "Daily nudge budget",
    nudgeValue: "1 of 2 available",
    principle:
      "Memoryling may decide when to speak, but it never decides your limits.",
    privacyLabel: "Local-first promise",
    privacyBody:
      "Approved records stay local and read-only at the source. Codex durable memories remain disconnected; raw records, credentials, and private files are never silently uploaded.",
    roadmap: "Local fixture and experimental work-record lineage available",
    brandHome: "Memoryling home",
    languageLabel: "Language",
    dashboardLabel: "Memoryling status dashboard",
    browserShellTitle: "Floating pet is available in the Windows desktop app",
    browserShellBody:
      "This browser preview does not imitate native pet, menu, tray, window, or persistence behavior. Memory access remains off.",
    showPetGuide: "Show pet guide again",
    guideReset: "The pet guide will appear the next time the pet is shown.",
    guideResetFailed: "The local setting did not change. Try again from the desktop app.",
  },
  "zh-TW": {
    prototype: "記憶存取關閉 · 尚無核准來源",
    prototypeActive: "Fixture 試行中 · 真實記憶存取關閉",
    prototypeThreadActive: "1 筆 Codex 工作紀錄已啟用 · durable memory 存取關閉",
    prototypeBrowser: "瀏覽器預覽 · 記憶存取關閉",
    tagline: "讓你的 Agent 記憶，長成一個生命。",
    intro:
      "一個從 AI Agent 記憶長大的桌面生命；只有你選擇開啟時，牠也會每天帶回一則對目前工作有用的情報。",
    creatureName: "你的第一隻記憶獸",
    creatureState: "正在等待故事開始",
    creatureStateActive: "一筆核准記憶正在塑造我",
    approvedCreatureLine: "你核准了一筆合成 Codex 記憶，因此完成之星出現了。",
    approvedThreadCreatureLine: "你核准了一筆 Codex 工作紀錄，因此完成之星出現了。",
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
    conceptBadge: "概念示意",
    memoryLabel: "記憶訊號",
    noSignals: "尚無具來源鏈的訊號",
    noSignalsBody: "核准上方合成 fixture 後，才會建立第一條真實來源鏈。",
    activeSignal: "一個完成事件留下了星星",
    activeSignalMeta: "完成 · 已持久化 · 可解釋",
    activeSignalBody: "完整的來源 → 事件 → 訊號 → 印記鏈，可在上方檢視。",
    initiativeLabel: "有限主動性",
    plannedBadge: "規劃中",
    quietHours: "安靜時段",
    quietValue: "22:00–09:00",
    nudgeBudget: "每日提醒額度",
    nudgeValue: "剩餘 1／2 次",
    principle: "記憶獸可以決定何時開口，但永遠不能替你決定界線。",
    privacyLabel: "Local-first 承諾",
    privacyBody:
      "核准紀錄留在本機，來源維持唯讀；Codex durable memories 仍未連線，原始紀錄、憑證與私密檔案不會被偷偷上傳。",
    roadmap: "桌面版已具備 fixture 與實驗性工作紀錄來源鏈",
    brandHome: "Memoryling 首頁",
    languageLabel: "語言",
    dashboardLabel: "Memoryling 狀態面板",
    browserShellTitle: "浮動寵物只在 Windows 桌面 App 提供",
    browserShellBody:
      "這個瀏覽器預覽不會假裝原生寵物、選單、系統匣、視窗或持久化行為；記憶存取仍維持關閉。",
    showPetGuide: "再次顯示寵物指南",
    guideReset: "下次顯示寵物時，會再次出現操作指南。",
    guideResetFailed: "本機設定沒有變更；請從桌面 App 再試一次。",
  },
} as const;

interface AppProps {
  memoryClient?: MemoryClient;
  dailyScoutClient?: DailyScoutClient;
  detailEvents?: DetailEventClient;
  detailShell?: DetailShellClient;
  browserPreview?: boolean;
  productSetupClient?: ProductSetupClient;
}

export function DetailSurface({
  memoryClient = nativeMemoryClient,
  dailyScoutClient = nativeDailyScoutClient,
  detailEvents = nativeDetailEventClient,
  detailShell = nativeDetailShellClient,
  browserPreview = !memoryClient.available,
  productSetupClient = nativeProductSetupClient,
}: AppProps) {
  const [locale, setLocale] = useStoredLocale();
  const [lineIndex, setLineIndex] = useState(0);
  const [eventSnoozed, setEventSnoozed] = useState(false);
  const [memoryState, setMemoryState] = useState(emptyMemoryState);
  const [detailResetRevision, setDetailResetRevision] = useState(0);
  const [dailyScoutRefreshRevision, setDailyScoutRefreshRevision] = useState(0);
  const [guideResetStatus, setGuideResetStatus] = useState<"success" | "failed" | null>(null);
  const [productSetupState, setProductSetupState] =
    useState<ProductSetupState | null>(() =>
      productSetupClient.available ? null : completeProductSetupState,
    );
  const refreshGeneration = useRef(0);
  const t = copy[locale];
  const activeMark = memoryState.marks[0];
  const hasApprovedMemory = Boolean(activeMark);
  const hasApprovedThread =
    activeMark?.lineage[0]?.adapterId === "codex-app-server-thread";

  useEffect(() => {
    if (!productSetupClient.available) return;
    let active = true;
    void productSetupClient
      .getState()
      .then((state) => {
        if (active) setProductSetupState(state);
      })
      .catch(() => {
        if (active) setProductSetupState(completeProductSetupState);
      });
    return () => {
      active = false;
    };
  }, [productSetupClient]);

  useEffect(() => {
    if (!memoryClient.available) return;
    let active = true;
    let revisionUnlisten: (() => void) | undefined;
    let resetUnlisten: (() => void) | undefined;

    async function refreshMemoryState() {
      const generation = ++refreshGeneration.current;
      try {
        const state = await memoryClient.getState();
        if (active && generation === refreshGeneration.current) setMemoryState(state);
      } catch {
        // Keep the last persisted state and never expose native error details.
      }
    }

    void detailEvents
      .onRenderRevision(() => {
        setDailyScoutRefreshRevision((value) => value + 1);
        void refreshMemoryState();
      })
      .then((unlisten) => {
        if (active) revisionUnlisten = unlisten;
        else unlisten();
      })
      .catch(() => undefined);
    void detailEvents
      .onDetailReset(() => {
        if (!active) return;
        setDetailResetRevision((value) => value + 1);
        setDailyScoutRefreshRevision((value) => value + 1);
        void refreshMemoryState();
      })
      .then((unlisten) => {
        if (active) resetUnlisten = unlisten;
        else unlisten();
      })
      .catch(() => undefined);

    return () => {
      active = false;
      refreshGeneration.current += 1;
      revisionUnlisten?.();
      resetUnlisten?.();
    };
  }, [detailEvents, memoryClient]);

  const creatureLine = useMemo(() => {
    const lines = hasApprovedMemory
      ? [
          hasApprovedThread
            ? t.approvedThreadCreatureLine
            : t.approvedCreatureLine,
          ...t.creatureLines,
        ]
      : t.creatureLines;
    return lines[lineIndex % lines.length];
  }, [
    hasApprovedMemory,
    hasApprovedThread,
    lineIndex,
    t.approvedCreatureLine,
    t.approvedThreadCreatureLine,
    t.creatureLines,
  ]);

  const accessStatus = !memoryClient.available
    ? t.prototypeBrowser
    : hasApprovedMemory
      ? hasApprovedThread
        ? t.prototypeThreadActive
        : t.prototypeActive
      : t.prototype;

  async function resetPetGuide() {
    setGuideResetStatus(null);
    try {
      await detailShell.resetOnboarding();
      setGuideResetStatus("success");
    } catch {
      setGuideResetStatus("failed");
    }
  }

  if (productSetupState === null) {
    return (
      <main className="product-setup-loading" role="status">
        Memoryling
      </main>
    );
  }

  if (!browserPreview && !productSetupState.setupComplete) {
    return (
      <ProductSetup
        dailyScoutClient={dailyScoutClient}
        locale={locale}
        onComplete={() => setProductSetupState(completeProductSetupState)}
        onLocaleChange={setLocale}
        setupClient={productSetupClient}
      />
    );
  }

  return (
    <main className="app-shell">
      <header className="topbar">
        <a className="brand" href="#top" aria-label={t.brandHome}>
          <span className="brand-mark" aria-hidden="true">
            <img src={memorylingIcon} alt="" />
          </span>
          <span>Memoryling</span>
        </a>
        <div className="topbar-actions">
          <span className="prototype-pill">{accessStatus}</span>
          <div className="locale-toggle" aria-label={t.languageLabel}>
            <button
              className={locale === "en" ? "active" : ""}
              onClick={() => setLocale("en")}
              type="button"
              aria-pressed={locale === "en"}
            >
              EN
            </button>
            <button
              className={locale === "zh-TW" ? "active" : ""}
              onClick={() => setLocale("zh-TW")}
              type="button"
              aria-pressed={locale === "zh-TW"}
            >
              繁中
            </button>
          </div>
          {!browserPreview && (
            <button
              className="guide-reset-button"
              onClick={() => void resetPetGuide()}
              type="button"
            >
              {t.showPetGuide}
            </button>
          )}
        </div>
      </header>

      <p className="guide-reset-status" aria-live="polite" role="status">
        {guideResetStatus === "success"
          ? t.guideReset
          : guideResetStatus === "failed"
            ? t.guideResetFailed
            : ""}
      </p>

      {browserPreview && (
        <aside className="browser-shell-boundary" data-testid="browser-shell-boundary">
          <strong>{t.browserShellTitle}</strong>
          <p>{t.browserShellBody}</p>
        </aside>
      )}

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
            <CreatureBody
              hasCompletionStar={
                hasApprovedMemory && activeMark.style === "completion-star"
              }
            />
          </button>

          <div className="creature-caption">
            <p>{t.creatureName}</p>
            <span>{hasApprovedMemory ? t.creatureStateActive : t.creatureState}</span>
          </div>

          <div className="speech-card" aria-live="polite">
            <span className="speech-spark">✦</span>
            <p>{creatureLine}</p>
          </div>
          <p className="tap-hint">{t.tapHint}</p>
        </div>
      </section>

      <FirstMemoryFlow
        client={memoryClient}
        locale={locale}
        memoryState={memoryState}
        onMemoryStateChange={setMemoryState}
        resetRevision={detailResetRevision}
      />

      <DailyScoutPanel
        client={dailyScoutClient}
        locale={locale}
        refreshRevision={dailyScoutRefreshRevision}
      />

      <section className="dashboard" aria-label={t.dashboardLabel}>
        <article className="panel event-panel">
          <div className="panel-heading">
            <span className="panel-icon amber">◌</span>
            <p>{t.eventLabel}</p>
            <span className="live-dot">{t.conceptBadge}</span>
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
          {hasApprovedMemory ? (
            <>
              <ul className="signal-list">
                <li>
                  <span className="signal-orb signal-2" />
                  <span>
                    <strong>{t.activeSignal}</strong>
                    <small>{t.activeSignalMeta}</small>
                  </span>
                </li>
              </ul>
              <p className="why-copy">{t.activeSignalBody}</p>
            </>
          ) : (
            <div className="empty-signal-state">
              <strong>{t.noSignals}</strong>
              <p>{t.noSignalsBody}</p>
            </div>
          )}
        </article>

        <article className="panel initiative-panel">
          <div className="panel-heading">
            <span className="panel-icon mint">⌁</span>
            <p>{t.initiativeLabel}</p>
            <span className="planned-dot">{t.plannedBadge}</span>
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

export const App = DetailSurface;
export default DetailSurface;
