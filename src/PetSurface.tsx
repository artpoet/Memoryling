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
  sanitizeCreatureRenderState,
  sanitizePetShellState,
  type CreatureClient,
  type MenuTrigger,
} from "./creatureClient";
import { useStoredLocale } from "./locale";
import { useCreatureRenderState } from "./useCreatureRenderState";
import "./PetSurface.css";

const DRAG_THRESHOLD_DIP = 6;
const DIALOGUE_VISIBLE_MS = 7_000;

interface ClipboardWriter {
  writeText(text: string): Promise<void>;
}

const petCopy = {
  en: {
    label:
      "Memoryling is waiting for an Agent operation. Right-click for menu; drag to move.",
    accessOff: "Waiting for Agent operation · memory access off",
    operationLabel:
      "Memoryling has an Agent-made update. Right-click for menu; drag to move; click to talk.",
    operationActive: "Agent operation applied · app memory scanning off",
    onboardingTitle: "Memoryling is open",
    drag: "Drag me to move me.",
    menu: "Right-click me, then choose Open Memoryling.",
    recovery: "If I hide, find me from the system tray.",
    activationPhrase: "Memoryling, wake up",
    operate: "Return to your Agent project and enter the activation phrase: “Memoryling, wake up”.",
    copyPhrase: "Copy “Memoryling, wake up”",
    copyingPhrase: "Copying…",
    copiedPhrase: "Copied “Memoryling, wake up”",
    copyFailed: "Copy failed — try again",
    dismissDialogue: "Dismiss Memoryling's message",
    privacy: "The app never scans Agent memory or calls AI by itself.",
    skip: "Got it",
    loading: "Waking up locally",
    reaction: "Memoryling gives a quiet blink.",
  },
  "zh-TW": {
    label: "Memoryling 正在等待 Agent 運作。按右鍵開啟選單；拖曳即可移動。",
    accessOff: "等待 Agent 運作 · 記憶存取關閉",
    operationLabel:
      "Memoryling 已收到 Agent 產生的更新。按右鍵開啟選單；拖曳可移動；點一下可交談。",
    operationActive: "Agent 運作已套用 · App 不掃描記憶",
    onboardingTitle: "Memoryling 已打開",
    drag: "拖曳我來移動位置。",
    menu: "按右鍵，再選擇開啟 Memoryling。",
    recovery: "找不到我時，可以從系統匣叫我回來。",
    activationPhrase: "醒來吧我的寵物",
    operate: "回到你目前工作的 Agent 專案，輸入發動語：「醒來吧我的寵物」。",
    copyPhrase: "複製「醒來吧我的寵物」",
    copyingPhrase: "複製中…",
    copiedPhrase: "已複製「醒來吧我的寵物」",
    copyFailed: "複製失敗，請再試一次",
    dismissDialogue: "關閉 Memoryling 的話",
    privacy: "App 不會自行掃描 Agent 記憶，也不會自行呼叫 AI。",
    skip: "知道了",
    loading: "正在本機醒來",
    reaction: "Memoryling 安靜地眨了眨眼。",
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
  clipboard?: ClipboardWriter | null;
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

export function PetSurface({
  client = nativeCreatureClient,
  clipboard,
}: PetSurfaceProps) {
  const [locale] = useStoredLocale();
  const t = petCopy[locale];
  const reducedMotion = useReducedMotion();
  const { renderState, setRenderState, shellState, setShellState, ready } =
    useCreatureRenderState(client);
  const [reaction, setReaction] = useState(false);
  const [dialogueVisible, setDialogueVisible] = useState(false);
  const [copyStatus, setCopyStatus] = useState<
    "idle" | "copying" | "copied" | "failed"
  >("idle");
  const dragGesture = useRef<DragGesture | null>(null);
  const suppressClick = useRef(false);
  const suppressClickTimer = useRef<number | undefined>(undefined);
  const reactionTimer = useRef<number | undefined>(undefined);
  const dialogueTimer = useRef<number | undefined>(undefined);
  const hasCompletionStar = renderState.marks.some(
    (mark) => mark.style === "completion-star",
  );
  const hasAgentOperation = renderState.agentOperationState === "applied";
  const dialogueText = renderState.dialogue
    ? locale === "zh-TW"
      ? renderState.dialogue.textZhTw
      : renderState.dialogue.textEn
    : "";

  useEffect(() => {
    document.title = "Memoryling";
    return () => {
      window.clearTimeout(reactionTimer.current);
      window.clearTimeout(dialogueTimer.current);
      window.clearTimeout(suppressClickTimer.current);
    };
  }, []);

  useEffect(() => {
    window.clearTimeout(dialogueTimer.current);
    if (!renderState.dialogue) {
      setDialogueVisible(false);
      return;
    }
    setDialogueVisible(true);
    dialogueTimer.current = window.setTimeout(
      () => setDialogueVisible(false),
      DIALOGUE_VISIBLE_MS,
    );
  }, [renderState.dialogue?.id]);

  function dismissDialogue() {
    window.clearTimeout(dialogueTimer.current);
    setDialogueVisible(false);
  }

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
    if (hasAgentOperation) {
      window.clearTimeout(dialogueTimer.current);
      setDialogueVisible(true);
      dialogueTimer.current = window.setTimeout(
        () => setDialogueVisible(false),
        DIALOGUE_VISIBLE_MS,
      );
      void client
        .advanceDialogue("on-interact")
        .then((next) => setRenderState(sanitizeCreatureRenderState(next)))
        .catch(() => undefined);
    }
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

  async function copyActivationPhrase() {
    setCopyStatus("copying");
    try {
      const writer =
        clipboard === undefined ? window.navigator.clipboard : clipboard;
      if (!writer) throw new Error("Clipboard unavailable");
      await writer.writeText(t.activationPhrase);
      setCopyStatus("copied");
    } catch {
      setCopyStatus("failed");
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
        aria-label={hasAgentOperation ? t.operationLabel : t.label}
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
          motionEnabled={!reducedMotion}
          stage={renderState.stage}
          agentActivity={renderState.agentActivity}
        />
      </button>

      <p className="pet-access-status">
        {hasAgentOperation ? t.operationActive : t.accessOff}
      </p>

      {!onboardingVisible && !hasAgentOperation && (
        <p className="pet-dialogue pet-dialogue-reminder" role="status">
          {t.operate}
        </p>
      )}

      {!onboardingVisible && renderState.dialogue && dialogueVisible && (
        <button
          aria-label={`${t.dismissDialogue}: ${dialogueText}`}
          aria-live="polite"
          className="pet-dialogue pet-dialogue-message"
          onClick={dismissDialogue}
          title={t.dismissDialogue}
          type="button"
        >
          {dialogueText}
        </button>
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
            <li>{t.operate}</li>
            <li>{t.privacy}</li>
          </ul>
          <div className="pet-onboarding-actions">
            <button
              className="pet-copy-button"
              disabled={copyStatus === "copying"}
              onClick={() => void copyActivationPhrase()}
              type="button"
            >
              {copyStatus === "copying"
                ? t.copyingPhrase
                : copyStatus === "copied"
                  ? t.copiedPhrase
                  : copyStatus === "failed"
                    ? t.copyFailed
                    : t.copyPhrase}
            </button>
            <button
              className="pet-dismiss-button"
              onClick={() => void dismissOnboarding()}
              type="button"
            >
              {t.skip}
            </button>
          </div>
        </section>
      )}

      <span aria-live="polite" className="pet-announcer" role="status">
        {!ready ? t.loading : reaction ? t.reaction : ""}
      </span>
    </main>
  );
}

export default PetSurface;
