import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const CREATURE_STATE_CHANGED = "memoryling://creature-state-changed";
export const DETAIL_RESET = "memoryling://detail-reset";
export const PET_SHELL_STATE_CHANGED = "memoryling://pet-shell-state-changed";

export interface CreatureMarkRenderState {
  id: string;
  style: "completion-star";
}

export interface CreatureRenderState {
  schemaVersion: 1;
  revision: string;
  realMemoryAccess: "off";
  fixtureState: "empty" | "approved";
  envelope: "compact";
  bodyModule: "baseline";
  palette: "violet-mint";
  motion: "calm";
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
  getPetShellState(): Promise<PetShellState>;
  showContextMenu(trigger: MenuTrigger): Promise<void>;
  startDragging(): Promise<void>;
  dismissOnboarding(): Promise<PetShellState>;
  onRenderRevision(listener: (payload: CreatureRevision) => void): Promise<UnlistenFn>;
  onPetShellState(listener: (payload: PetShellState) => void): Promise<UnlistenFn>;
}

export interface DetailEventClient {
  onRenderRevision(listener: (payload: CreatureRevision) => void): Promise<UnlistenFn>;
  onDetailReset(listener: () => void): Promise<UnlistenFn>;
}

export interface DetailShellClient {
  resetOnboarding(): Promise<PetShellState>;
}

function listenFor<T>(eventName: string, listener: (payload: T) => void) {
  return listen<T>(eventName, (event) => listener(event.payload));
}

export const nativeCreatureClient: CreatureClient = {
  getRenderState: () => invoke<CreatureRenderState>("get_creature_render_state"),
  getPetShellState: () => invoke<PetShellState>("get_pet_shell_state"),
  showContextMenu: (trigger) =>
    invoke<void>("show_pet_context_menu", { trigger }),
  startDragging: () => invoke<void>("start_pet_dragging"),
  dismissOnboarding: () =>
    invoke<PetShellState>("dismiss_pet_onboarding"),
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
  schemaVersion: 1,
  revision: "0".repeat(64),
  realMemoryAccess: "off",
  fixtureState: "empty",
  envelope: "compact",
  bodyModule: "baseline",
  palette: "violet-mint",
  motion: "calm",
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

export function sanitizeCreatureRenderState(value: unknown): CreatureRenderState {
  if (!value || typeof value !== "object") return baselineCreatureRenderState;
  const state = value as Partial<CreatureRenderState>;
  const valid =
    state.schemaVersion === 1 &&
    isValidRevision(state.revision) &&
    state.realMemoryAccess === "off" &&
    (state.fixtureState === "empty" || state.fixtureState === "approved") &&
    state.envelope === "compact" &&
    state.bodyModule === "baseline" &&
    state.palette === "violet-mint" &&
    state.motion === "calm" &&
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
    schemaVersion: 1,
    revision: state.revision!,
    realMemoryAccess: "off",
    fixtureState: state.fixtureState!,
    envelope: "compact",
    bodyModule: "baseline",
    palette: "violet-mint",
    motion: "calm",
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
