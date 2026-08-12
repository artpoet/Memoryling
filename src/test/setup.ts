import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

let reducedMotion = false;

if (!("PointerEvent" in window)) {
  class TestPointerEvent extends MouseEvent {
    pointerId: number;
    isPrimary: boolean;

    constructor(type: string, init: PointerEventInit = {}) {
      super(type, init);
      this.pointerId = init.pointerId ?? 0;
      this.isPrimary = init.isPrimary ?? true;
    }
  }
  Object.defineProperty(window, "PointerEvent", {
    configurable: true,
    value: TestPointerEvent,
  });
}

Object.defineProperty(window, "matchMedia", {
  configurable: true,
  value: (query: string) => ({
    matches: query === "(prefers-reduced-motion: reduce)" && reducedMotion,
    media: query,
    onchange: null,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
    addListener: () => undefined,
    removeListener: () => undefined,
    dispatchEvent: () => true,
  }),
});

export function setReducedMotionForTest(value: boolean) {
  reducedMotion = value;
}

afterEach(() => {
  cleanup();
  window.localStorage.clear();
  window.history.replaceState({}, "", "/");
  document.documentElement.removeAttribute("data-surface");
  document.documentElement.lang = "en";
  document.title = "";
  reducedMotion = false;
});
