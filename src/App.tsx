import { useEffect, useMemo, useRef, useState } from "react";
import "./App.css";
import memorylingIcon from "./assets/memoryling-icon.png";
import AgentOperationPanel from "./AgentOperationPanel";
import CreatureBody from "./CreatureBody";
import {
  nativeDetailEventClient,
  nativeDetailShellClient,
  type DetailEventClient,
  type DetailShellClient,
} from "./creatureClient";
import { useStoredLocale } from "./locale";
import {
  emptyMemoryState,
  nativeMemoryClient,
  type MemoryClient,
} from "./memoryClient";

const copy = {
  en: {
    prototype: "Waiting for Agent operation · app-side AI is off",
    prototypeOperationActive: "Agent operation applied · local pet rules active",
    prototypeBrowser: "Browser preview · memory access is off",
    tagline: "Your agent memories, alive.",
    intro:
      "Install and open Memoryling, then say one phrase in your agent project. The Agent turns its approved memory and recent work into appearance, dialogue, and continuing life for the already-running local pet.",
    creatureName: "Your first Memoryling",
    creatureState: "Listening for a beginning",
    creatureStateActive: "One approved memory is shaping me",
    approvedOperationCreatureLine:
      "My latest form and words arrived in a privacy-minimized Agent operation package.",
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
    noSignalsBody: "Keep this app open, then say “Memoryling, wake up” in your Agent project to create the first update package.",
    activeOperationSignal: "An Agent operation shaped the pet",
    activeOperationSignalMeta: "memory + recent work · compiled locally · rule-driven dialogue",
    activeOperationSignalBody: "The App received only generated pet state and hashed evidence references—not raw Agent memory.",
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
      "The Agent reads only what its environment already authorizes. Memoryling receives a bounded local update package; it does not scan Agent storage or call an AI API by itself.",
    roadmap: "Agent Operation Protocol v1 available",
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
    prototype: "等待 Agent 運作 · App 端 AI 關閉",
    prototypeOperationActive: "Agent 更新已套用 · 本機寵物規則運作中",
    prototypeBrowser: "瀏覽器預覽 · 記憶存取關閉",
    tagline: "讓你的 Agent 記憶，長成一個生命。",
    intro:
      "安裝並打開 Memoryling 後，回到 Agent 專案喊出一句口號；Agent 就會把獲准的記憶與近期工作，轉化成這隻已開啟的本機寵物之外觀、對話與持續生活。",
    creatureName: "你的第一隻記憶獸",
    creatureState: "正在等待故事開始",
    creatureStateActive: "一筆核准記憶正在塑造我",
    approvedOperationCreatureLine: "我最新的外觀與話語，來自一份隱私最小化的 Agent 更新包。",
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
    noSignalsBody: "請讓 App 保持開啟，再回 Agent 專案說「寵物醒來」，建立第一份更新包。",
    activeOperationSignal: "一次 Agent 運作塑造了寵物",
    activeOperationSignalMeta: "記憶＋近期工作 · 本機編譯 · 規則化對話",
    activeOperationSignalBody: "App 只收到生成後的寵物狀態與雜湊證據引用，不會收到原始 Agent 記憶。",
    initiativeLabel: "有限主動性",
    plannedBadge: "規劃中",
    quietHours: "安靜時段",
    quietValue: "22:00–09:00",
    nudgeBudget: "每日提醒額度",
    nudgeValue: "剩餘 1／2 次",
    principle: "記憶獸可以決定何時開口，但永遠不能替你決定界線。",
    privacyLabel: "Local-first 承諾",
    privacyBody:
      "Agent 只讀取其環境原本授權的資訊；Memoryling 接收有限的本機更新包，不會自行掃描 Agent 儲存空間或呼叫 AI API。",
    roadmap: "Agent Operation Protocol v1 已可使用",
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
  detailEvents?: DetailEventClient;
  detailShell?: DetailShellClient;
  browserPreview?: boolean;
}

export function DetailSurface({
  memoryClient = nativeMemoryClient,
  detailEvents = nativeDetailEventClient,
  detailShell = nativeDetailShellClient,
  browserPreview = !memoryClient.available,
}: AppProps) {
  const [locale, setLocale] = useStoredLocale();
  const [lineIndex, setLineIndex] = useState(0);
  const [eventSnoozed, setEventSnoozed] = useState(false);
  const [memoryState, setMemoryState] = useState(emptyMemoryState);
  const [guideResetStatus, setGuideResetStatus] = useState<"success" | "failed" | null>(null);
  const refreshGeneration = useRef(0);
  const t = copy[locale];
  const hasAgentOperation = memoryState.agentOperation?.state === "applied";

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

    void refreshMemoryState();

    void detailEvents
      .onRenderRevision(() => {
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
    const lines = hasAgentOperation
      ? [t.approvedOperationCreatureLine, ...t.creatureLines]
      : t.creatureLines;
    return lines[lineIndex % lines.length];
  }, [
    hasAgentOperation,
    lineIndex,
    t.approvedOperationCreatureLine,
    t.creatureLines,
  ]);

  const accessStatus = !memoryClient.available
    ? t.prototypeBrowser
    : hasAgentOperation
      ? t.prototypeOperationActive
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

  async function clearAgentOperation() {
    setMemoryState(await memoryClient.clearAgentOperation());
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
              hasCompletionStar={false}
              agentActivity={memoryState.agentOperation?.activity ?? "off"}
            />
          </button>

          <div className="creature-caption">
            <p>{t.creatureName}</p>
            <span>{hasAgentOperation ? t.creatureStateActive : t.creatureState}</span>
          </div>

          <div className="speech-card" aria-live="polite">
            <span className="speech-spark">✦</span>
            <p>{creatureLine}</p>
          </div>
          <p className="tap-hint">{t.tapHint}</p>
        </div>
      </section>

      <AgentOperationPanel
        browserPreview={browserPreview}
        locale={locale}
        memoryState={memoryState}
        onClear={memoryClient.available ? clearAgentOperation : undefined}
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
          {hasAgentOperation ? (
            <>
              <ul className="signal-list">
                <li>
                  <span className="signal-orb signal-2" />
                  <span>
                    <strong>{t.activeOperationSignal}</strong>
                    <small>{t.activeOperationSignalMeta}</small>
                  </span>
                </li>
              </ul>
              <p className="why-copy">{t.activeOperationSignalBody}</p>
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
