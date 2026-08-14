import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import { DetailSurface } from "./App";
import type {
  CreatureRevision,
  DetailEventClient,
  DetailShellClient,
} from "./creatureClient";
import {
  emptyMemoryState,
  type MemoryClient,
  type MemoryState,
} from "./memoryClient";

const operationState: MemoryState = {
  ...emptyMemoryState,
  agentOperation: {
    state: "applied",
    appliedAt: "2026-08-13T10:00:00Z",
    activity: "building",
    dialogueCount: 5,
  },
};

function createMemoryClient(state: MemoryState = emptyMemoryState): MemoryClient {
  return {
    available: true,
    listSources: vi.fn(async () => []),
    getState: vi.fn(async () => state),
    clearAgentOperation: vi.fn(async () => emptyMemoryState),
    previewSource: vi.fn(async () => {
      throw new Error("legacy connector is not part of this surface");
    }),
    listCodexThreads: vi.fn(async () => ({ catalogId: "unused", candidates: [] })),
    previewCodexThread: vi.fn(async () => {
      throw new Error("legacy connector is not part of this surface");
    }),
    cancelPreview: vi.fn(async () => undefined),
    approveImport: vi.fn(async () => state),
    forgetSource: vi.fn(async () => state),
    syncCodexMemories: vi.fn(async () => state),
  };
}

function createDetailEvents() {
  let revisionListener: ((payload: CreatureRevision) => void) | undefined;
  let resetListener: (() => void) | undefined;
  const client: DetailEventClient = {
    onRenderRevision: vi.fn(async (listener) => {
      revisionListener = listener;
      return vi.fn();
    }),
    onDetailReset: vi.fn(async (listener) => {
      resetListener = listener;
      return vi.fn();
    }),
  };
  return {
    client,
    emitRevision: (revision = "a".repeat(64)) => revisionListener?.({ revision }),
    emitReset: () => resetListener?.(),
  };
}

const detailShell: DetailShellClient = {
  resetOnboarding: vi.fn(async () => ({
    schemaVersion: 1 as const,
    onboardingDismissed: false,
    alwaysOnTop: true,
  })),
};

describe("Agent-operated Memoryling detail surface", () => {
  test("renders the applied operation without exposing a direct memory connector", async () => {
    const user = userEvent.setup();
    const memoryClient = createMemoryClient(operationState);
    render(
      <DetailSurface
        browserPreview={false}
        detailEvents={createDetailEvents().client}
        detailShell={detailShell}
        memoryClient={memoryClient}
      />,
    );

    expect(await screen.findByText("Agent operation applied · local pet rules active")).toBeInTheDocument();
    expect(screen.getByText("Latest Agent operation applied")).toBeInTheDocument();
    expect(screen.getByText(/5 dialogue cards are ready/)).toBeInTheDocument();
    expect(document.querySelector('[data-agent-activity="building"]')).toBeInTheDocument();
    expect(screen.getByText("An Agent operation shaped the pet")).toBeInTheDocument();
    expect(screen.queryByRole("radio", { name: /Agent memories/i })).not.toBeInTheDocument();
    expect(screen.queryByText(/API key/i)).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Clear this pet update" }));
    expect(memoryClient.clearAgentOperation).toHaveBeenCalledTimes(1);
    expect(await screen.findByText("Memoryling is open and waiting")).toBeInTheDocument();
  });

  test("keeps browser preview honest and performs no memory read", () => {
    const memoryClient = createMemoryClient();
    memoryClient.available = false;
    render(
      <DetailSurface
        browserPreview
        detailEvents={createDetailEvents().client}
        detailShell={detailShell}
        memoryClient={memoryClient}
      />,
    );

    expect(screen.getByTestId("browser-shell-boundary")).toBeInTheDocument();
    expect(screen.getByText("Open Memoryling, then say “Memoryling, wake up” in your agent project")).toBeInTheDocument();
    expect(screen.getByText(/performs no AI or memory read/i)).toBeInTheDocument();
    expect(memoryClient.getState).not.toHaveBeenCalled();
  });

  test("keeps the Agent-operated explanation in Traditional Chinese", async () => {
    const user = userEvent.setup();
    render(
      <DetailSurface
        browserPreview
        detailEvents={createDetailEvents().client}
        detailShell={detailShell}
        memoryClient={{ ...createMemoryClient(), available: false }}
      />,
    );

    await user.click(screen.getByRole("button", { name: "繁中" }));
    expect(screen.getByText("先打開 Memoryling，再回 Agent 專案說：「醒來吧我的寵物」")).toBeInTheDocument();
    expect(screen.getByText(/這個頁面不會自行執行 AI 或讀取記憶/)).toBeInTheDocument();
  });

  test("refreshes the full operation summary after a native revision", async () => {
    const memoryClient = createMemoryClient();
    vi.mocked(memoryClient.getState)
      .mockResolvedValueOnce(emptyMemoryState)
      .mockResolvedValueOnce(operationState);
    const events = createDetailEvents();
    render(
      <DetailSurface
        browserPreview={false}
        detailEvents={events.client}
        detailShell={detailShell}
        memoryClient={memoryClient}
      />,
    );
    await waitFor(() => expect(memoryClient.getState).toHaveBeenCalledTimes(1));

    act(() => events.emitRevision());
    expect(await screen.findByText("Latest Agent operation applied")).toBeInTheDocument();
    expect(memoryClient.getState).toHaveBeenCalledTimes(2);
  });

  test("resets the pet guide with honest success reporting", async () => {
    const user = userEvent.setup();
    const resetOnboarding = vi.fn(async () => ({
      schemaVersion: 1 as const,
      onboardingDismissed: false,
      alwaysOnTop: true,
    }));
    render(
      <DetailSurface
        browserPreview={false}
        detailEvents={createDetailEvents().client}
        detailShell={{ resetOnboarding }}
        memoryClient={createMemoryClient()}
      />,
    );

    await user.click(screen.getByRole("button", { name: "Show pet guide again" }));
    expect(resetOnboarding).toHaveBeenCalledTimes(1);
    expect(screen.getByText(/guide will appear the next time/)).toBeInTheDocument();
  });
});
