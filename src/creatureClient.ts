import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const CREATURE_STATE_CHANGED = "memoryling://creature-state-changed";
export const DETAIL_RESET = "memoryling://detail-reset";
export const PET_SHELL_STATE_CHANGED = "memoryling://pet-shell-state-changed";

export interface CreatureMarkRenderState {
  id: string;
  style: "completion-star";
}

export type CreatureStage = "seed";
export type CreatureBodyModule = "memory-seed-egg-v1";
export type AgentActivity =
  | "off"
  | "building"
  | "research"
  | "design"
  | "planning"
  | "debugging"
  | "writing"
  | "coordination"
  | "shipping";

export interface PetDialogue {
  id: string;
  textEn: string;
  textZhTw: string;
  trigger: "on-open" | "on-interact" | "ambient";
}

export interface CreatureRenderState {
  schemaVersion: 6;
  revision: string;
  realMemoryAccess: "off" | "codex-local";
  importState:
    | "empty"
    | "fixture-approved"
    | "thread-approved"
    | "agent-memory-approved";
  envelope: "compact";
  stage: CreatureStage;
  bodyModule: CreatureBodyModule;
  palette: "violet-mint";
  motion: "calm";
  dailyScoutState: "off" | "waiting" | "ready";
  agentOperationState: "empty" | "applied";
  agentActivity: AgentActivity;
  dialogue?: PetDialogue;
  marks: CreatureMarkRenderState[];
}

export interface CreatureRevision {
  revision: string;
}

export interface PetShellState {
  schemaVersion: 1;
  onboardingDismissed: boolean;
  alwaysOnTop: boolean;
}

export type MenuTrigger = "pointer" | "keyboard";

export interface CreatureClient {
  getRenderState(): Promise<CreatureRenderState>;
  advanceDialogue(trigger: "on-interact" | "ambient"): Promise<CreatureRenderState>;
  getPetShellState(): Promise<PetShellState>;
  showContextMenu(trigger: MenuTrigger): Promise<void>;
  startDragging(): Promise<void>;
  dismissOnboarding(): Promise<PetShellState>;
  onRenderRevision(
    listener: (payload: CreatureRevision) => void,
  ): Promise<UnlistenFn>;
  onPetShellState(
    listener: (payload: PetShellState) => void,
  ): Promise<UnlistenFn>;
}

export interface DetailEventClient {
  onRenderRevision(
    listener: (payload: CreatureRevision) => void,
  ): Promise<UnlistenFn>;
  onDetailReset(listener: () => void): Promise<UnlistenFn>;
}

export interface DetailShellClient {
  resetOnboarding(): Promise<PetShellState>;
}

function listenFor<T>(eventName: string, listener: (payload: T) => void) {
  return listen<T>(eventName, (event) => listener(event.payload));
}

export const nativeCreatureClient: CreatureClient = {
  getRenderState: () =>
    invoke<CreatureRenderState>("get_creature_render_state"),
  advanceDialogue: (trigger) =>
    invoke<CreatureRenderState>("advance_pet_dialogue", { trigger }),
  getPetShellState: () => invoke<PetShellState>("get_pet_shell_state"),
  showContextMenu: (trigger) =>
    invoke<void>("show_pet_context_menu", { trigger }),
  startDragging: () => invoke<void>("start_pet_dragging"),
  dismissOnboarding: () => invoke<PetShellState>("dismiss_pet_onboarding"),
  onRenderRevision: (listener) =>
    listenFor<CreatureRevision>(CREATURE_STATE_CHANGED, listener),
  onPetShellState: (listener) =>
    listenFor<PetShellState>(PET_SHELL_STATE_CHANGED, listener),
};

export const nativeDetailEventClient: DetailEventClient = {
  onRenderRevision: (listener) =>
    listenFor<CreatureRevision>(CREATURE_STATE_CHANGED, listener),
  onDetailReset: (listener) => listenFor<void>(DETAIL_RESET, listener),
};

export const nativeDetailShellClient: DetailShellClient = {
  resetOnboarding: () => invoke<PetShellState>("reset_pet_onboarding"),
};

export const baselineCreatureRenderState: CreatureRenderState = {
  schemaVersion: 6,
  revision: "0".repeat(64),
  realMemoryAccess: "off",
  importState: "empty",
  envelope: "compact",
  stage: "seed",
  bodyModule: "memory-seed-egg-v1",
  palette: "violet-mint",
  motion: "calm",
  dailyScoutState: "off",
  agentOperationState: "empty",
  agentActivity: "off",
  marks: [],
};

export const baselinePetShellState: PetShellState = {
  schemaVersion: 1,
  onboardingDismissed: false,
  alwaysOnTop: true,
};

const REVISION_PATTERN = /^[a-f0-9]{64}$/;
const OPAQUE_MARK_ID_PATTERN = /^[a-z0-9][a-z0-9._-]{0,63}$/;

export function isValidRevision(value: unknown): value is string {
  return typeof value === "string" && REVISION_PATTERN.test(value);
}

export function sanitizeCreatureRenderState(
  value: unknown,
): CreatureRenderState {
  if (!value || typeof value !== "object") return baselineCreatureRenderState;
  const state = value as Partial<CreatureRenderState>;
  const valid =
    state.schemaVersion === 6 &&
    isValidRevision(state.revision) &&
    (state.realMemoryAccess === "off" || state.realMemoryAccess === "codex-local") &&
    (state.importState === "empty" ||
      state.importState === "fixture-approved" ||
      state.importState === "thread-approved" ||
      state.importState === "agent-memory-approved") &&
    state.envelope === "compact" &&
    state.stage === "seed" &&
    state.bodyModule === "memory-seed-egg-v1" &&
    state.palette === "violet-mint" &&
    state.motion === "calm" &&
    (state.dailyScoutState === "off" ||
      state.dailyScoutState === "waiting" ||
      state.dailyScoutState === "ready") &&
    (state.agentOperationState === "empty" ||
      state.agentOperationState === "applied") &&
    (state.agentActivity === "off" ||
      state.agentActivity === "building" ||
      state.agentActivity === "research" ||
      state.agentActivity === "design" ||
      state.agentActivity === "planning" ||
      state.agentActivity === "debugging" ||
      state.agentActivity === "writing" ||
      state.agentActivity === "coordination" ||
      state.agentActivity === "shipping") &&
    (state.dialogue === undefined ||
      (Boolean(state.dialogue) &&
        typeof state.dialogue.id === "string" &&
        OPAQUE_MARK_ID_PATTERN.test(state.dialogue.id) &&
        typeof state.dialogue.textEn === "string" &&
        state.dialogue.textEn.length > 0 &&
        state.dialogue.textEn.length <= 160 &&
        !/[\r\n]/.test(state.dialogue.textEn) &&
        typeof state.dialogue.textZhTw === "string" &&
        state.dialogue.textZhTw.length > 0 &&
        state.dialogue.textZhTw.length <= 160 &&
        !/[\r\n]/.test(state.dialogue.textZhTw) &&
        (state.dialogue.trigger === "on-open" ||
          state.dialogue.trigger === "on-interact" ||
          state.dialogue.trigger === "ambient"))) &&
    Array.isArray(state.marks) &&
    state.marks.every(
      (mark) =>
        Boolean(mark) &&
        typeof mark.id === "string" &&
        OPAQUE_MARK_ID_PATTERN.test(mark.id) &&
        mark.style === "completion-star",
    );
  if (!valid) return baselineCreatureRenderState;
  return {
    schemaVersion: 6,
    revision: state.revision!,
    realMemoryAccess: state.realMemoryAccess!,
    importState: state.importState!,
    envelope: "compact",
    stage: "seed",
    bodyModule: "memory-seed-egg-v1",
    palette: "violet-mint",
    motion: "calm",
    dailyScoutState: state.dailyScoutState!,
    agentOperationState: state.agentOperationState!,
    agentActivity: state.agentActivity!,
    dialogue: state.dialogue
      ? {
          id: state.dialogue.id,
          textEn: state.dialogue.textEn,
          textZhTw: state.dialogue.textZhTw,
          trigger: state.dialogue.trigger,
        }
      : undefined,
    marks: state.marks!.map((mark) => ({ id: mark.id, style: mark.style })),
  };
}

export function sanitizePetShellState(value: unknown): PetShellState {
  if (!value || typeof value !== "object") return baselinePetShellState;
  const state = value as Partial<PetShellState>;
  if (
    state.schemaVersion !== 1 ||
    typeof state.onboardingDismissed !== "boolean" ||
    typeof state.alwaysOnTop !== "boolean"
  ) {
    return baselinePetShellState;
  }
  return {
    schemaVersion: 1,
    onboardingDismissed: state.onboardingDismissed,
    alwaysOnTop: state.alwaysOnTop,
  };
}
