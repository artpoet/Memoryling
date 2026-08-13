import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
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
  test("reports one active work record without claiming durable-memory access", async () => {
    const fixture = createClient(threadApprovedState);
    render(<PetSurface client={fixture.client} />);
    expect(
      await screen.findByText("1 Codex work record active · durable memory off"),
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", {
        name: /One Codex work record is active; durable memory access is off/,
      }),
    ).toBeInTheDocument();
  });

  test("renders only the safe state and updates a completion star by revision", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);

    expect(screen.getByText("Memory access off")).toBeInTheDocument();
    await waitFor(() => expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1));
    expect(screen.queryByTestId("derived-memory-mark")).not.toBeInTheDocument();

    vi.mocked(fixture.client.getRenderState).mockResolvedValueOnce(approvedState);
    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();

    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(fixture.client.getRenderState).toHaveBeenCalledTimes(2);
  });

  test("shows only a neutral pet notice when a daily insight is ready", async () => {
    const fixture = createClient({
      ...threadApprovedState,
      dailyScoutState: "ready",
    });
    render(<PetSurface client={fixture.client} />);

    expect(
      await screen.findByText(
        "I found something useful for you. Open Memoryling to see it.",
      ),
    ).toBeInTheDocument();
    expect(screen.queryByText(/Official guide|https:\/\//)).not.toBeInTheDocument();
  });

  test("allows a failed revision fetch to retry when the same event is emitted", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);
    await waitFor(() => expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1));

    vi.mocked(fixture.client.getRenderState)
      .mockRejectedValueOnce(new Error("private native detail"))
      .mockResolvedValueOnce(approvedState);
    act(() => fixture.emitRender({ revision: revisionTwo }));
    await waitFor(() => expect(fixture.client.getRenderState).toHaveBeenCalledTimes(2));
    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
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
    await waitFor(() => expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1));

    act(() => fixture.emitRender({ revision: revisionTwo }));
    act(() => fixture.emitRender({ revision: revisionTwo }));
    expect(fixture.client.getRenderState).toHaveBeenCalledTimes(2);
    act(() => fixture.emitRender({ revision: revisionThree }));
    newest.resolve({ ...approvedState, revision: revisionThree });
    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
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
    await waitFor(() => expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1));
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
    expect(sanitized.marks).toEqual([{ id: "mark-1", style: "completion-star" }]);
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
    const pet = screen.getByRole("button", { name: /Memoryling\. Memory access is off/ });

    await user.click(pet);
    expect(screen.getByText("Memoryling gives a quiet blink.")).toBeInTheDocument();
    expect(fixture.client.showContextMenu).not.toHaveBeenCalled();
    fireEvent.contextMenu(pet);
    expect(fixture.client.showContextMenu).toHaveBeenCalledWith("pointer");

    for (const key of ["Enter", " ", "ContextMenu"]) fireEvent.keyDown(pet, { key });
    fireEvent.keyDown(pet, { key: "F10", shiftKey: true });
    fireEvent.keyDown(pet, { key: "F10" });
    expect(fixture.client.showContextMenu).toHaveBeenCalledTimes(5);
    expect(fixture.client.showContextMenu).toHaveBeenLastCalledWith("keyboard");
  });

  test("starts dragging only after the threshold and does not poison the next click", async () => {
    const fixture = createClient();
    render(<PetSurface client={fixture.client} />);
    const pet = screen.getByRole("button", { name: /Memoryling\. Memory access is off/ });

    fireEvent.pointerDown(pet, {
      pointerId: 1,
      button: 0,
      isPrimary: true,
      clientX: 10,
      clientY: 10,
    });
    fireEvent.pointerMove(pet, { pointerId: 1, isPrimary: true, clientX: 13, clientY: 13 });
    expect(fixture.client.startDragging).not.toHaveBeenCalled();
    fireEvent.pointerMove(pet, { pointerId: 1, isPrimary: true, clientX: 20, clientY: 20 });
    expect(fixture.client.startDragging).toHaveBeenCalledTimes(1);
    fireEvent.pointerUp(pet, { pointerId: 1 });
    await act(async () => {
      await new Promise((resolve) => window.setTimeout(resolve, 0));
    });
    fireEvent.click(pet, { detail: 1 });
    expect(screen.getByText("Memoryling gives a quiet blink.")).toBeInTheDocument();
  });

  test("recovers after native dragging absorbs pointer-up", async () => {
    const drag = deferred<void>();
    const fixture = createClient();
    vi.mocked(fixture.client.startDragging).mockReturnValueOnce(drag.promise);
    render(<PetSurface client={fixture.client} />);
    const pet = screen.getByRole("button", { name: /Memoryling\. Memory access is off/ });

    fireEvent.pointerDown(pet, {
      pointerId: 7,
      button: 0,
      isPrimary: true,
      clientX: 10,
      clientY: 10,
    });
    fireEvent.pointerMove(pet, { pointerId: 7, isPrimary: true, clientX: 20, clientY: 20 });
    fireEvent.click(pet, { detail: 1 });
    expect(screen.queryByText("Memoryling gives a quiet blink.")).not.toBeInTheDocument();

    drag.resolve();
    await act(async () => {
      await drag.promise;
      await new Promise((resolve) => window.setTimeout(resolve, 0));
    });
    fireEvent.click(pet, { detail: 1 });
    expect(screen.getByText("Memoryling gives a quiet blink.")).toBeInTheDocument();
  });

  test("persists onboarding dismissal, supports Chinese, and honors reduced motion", async () => {
    window.localStorage.setItem("memoryling:locale", "zh-TW");
    setReducedMotionForTest(true);
    const fixture = createClient(emptyState, {
      ...dismissedShell,
      onboardingDismissed: false,
    });
    render(<PetSurface client={fixture.client} />);

    expect(await screen.findByText("記憶存取關閉")).toBeInTheDocument();
    expect(await screen.findByText("按右鍵，再選擇開啟 Memoryling。")).toBeInTheDocument();
    expect(screen.getByTestId("pet-surface")).toHaveAttribute("data-motion", "reduced");
    await userEvent.click(screen.getByRole("button", { name: "知道了" }));
    expect(fixture.client.dismissOnboarding).toHaveBeenCalledTimes(1);
    await waitFor(() =>
      expect(screen.queryByText("認識你的 Memoryling")).not.toBeInTheDocument(),
    );
  });

  test("cleans up both native listeners", async () => {
    const fixture = createClient();
    const view = render(<PetSurface client={fixture.client} />);
    await waitFor(() => expect(fixture.client.onPetShellState).toHaveBeenCalled());
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
    await waitFor(() => expect(fixture.client.getRenderState).toHaveBeenCalledTimes(1));
    expect(screen.getByText("Memory access off")).toBeInTheDocument();
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
    await waitFor(() => expect(fixture.client.onRenderRevision).toHaveBeenCalled());
    view.unmount();
    late.resolve(lateUnlisten);
    await waitFor(() => expect(lateUnlisten).toHaveBeenCalledTimes(1));
  });
});
