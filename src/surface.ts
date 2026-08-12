import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

export type AppSurface = "pet" | "detail" | "browser" | "unsupported-native";

export interface SurfaceContext {
  native: boolean;
  windowLabel?: string;
  requestedSurface?: string | null;
}

export function resolveSurface({ native, windowLabel }: SurfaceContext): AppSurface {
  if (!native) return "browser";
  if (windowLabel === "pet") return "pet";
  if (windowLabel === "main") return "detail";
  return "unsupported-native";
}

export function getSurfaceContext(): SurfaceContext {
  const native = isTauri();
  const requestedSurface = new URLSearchParams(window.location.search).get("surface");
  if (!native) return { native, requestedSurface };

  try {
    return {
      native,
      requestedSurface,
      windowLabel: getCurrentWebviewWindow().label,
    };
  } catch {
    return { native, requestedSurface };
  }
}

export function getCurrentSurface(): AppSurface {
  return resolveSurface(getSurfaceContext());
}
