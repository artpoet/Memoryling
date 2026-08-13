import {
  useEffect,
  useRef,
  useState,
  type KeyboardEvent,
  type PointerEvent,
} from "react";
import CreatureBody from "./CreatureBody";
import {
  nativeCreatureClient,
  sanitizePetShellState,
  type CreatureClient,
  type MenuTrigger,
} from "./creatureClient";
import { useStoredLocale } from "./locale";
import { useCreatureRenderState } from "./useCreatureRenderState";
import "./PetSurface.css";

const DRAG_THRESHOLD_DIP = 6;

const petCopy = {
  en: {
    label:
      "Memoryling. Memory access is off. Right-click for menu; drag to move.",
    accessOff: "Memory access off",
    threadLabel:
      "Memoryling. One Codex work record is active; durable memory access is off. Right-click for menu; drag to move.",
    threadActive: "1 Codex work record active · durable memory off",
    agentLabel:
      "Memoryling. Codex Agent memories are connected read-only and sync locally. Right-click for menu; drag to move.",
    agentActive: "Codex Agent memories connected · read-only auto-sync",
    onboardingTitle: "Meet your Memoryling",
    drag: "Drag me to move me.",
    menu: "Right-click me, then choose Open Memoryling.",
    recovery: "If I hide, find me from the system tray.",
    privacy: "Real memory access is currently off.",
    skip: "Got it",
    loading: "Waking up locally",
    reaction: "Memoryling gives a quiet blink.",
    scoutReady: "I found something useful for you. Open Memoryling to see it.",
  },
  "zh-TW": {
    label: "Memoryling。記憶存取關閉。按右鍵開啟選單；拖曳即可移動。",
    accessOff: "記憶存取關閉",
    threadLabel:
      "Memoryling。一筆 Codex 工作紀錄已啟用；durable memory 存取仍關閉。按右鍵開啟選單；拖曳即可移動。",
    threadActive: "1 筆 Codex 工作紀錄已啟用 · durable memory 關閉",
    agentLabel:
      "Memoryling。Codex Agent 記憶已唯讀連線並在本機自動同步。按右鍵開啟選單；拖曳即可移動。",
    agentActive: "Codex Agent 記憶已連線 · 唯讀自動同步",
    onboardingTitle: "認識你的 Memoryling",
    drag: "拖曳我來移動位置。",
    menu: "按右鍵，再選擇開啟 Memoryling。",
    recovery: "找不到我時，可以從系統匣叫我回來。",
    privacy: "真實記憶存取目前關閉。",
    skip: "知道了",
    loading: "正在本機醒來",
    reaction: "Memoryling 安靜地眨了眨眼。",
    scoutReady: "我找到一則可能對你有用的情報，打開 Memoryling 看看吧。",
  },
} as const;

interface DragGesture {
  pointerId: number;
  x: number;
  y: number;
  dragging: boolean;
}

export interface PetSurfaceProps {
  client?: CreatureClient;
}

function useReducedMotion() {
  const [reduced, setReduced] = useState(
    () =>
      window.matchMedia?.("(prefers-reduced-motion: reduce)").matches ?? false,
  );
  useEffect(() => {
    const media = window.matchMedia?.("(prefers-reduced-motion: reduce)");
    if (!media) return;
    const update = () => setReduced(media.matches);
    media.addEventListener("change", update);
    return () => media.removeEventListener("change", update);
  }, []);
  return reduced;
}

export function PetSurface({ client = nativeCreatureClient }: PetSurfaceProps) {
  const [locale] = useStoredLocale();
  const t = petCopy[locale];
  const reducedMotion = useReducedMotion();
  const { renderState, shellState, setShellState, ready } =
    useCreatureRenderState(client);
  const [reaction, setReaction] = useState(false);
  const dragGesture = useRef<DragGesture | null>(null);
  const suppressClick = useRef(false);
  const suppressClickTimer = useRef<number | undefined>(undefined);
  const reactionTimer = useRef<number | undefined>(undefined);
  const hasCompletionStar = renderState.marks.some(
    (mark) => mark.style === "completion-star",
  );
  const hasMemoryHalo = renderState.marks.some(
    (mark) => mark.style === "memory-halo",
  );
  const hasThreadImport = renderState.importState === "thread-approved";
  const hasAgentMemory =
    renderState.importState === "agent-memory-approved" &&
    renderState.realMemoryAccess === "codex-local";
  const scoutReady = renderState.dailyScoutState === "ready";

  useEffect(() => {
    document.title = "Memoryling";
    return () => {
      window.clearTimeout(reactionTimer.current);
      window.clearTimeout(suppressClickTimer.current);
    };
  }, []);

  function showMenu(trigger: MenuTrigger) {
    void client.showContextMenu(trigger).catch(() => undefined);
  }

  function handlePointerDown(event: PointerEvent<HTMLButtonElement>) {
    if (event.button !== 0 || event.isPrimary === false) return;
    dragGesture.current = {
      pointerId: event.pointerId,
      x: event.clientX,
      y: event.clientY,
      dragging: false,
    };
  }

  function handlePointerMove(event: PointerEvent<HTMLButtonElement>) {
    const gesture = dragGesture.current;
    if (!gesture || gesture.pointerId !== event.pointerId || gesture.dragging)
      return;
    if (
      Math.hypot(event.clientX - gesture.x, event.clientY - gesture.y) <
      DRAG_THRESHOLD_DIP
    ) {
      return;
    }
    gesture.dragging = true;
    suppressClick.current = true;
    window.clearTimeout(suppressClickTimer.current);
    suppressClickTimer.current = window.setTimeout(() => {
      suppressClick.current = false;
    }, 2000);
    void client
      .startDragging()
      .then(() => {
        if (dragGesture.current !== gesture) return;
        dragGesture.current = null;
        window.clearTimeout(suppressClickTimer.current);
        suppressClickTimer.current = window.setTimeout(() => {
          suppressClick.current = false;
        }, 0);
      })
      .catch(() => {
        if (dragGesture.current === gesture) dragGesture.current = null;
        gesture.dragging = false;
        suppressClick.current = false;
        window.clearTimeout(suppressClickTimer.current);
      });
  }

  function endPointerGesture(_event: PointerEvent<HTMLButtonElement>) {
    const gesture = dragGesture.current;
    if (!gesture) return;
    const wasDragging = gesture.dragging;
    dragGesture.current = null;
    if (wasDragging) {
      suppressClickTimer.current = window.setTimeout(() => {
        suppressClick.current = false;
      }, 0);
    }
  }

  function handleClick(event: React.MouseEvent<HTMLButtonElement>) {
    if (event.detail === 0 || suppressClick.current) {
      suppressClick.current = false;
      return;
    }
    setReaction(true);
    window.clearTimeout(reactionTimer.current);
    reactionTimer.current = window.setTimeout(() => setReaction(false), 1400);
  }

  function handleKeyDown(event: KeyboardEvent<HTMLButtonElement>) {
    if (event.repeat) return;
    const opensMenu =
      event.key === "Enter" ||
      event.key === " " ||
      event.key === "ContextMenu" ||
      (event.shiftKey && event.key === "F10");
    if (!opensMenu) return;
    event.preventDefault();
    showMenu("keyboard");
  }

  async function dismissOnboarding() {
    try {
      setShellState(sanitizePetShellState(await client.dismissOnboarding()));
    } catch {
      // Keep onboarding visible when persistence or resize fails.
    }
  }

  const onboardingVisible = ready && !shellState.onboardingDismissed;

  return (
    <main
      className={`pet-surface${onboardingVisible ? " pet-surface-onboarding" : ""}`}
      data-motion={reducedMotion ? "reduced" : "full"}
      data-testid="pet-surface"
    >
      <button
        aria-haspopup="menu"
        aria-label={hasAgentMemory ? t.agentLabel : hasThreadImport ? t.threadLabel : t.label}
        className={`pet-button${reaction ? " pet-reacting" : ""}`}
        onClick={handleClick}
        onContextMenu={(event) => {
          event.preventDefault();
          showMenu("pointer");
        }}
        onKeyDown={handleKeyDown}
        onPointerCancel={endPointerGesture}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={endPointerGesture}
        type="button"
      >
        <CreatureBody
          bodyModule={renderState.bodyModule}
          hasCompletionStar={hasCompletionStar}
          hasMemoryHalo={hasMemoryHalo}
          motionEnabled={!reducedMotion}
          stage={renderState.stage}
        />
      </button>

      <p className="pet-access-status">
        {hasAgentMemory ? t.agentActive : hasThreadImport ? t.threadActive : t.accessOff}
      </p>

      {scoutReady && !onboardingVisible && (
        <p className="pet-scout-ready" role="status">
          {t.scoutReady}
        </p>
      )}

      {onboardingVisible && (
        <section
          className="pet-onboarding"
          aria-labelledby="pet-onboarding-title"
        >
          <h1 id="pet-onboarding-title">{t.onboardingTitle}</h1>
          <ul>
            <li>{t.drag}</li>
            <li>{t.menu}</li>
            <li>{t.recovery}</li>
            <li>{t.privacy}</li>
          </ul>
          <button onClick={() => void dismissOnboarding()} type="button">
            {t.skip}
          </button>
        </section>
      )}

      <span aria-live="polite" className="pet-announcer" role="status">
        {!ready ? t.loading : reaction ? t.reaction : ""}
      </span>
    </main>
  );
}

export default PetSurface;
