import { useState } from "react";
import type { Locale } from "./locale";
import type { MemoryState } from "./memoryClient";
import "./AgentOperationPanel.css";

const copy = {
  en: {
    eyebrow: "INSTALLED APP + AGENT PHRASE · NO APP-SIDE AI API",
    title: "Open Memoryling, then say “Memoryling, wake up” in your agent project",
    intro:
      "Install and open the Windows app first. Memoryling stays local and waits while your Agent prepares the update.",
    agent: "1 · Open the installed pet",
    agentBody: "Launch Memoryling from its EXE or Start menu and keep the pet running.",
    package: "2 · Say the activation phrase",
    packageBody: "Your Agent reads authorized context and compiles a bounded bilingual update with hashed lineage.",
    pet: "3 · Memoryling updates",
    petBody: "The already-open pet applies the package locally, then speaks by trigger, expiry, cooldown, quiet hours, and daily budget.",
    waiting: "Memoryling is open and waiting",
    waitingBody: "Return to your Agent project and say “Memoryling, wake up” to begin the next update.",
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
    eyebrow: "安裝版 APP＋AGENT 發動語 · APP 不自行呼叫 AI API",
    title: "先打開 Memoryling，再回 Agent 專案說：「寵物醒來」",
    intro:
      "請先安裝並打開 Windows App；Memoryling 會留在本機等待，由你的 Agent 準備更新。",
    agent: "1 · 打開安裝版寵物",
    agentBody: "從 EXE 或開始功能表啟動 Memoryling，並讓寵物保持開啟。",
    package: "2 · 輸入發動語",
    packageBody: "Agent 讀取已授權內容，編譯有限的雙語更新並保留雜湊來源鏈。",
    pet: "3 · Memoryling 更新",
    petBody: "已開啟的寵物會在本機套用更新，再依觸發條件、期限、冷卻、安靜時段與每日額度說話。",
    waiting: "Memoryling 已開啟，正在等待",
    waitingBody: "回到你的 Agent 專案，輸入「寵物醒來」即可開始下一次更新。",
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
