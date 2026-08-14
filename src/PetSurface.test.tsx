import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import {
  baselineCreatureRenderState,
  sanitizeCreatureRenderState,
  sanitizePetShellState,
  type CreatureClient,
  type CreatureRenderState,
  type CreatureRevision,
  type PetShellState,
} from "./creatureClient";
import { PetSurface } from "./PetSurface";
import { setReducedMotionForTest } from "./test/setup";

const revisionOne = "1".repeat(64);
const revisionTwo = "2".repeat(64);
const revisionThree = "3".repeat(64);
const emptyState: CreatureRenderState = {
  ...baselineCreatureRenderState,
  revision: revisionOne,
};
const approvedState: CreatureRenderState = {
  ...emptyState,
  revision: revisionTwo,
  importState: "fixture-approved",
  marks: [{ id: "mark-1", style: "completion-star" }],
};
const threadApprovedState: CreatureRenderState = {
  ...approvedState,
  importState: "thread-approved",
};
const agentMemoryState: CreatureRenderState = {
  ...emptyState,
  revision: revisionThree,
  agentOperationState: "applied",
  agentActivity: "design",
  dialogue: {
    id: "dialogue.open",
    textEn: "The shape is becoming clear.",
    textZhTw: "輪廓正在變得清楚。",
    trigger: "on-open",
  },
  marks: [{ id: "mark-1", style: "completion-star" }],
};
const dismissedShell: PetShellState = {
  schemaVersion: 1,
  onboardingDismissed: true,
  alwaysOnTop: true,
};

function createClient(
  renderState: CreatureRenderState | unknown = emptyState,
  shellState: PetShellState | unknown = dismissedShell,
) {
  let renderListener: ((payload: CreatureRevision) => void) | undefined;
  let shellListener: ((payload: PetShellState) => void) | undefined;
  const renderUnlisten = vi.fn();
  const shellUnlisten = vi.fn();
  const client: CreatureClient = {
    getRenderState: vi.fn(async () => renderState as CreatureRenderState),
    advanceDialogue: vi.fn(async () => renderState as CreatureRenderState),
    getPetShellState: vi.fn(async () => shellState as PetShellState),
    showContextMenu: vi.fn(async () => undefined),
    startDragging: vi.fn(async () => undefined),
    dismissOnboarding: vi.fn(async () => dismissedShell),
    onRenderRevision: vi.fn(async (listener) => {
      renderListener = listener;
      return renderUnlisten;
    }),
    onPetShellState: vi.fn(async (listener) => {
      shellListener = listener;
      return shellUnlisten;
    }),
  };
  return {
    client,
    emitRender: (payload: CreatureRevision) => renderListener?.(payload),
    emitShell: (payload: PetShellState) => shellListener?.(payload),
    renderUnlisten,
    shellUnlisten,
  };
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (reason?: unknown) => void;
  const promise = new Promise<T>((promiseResolve, promiseReject) => {
    resolve = promiseResolve;
    reject = promiseReject;
  });
  return { promise, resolve, reject };
}

describe("pet surface", () => {
  test("shows a render-safe Agent operation, activity, and dialogue", async () => {
    const fixture = createClient(agentMemoryState);
    render(<PetSurface client={fixture.client} />);
    expect(
      await screen.findByText("Agent operation applied · app memory scanning off"),
    ).toBeInTheDocument();
    expect(await screen.findByText("The shape is becoming clear.")).toBeInTheDocument();
    expect(document.querySelector('[data-agent-activity="design"]')).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /Agent-made update/i }),
    ).toBeInTheDocument();
  });

  test("renders dialogue as a dismissible speech bubble", async () => {
    const fixture = createClient(agentMemoryState);
    render(<PetSurface client={fixture.client} />);

    const bubble = await screen.findByRole("button", {
      name: /Dismiss Memoryling's message/,
    });
    expect(bubble).toHaveClass("pet-dialogue-message");
    fireEvent.click(bubble);
    expect(screen.queryByText("The shape is becoming clear.")).not.toBeInTheDocument();
  });

  test("asks the local rule engine for another dialogue on interaction", async () => {
    const user = userEvent.setup();
    const fixture = createClient(agentMemoryState);
    vi.mocked(fixture.client.advanceDialogue).mockResolvedValueOnce({
      ...agentMemoryState,
      revision: "4".repeat(64),
      dialogue: {
        id: "dialogue.touch",
        textEn: "A careful finish is still progress.",
        textZhTw: "仔細收尾，也是一種前進。",
        trigger: "on-interact",
      },
    });
    render(<PetSurface client={fixture.client} />);
    const pet = await screen.findByRole("button", { name: /Agent-made update/i });

    await user.click(pet);
    expect(fixture.client.advanceDialogue).toHaveBeenCalledWith("on-interact");
    expect(await screen.findByText("A careful finish is still progress.")).toBeInTheDocument();
  });

  test("does not present a legacy work record as an Agent operation", async () => {
    const fixture = createClient(threadApprovedState);
    render(<PetSurface client={fixture.client} />);
    expect(
      await screen.findByText("Waiting for Agent operation · memory access off"),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", {
        name: /waiting for an Agent operation/i,
      }),
    ).toBeInTheDocument();
    expect(
      screen.getByText(/enter the activation phrase: “Memoryling, wake up”/i),
    ).toBeInTheDocument();
  });

  test("renders only the safe state and updates a completion star by revision", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);

    expect(screen.getByText("Waiting for Agent operation · memory access off")).toBeInTheDocument();
    await waitFor(() =>
      expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1),
    );
    expect(screen.queryByTestId("derived-memory-mark")).not.toBeInTheDocument();

    vi.mocked(fixture.client.getRenderState).mockResolvedValueOnce(
      approvedState,
    );
    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(
      await screen.findByTestId("derived-memory-mark"),
    ).toBeInTheDocument();

    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(fixture.client.getRenderState).toHaveBeenCalledTimes(2);
  });

  test("renders the approved seed-stage programmatic body module", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);

    const renderer = await screen.findByTestId("memoryling-seed-renderer");
    expect(renderer.tagName).toBe("svg");
    expect(renderer).toHaveAttribute("data-renderer", "procedural-svg-v8");
    expect(renderer).toHaveAttribute("data-stage", "seed");
    expect(renderer).toHaveAttribute(
      "data-body-module",
      "memory-seed-egg-v1",
    );
    expect(renderer.querySelectorAll("path").length).toBeGreaterThan(10);
    expect(renderer.querySelector("image")).not.toBeInTheDocument();

    const shell = renderer.querySelector(".seed-shell");
    expect(shell).toHaveAttribute(
      "d",
      "M120 37C108 49 85 52 68 69C50 87 42 116 43 150C44 188 69 215 102 226C113 230 127 230 138 226C171 215 196 188 197 150C198 116 190 87 172 69C155 52 132 49 120 37Z",
    );

    const innerPlates = renderer.querySelectorAll(".seed-inner-plate");
    expect(innerPlates).toHaveLength(2);
    expect(Array.from(innerPlates, (plate) => plate.getAttribute("d"))).toEqual([
      "M68 174C91 181 108 198 120 219C97 217 77 199 68 174Z",
      "M172 174C149 181 132 198 120 219C143 217 163 199 172 174Z",
    ]);
    expect(renderer.querySelectorAll(".seed-plate-thickness")).toHaveLength(0);
    const softEdgeFilter = renderer.querySelector(
      "filter[id$='-plate-shadow']",
    );
    expect(softEdgeFilter?.querySelector("feGaussianBlur")).toHaveAttribute(
      "stdDeviation",
      "2.8",
    );
    const sidePlateShadows = renderer.querySelectorAll(
      ".seed-side-plate-shadow",
    );
    expect(sidePlateShadows).toHaveLength(2);
    expect(
      Array.from(sidePlateShadows, (shadow) => shadow.getAttribute("filter")),
    ).toEqual([
      expect.stringMatching(/^url\(#.+-plate-shadow\)$/),
      expect.stringMatching(/^url\(#.+-plate-shadow\)$/),
    ]);
    expect(renderer.querySelectorAll(".seed-side-plate-rim")).toHaveLength(2);
    expect(
      Array.from(renderer.querySelectorAll(".seed-side-plate-rim"), (rim) =>
        rim.getAttribute("d"),
      ),
    ).toEqual([
      "M43 124C33 142 33 163 44 183C53 199 67 210 84 216L87 211C72 205 59 194 51 178C41 160 40 142 46 129Z",
      "M197 124C207 142 207 163 196 183C187 199 173 210 156 216L153 211C168 205 181 194 189 178C199 160 200 142 194 129Z",
    ]);
    expect(renderer.querySelectorAll(".seed-shell-facet")).toHaveLength(4);

    const pairedEyes = renderer.querySelector(".seed-eyes");
    expect(pairedEyes).not.toBeNull();
    expect(pairedEyes?.querySelectorAll(".seed-eye")).toHaveLength(2);

    const eyeShapes = pairedEyes?.querySelectorAll(
      ".seed-eye > ellipse:first-child",
    ) ?? [];
    expect(eyeShapes).toHaveLength(2);
    expect(
      Array.from(eyeShapes, (eye) => eye.getAttribute("cy")),
    ).toEqual(["120", "120"]);
    expect(
      Array.from(eyeShapes, (eye) => eye.getAttribute("cx")),
    ).toEqual(["92", "148"]);
    expect(
      Array.from(eyeShapes, (eye) => [
        eye.getAttribute("rx"),
        eye.getAttribute("ry"),
      ]),
    ).toEqual([
      ["10", "14"],
      ["10", "14"],
    ]);
    expect(
      Array.from(pairedEyes?.querySelectorAll(".seed-eye") ?? [], (eye) =>
        eye.getAttribute("transform"),
      ),
    ).toEqual([null, null]);
    const sidePlates = renderer.querySelectorAll(".seed-side-plate");
    expect(sidePlates).toHaveLength(2);
    expect(Array.from(sidePlates, (plate) => plate.getAttribute("d"))).toEqual([
      "M43 126C66 141 94 174 113 219C89 214 63 198 49 177C38 160 36 141 43 126Z",
      "M197 126C174 141 146 174 127 219C151 214 177 198 191 177C202 160 204 141 197 126Z",
    ]);
    expect(renderer.querySelectorAll(".seed-inner-plate")).toHaveLength(2);

    const accessStatus = screen.getByText("Waiting for Agent operation · memory access off");
    expect(accessStatus).toHaveClass("pet-access-status");
    expect(document.querySelector(".pet-access-badge")).not.toBeInTheDocument();
  });

  test("does not surface the retired Daily Scout state in the pet", async () => {
    const fixture = createClient({
      ...threadApprovedState,
      dailyScoutState: "ready",
    });
    render(<PetSurface client={fixture.client} />);

    expect(await screen.findByText("Waiting for Agent operation · memory access off")).toBeInTheDocument();
    expect(screen.queryByText(/found something useful/i)).not.toBeInTheDocument();
    expect(
      screen.queryByText(/Official guide|https:\/\//),
    ).not.toBeInTheDocument();
  });

  test("allows a failed revision fetch to retry when the same event is emitted", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);
    await waitFor(() =>
      expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1),
    );

    vi.mocked(fixture.client.getRenderState)
      .mockRejectedValueOnce(new Error("private native detail"))
      .mockResolvedValueOnce(approvedState);
    act(() => fixture.emitRender({ revision: revisionTwo }));
    await waitFor(() =>
      expect(fixture.client.getRenderState).toHaveBeenCalledTimes(2),
    );
    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(
      await screen.findByTestId("derived-memory-mark"),
    ).toBeInTheDocument();
  });

  test("does not let a slow stale response roll back a newer revision", async () => {
    const fixture = createClient();
    const stale = deferred<CreatureRenderState>();
    const newest = deferred<CreatureRenderState>();
    vi.mocked(fixture.client.getRenderState)
      .mockResolvedValueOnce(emptyState)
      .mockReturnValueOnce(stale.promise)
      .mockReturnValueOnce(newest.promise);
    render(<PetSurface client={fixture.client} />);
    await waitFor(() =>
      expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1),
    );

    act(() => fixture.emitRender({ revision: revisionTwo }));
    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(fixture.client.getRenderState).toHaveBeenCalledTimes(2);
    act(() => fixture.emitRender({ revision: revisionThree }));
    newest.resolve({ ...approvedState, revision: revisionThree });
    expect(
      await screen.findByTestId("derived-memory-mark"),
    ).toBeInTheDocument();
    stale.resolve({ ...emptyState, revision: revisionTwo });
    await act(async () => {
      await stale.promise;
    });
    expect(screen.getByTestId("derived-memory-mark")).toBeInTheDocument();
  });

  test("ignores invalid events and fail-closes unknown render payloads", async () => {
    const fixture = createClient({
      ...approvedState,
      bodyModule: "payload containing private text",
    });
    render(<PetSurface client={fixture.client} />);
    await waitFor(() =>
      expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1),
    );
    expect(screen.queryByText(/private text/)).not.toBeInTheDocument();
    expect(screen.queryByTestId("derived-memory-mark")).not.toBeInTheDocument();
    act(() => fixture.emitRender({ revision: "invalid" }));
    expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1);
  });

  test("rebuilds a valid safe DTO from an explicit field allowlist", () => {
    const tainted = {
      ...approvedState,
      memoryText: "must not survive",
      marks: [
        {
          id: "mark-1",
          style: "completion-star",
          locator: "private://must-not-survive",
        },
      ],
    };
    const sanitized = sanitizeCreatureRenderState(tainted);
    expect(sanitized.marks).toEqual([
      { id: "mark-1", style: "completion-star" },
    ]);
    expect(sanitized).not.toHaveProperty("memoryText");
    expect(sanitized.marks[0]).not.toHaveProperty("locator");

    const safeShell = sanitizePetShellState({
      ...dismissedShell,
      sourcePath: "private://must-not-survive",
    });
    expect(safeShell).toEqual(dismissedShell);
    expect(safeShell).not.toHaveProperty("sourcePath");
  });

  test("routes pointer and keyboard menu triggers and keeps click local", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);
    const pet = screen.getByRole("button", {
      name: /waiting for an Agent operation/i,
    });

    await user.click(pet);
    expect(
      screen.getByText("Memoryling gives a quiet blink."),
    ).toBeInTheDocument();
    expect(fixture.client.showContextMenu).not.toHaveBeenCalled();
    fireEvent.contextMenu(pet);
    expect(fixture.client.showContextMenu).toHaveBeenCalledWith("pointer");

    for (const key of ["Enter", " ", "ContextMenu"])
      fireEvent.keyDown(pet, { key });
    fireEvent.keyDown(pet, { key: "F10", shiftKey: true });
    fireEvent.keyDown(pet, { key: "F10" });
    expect(fixture.client.showContextMenu).toHaveBeenCalledTimes(5);
    expect(fixture.client.showContextMenu).toHaveBeenLastCalledWith("keyboard");
  });

  test("starts dragging only after the threshold and does not poison the next click", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);
    const pet = screen.getByRole("button", {
      name: /waiting for an Agent operation/i,
    });

    fireEvent.pointerDown(pet, {
      pointerId: 1,
      button: 0,
      isPrimary: true,
      clientX: 10,
      clientY: 10,
    });
    fireEvent.pointerMove(pet, {
      pointerId: 1,
      isPrimary: true,
      clientX: 13,
      clientY: 13,
    });
    expect(fixture.client.startDragging).not.toHaveBeenCalled();
    fireEvent.pointerMove(pet, {
      pointerId: 1,
      isPrimary: true,
      clientX: 20,
      clientY: 20,
    });
    expect(fixture.client.startDragging).toHaveBeenCalledTimes(1);
    fireEvent.pointerUp(pet, { pointerId: 1 });
    await act(async () => {
      await new Promise((resolve) => window.setTimeout(resolve, 0));
    });
    fireEvent.click(pet, { detail: 1 });
    expect(
      screen.getByText("Memoryling gives a quiet blink."),
    ).toBeInTheDocument();
  });

  test("recovers after native dragging absorbs pointer-up", async () => {
    const drag = deferred<void>();
    const fixture = createClient();
    vi.mocked(fixture.client.startDragging).mockReturnValueOnce(drag.promise);
    render(<PetSurface client={fixture.client} />);
    const pet = screen.getByRole("button", {
      name: /waiting for an Agent operation/i,
    });

    fireEvent.pointerDown(pet, {
      pointerId: 7,
      button: 0,
      isPrimary: true,
      clientX: 10,
      clientY: 10,
    });
    fireEvent.pointerMove(pet, {
      pointerId: 7,
      isPrimary: true,
      clientX: 20,
      clientY: 20,
    });
    fireEvent.click(pet, { detail: 1 });
    expect(
      screen.queryByText("Memoryling gives a quiet blink."),
    ).not.toBeInTheDocument();

    drag.resolve();
    await act(async () => {
      await drag.promise;
      await new Promise((resolve) => window.setTimeout(resolve, 0));
    });
    fireEvent.click(pet, { detail: 1 });
    expect(
      screen.getByText("Memoryling gives a quiet blink."),
    ).toBeInTheDocument();
  });

  test("copies the Chinese activation phrase, persists onboarding dismissal, and honors reduced motion", async () => {
    window.localStorage.setItem("memoryling:locale", "zh-TW");
    setReducedMotionForTest(true);
    const clipboard = { writeText: vi.fn(async () => undefined) };
    const fixture = createClient(emptyState, {
      ...dismissedShell,
      onboardingDismissed: false,
    });
    render(<PetSurface client={fixture.client} clipboard={clipboard} />);

    expect(await screen.findByText("等待 Agent 運作 · 記憶存取關閉")).toBeInTheDocument();
    expect(
      await screen.findByText("按右鍵，再選擇開啟 Memoryling。"),
    ).toBeInTheDocument();
    expect(
      screen.getByText("回到你目前工作的 Agent 專案，輸入發動語：「醒來吧我的寵物」。"),
    ).toBeInTheDocument();
    await userEvent.click(
      screen.getByRole("button", { name: "複製「醒來吧我的寵物」" }),
    );
    expect(clipboard.writeText).toHaveBeenCalledWith("醒來吧我的寵物");
    expect(
      screen.getByRole("button", { name: "已複製「醒來吧我的寵物」" }),
    ).toBeInTheDocument();
    expect(screen.getByTestId("pet-surface")).toHaveAttribute(
      "data-motion",
      "reduced",
    );
    await userEvent.click(screen.getByRole("button", { name: "知道了" }));
    expect(fixture.client.dismissOnboarding).toHaveBeenCalledTimes(1);
    await waitFor(() =>
      expect(screen.queryByText("Memoryling 已打開")).not.toBeInTheDocument(),
    );
    expect(
      screen.getByText("回到你目前工作的 Agent 專案，輸入發動語：「醒來吧我的寵物」。"),
    ).toBeInTheDocument();
  });

  test("copies the English activation phrase from the first-run guide", async () => {
    const clipboard = { writeText: vi.fn(async () => undefined) };
    const fixture = createClient(emptyState, {
      ...dismissedShell,
      onboardingDismissed: false,
    });
    render(<PetSurface client={fixture.client} clipboard={clipboard} />);

    await userEvent.click(
      await screen.findByRole("button", {
        name: "Copy “Memoryling, wake up”",
      }),
    );
    expect(clipboard.writeText).toHaveBeenCalledWith("Memoryling, wake up");
    expect(
      screen.getByRole("button", {
        name: "Copied “Memoryling, wake up”",
      }),
    ).toBeInTheDocument();
  });

  test("keeps the activation phrase visible when clipboard copy fails", async () => {
    window.localStorage.setItem("memoryling:locale", "zh-TW");
    const fixture = createClient(emptyState, {
      ...dismissedShell,
      onboardingDismissed: false,
    });
    render(<PetSurface client={fixture.client} clipboard={null} />);

    await userEvent.click(
      await screen.findByRole("button", { name: "複製「醒來吧我的寵物」" }),
    );
    expect(
      screen.getByRole("button", { name: "複製失敗，請再試一次" }),
    ).toBeInTheDocument();
    expect(
      screen.getByText("回到你目前工作的 Agent 專案，輸入發動語：「醒來吧我的寵物」。"),
    ).toBeInTheDocument();
  });

  test("cleans up both native listeners", async () => {
    const fixture = createClient();
    const view = render(<PetSurface client={fixture.client} />);
    await waitFor(() =>
      expect(fixture.client.onPetShellState).toHaveBeenCalled(),
    );
    view.unmount();
    expect(fixture.renderUnlisten).toHaveBeenCalledTimes(1);
    expect(fixture.shellUnlisten).toHaveBeenCalledTimes(1);
  });

  test("still fetches safely when one listener registration fails", async () => {
    const fixture = createClient();
    fixture.client.onPetShellState = vi.fn(async () => {
      throw new Error("listener unavailable");
    });
    const view = render(<PetSurface client={fixture.client} />);
    await waitFor(() =>
      expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1),
    );
    expect(screen.getByText("Waiting for Agent operation · memory access off")).toBeInTheDocument();
    view.unmount();
    expect(fixture.renderUnlisten).toHaveBeenCalledTimes(1);
  });

  test("cleans up a late listener even when the other registration fails", async () => {
    const late = deferred<() => void>();
    const lateUnlisten = vi.fn();
    const fixture = createClient();
    fixture.client.onRenderRevision = vi.fn(() => late.promise);
    fixture.client.onPetShellState = vi.fn(async () => {
      throw new Error("listener unavailable");
    });
    const view = render(<PetSurface client={fixture.client} />);
    await waitFor(() =>
      expect(fixture.client.onRenderRevision).toHaveBeenCalled(),
    );
    view.unmount();
    late.resolve(lateUnlisten);
    await waitFor(() => expect(lateUnlisten).toHaveBeenCalledTimes(1));
  });
});
