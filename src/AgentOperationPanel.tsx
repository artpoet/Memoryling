import { useState } from "react";
import type { Locale } from "./locale";
import type { MemoryState } from "./memoryClient";
import "./AgentOperationPanel.css";

const copy = {
  en: {
    eyebrow: "AGENT-OPERATED · NO APP-SIDE AI API",
    title: "Say “Run Memoryling” in your agent project",
    intro:
      "Stay in the conversation. Your agent compiles the update, wakes the installed app, and waits for the pet to apply it.",
    agent: "1 · Agent understands",
    agentBody: "Reads its available memories, current thread, and project SSOT after your phrase.",
    package: "2 · Agent compiles",
    packageBody: "Creates a bounded appearance profile and 3–12 bilingual dialogue cards with hashed lineage.",
    pet: "3 · Memoryling lives",
    petBody: "Opens automatically, applies the package locally, then speaks by trigger, expiry, cooldown, quiet hours, and daily budget.",
    waiting: "Waiting for the next Agent operation",
    waitingBody: "Say the phrase in a configured Agent project. The Agent handles compilation, submission, and opening the pet.",
    applied: "Latest Agent operation applied",
    appliedBody: (count: number, activity: string) =>
      `${count} dialogue cards are ready · dominant activity: ${activity}`,
    browser: "Desktop inbox is unavailable in browser preview; this page performs no AI or memory read.",
    privacy: "The package contains generated pet state and hashed references—not raw memories, prompts, paths, secrets, or reasoning.",
    clear: "Clear this pet update",
    clearing: "Clearing…",
    clearFailed: "The local update could not be cleared.",
  },
  "zh-TW": {
    eyebrow: "由 AGENT 運作 · APP 不自行呼叫 AI API",
    title: "在 Agent 專案中說：「運作 Memoryling」",
    intro:
      "留在對話裡就好；Agent 會編譯更新、喚醒已安裝的 App，並等待寵物完成套用。",
    agent: "1 · Agent 理解",
    agentBody: "聽見口號後，讀取可用記憶、目前對話與專案 SSOT。",
    package: "2 · Agent 編譯",
    packageBody: "產生有限的外觀輪廓與 3–12 組雙語對話卡，並保留雜湊來源鏈。",
    pet: "3 · Memoryling 生活",
    petBody: "自動出現並在本機套用更新，再依觸發條件、期限、冷卻、安靜時段與每日額度說話。",
    waiting: "正在等待下一次 Agent 運作",
    waitingBody: "在已設定的 Agent 專案喊出口號即可；Agent 會處理編譯、提交與打開寵物。",
    applied: "最新 Agent 更新已套用",
    appliedBody: (count: number, activity: string) =>
      `${count} 組對話已就緒 · 主要活動：${activity}`,
    browser: "瀏覽器預覽沒有桌面收件匣；這個頁面不會自行執行 AI 或讀取記憶。",
    privacy: "更新包只含生成後的寵物狀態與雜湊引用，不含原始記憶、prompt、路徑、祕密或 reasoning。",
    clear: "清除這次寵物更新",
    clearing: "正在清除……",
    clearFailed: "無法清除本機更新。",
  },
} as const;

interface AgentOperationPanelProps {
  browserPreview: boolean;
  locale: Locale;
  memoryState: MemoryState;
  onClear?(): Promise<void>;
}

export function AgentOperationPanel({
  browserPreview,
  locale,
  memoryState,
  onClear,
}: AgentOperationPanelProps) {
  const t = copy[locale];
  const operation = memoryState.agentOperation;
  const [clearing, setClearing] = useState(false);
  const [clearFailed, setClearFailed] = useState(false);

  async function clearOperation() {
    if (!onClear || clearing) return;
    setClearing(true);
    setClearFailed(false);
    try {
      await onClear();
    } catch {
      setClearFailed(true);
    } finally {
      setClearing(false);
    }
  }
  return (
    <section className="agent-operation-panel" aria-labelledby="agent-operation-title">
      <div className="agent-operation-heading">
        <div>
          <p>{t.eyebrow}</p>
          <h2 id="agent-operation-title">{t.title}</h2>
          <span>{t.intro}</span>
        </div>
        <strong>{operation ? "APPLIED" : "READY"}</strong>
      </div>
      <ol className="agent-operation-steps">
        <li><strong>{t.agent}</strong><span>{t.agentBody}</span></li>
        <li><strong>{t.package}</strong><span>{t.packageBody}</span></li>
        <li><strong>{t.pet}</strong><span>{t.petBody}</span></li>
      </ol>
      <div className={`agent-operation-status${operation ? " applied" : ""}`} role="status">
        <strong>{operation ? t.applied : t.waiting}</strong>
        <span>{operation ? t.appliedBody(operation.dialogueCount, operation.activity) : t.waitingBody}</span>
      </div>
      <p className="agent-operation-privacy">{browserPreview ? t.browser : t.privacy}</p>
      {operation && onClear && !browserPreview && (
        <button
          className="agent-operation-clear"
          disabled={clearing}
          onClick={() => void clearOperation()}
          type="button"
        >
          {clearing ? t.clearing : t.clear}
        </button>
      )}
      {clearFailed && <p className="agent-operation-clear-error" role="alert">{t.clearFailed}</p>}
    </section>
  );
}

export default AgentOperationPanel;
