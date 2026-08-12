import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import { App } from "./App";
import type { DetailEventClient, DetailShellClient } from "./creatureClient";
import {
  emptyMemoryState,
  type ImportPreview,
  type MemoryClient,
  type MemoryState,
  type SourceOption,
} from "./memoryClient";

const source: SourceOption = {
  id: "codex.synthetic.first-memory",
  adapterId: "codex-durable-memory",
  adapterVersion: 1,
  displayName: "Codex · First memory fixture",
  locator: "resource://fixtures/codex-first-memory-v1.json",
  fixtureOnly: true,
};

const contentHash = "a".repeat(64);
const consentScopeHash = "b".repeat(64);

const preview: ImportPreview = {
  previewId: "preview_123",
  source,
  recordCount: 1,
  timeRange: {
    start: "2026-08-10T08:15:00Z",
    end: "2026-08-10T08:15:00Z",
  },
  records: [
    {
      id: "synthetic-memory-001",
      sourceTimestamp: "2026-08-10T08:15:00Z",
      kind: "completion",
      textPreview:
        "Shipped a local-first creature whose changes can always explain their source.",
      characterCount: 79,
      contentHash,
    },
  ],
  accessScope: {
    readOnly: true,
    sourceWriteAccess: false,
    networkAccess: false,
    arbitraryPathAccess: false,
  },
  consentScope: {
    schemaVersion: 1,
    revision: 1,
    sourceId: source.id,
    adapterId: source.adapterId,
    adapterVersion: source.adapterVersion,
    dataCategories: ["synthetic-completion"],
    purposes: ["local-creature-derivation"],
    readOnly: true,
  },
  consentScopeHash,
};

const approvedState: MemoryState = {
  storeSchemaVersion: 2,
  sourceCount: 1,
  eventCount: 1,
  signalCount: 1,
  marks: [
    {
      id: "effect_123",
      style: "completion-star",
      signalType: "completion",
      confidence: 1,
      derivationVersion: 1,
      explanationKey: "approved_completion_created_star",
      lineage: [
        {
          memoryEventId: "memory_123",
          memoryEventSchemaVersion: 1,
          sourceId: source.id,
          sourceLabel: source.displayName,
          adapterId: source.adapterId,
          adapterVersion: source.adapterVersion,
          sourceRecordId: "synthetic-memory-001",
          sourceTimestamp: "2026-08-10T08:15:00Z",
          memoryText:
            "Shipped a local-first creature whose changes can always explain their source.",
          contentRedacted: false,
          characterCount: 79,
          contentHash,
        },
      ],
    },
  ],
};

const threadSource: SourceOption = {
  id: "source_work_1",
  adapterId: "codex-app-server-thread",
  adapterVersion: 1,
  displayName: "Codex work record · Aug 12",
  locator: "opaque://source_work_1",
  fixtureOnly: false,
};

const threadConsentScopeHash = "c".repeat(64);
const threadPreview: ImportPreview = {
  previewId: "preview_work_1",
  source: threadSource,
  recordCount: 1,
  timeRange: {
    start: "2026-08-12T07:30:00Z",
    end: "2026-08-12T07:30:00Z",
  },
  records: [
    {
      id: "record_work_1",
      sourceTimestamp: "2026-08-12T07:30:00Z",
      kind: "completion",
      characterCount: 428,
      contentHash: contentHash,
    },
  ],
  accessScope: {
    readOnly: true,
    sourceWriteAccess: false,
    networkAccess: false,
    arbitraryPathAccess: false,
  },
  consentScope: {
    schemaVersion: 1,
    revision: 1,
    sourceId: threadSource.id,
    adapterId: "codex-app-server-thread",
    adapterVersion: 1,
    dataCategories: ["user-confirmed-completion"],
    purposes: ["local-creature-derivation"],
    readOnly: true,
  },
  consentScopeHash: threadConsentScopeHash,
};

const approvedThreadState: MemoryState = {
  ...approvedState,
  marks: [
    {
      ...approvedState.marks[0],
      lineage: [
        {
          ...approvedState.marks[0].lineage[0],
          sourceId: threadSource.id,
          sourceLabel: threadSource.displayName,
          adapterId: threadSource.adapterId,
          sourceRecordId: "raw-thread-id-must-not-render",
          memoryText: "PRIVATE FINAL ANSWER MUST NOT RENDER",
          contentRedacted: true,
          characterCount: 428,
          consentScopeHash: threadConsentScopeHash,
          consentRevision: 1,
        },
      ],
    },
  ],
};

function createClient(initialState: MemoryState = emptyMemoryState) {
  const approveImport = vi.fn(async () => approvedState);
  const forgetSource = vi.fn(async () => emptyMemoryState);
  const cancelPreview = vi.fn(async () => undefined);
  const client: MemoryClient = {
    available: true,
    listSources: vi.fn(async () => [source]),
    getState: vi.fn(async () => initialState),
    previewSource: vi.fn(async () => preview),
    listCodexThreads: vi.fn(async () => ({ catalogId: "catalog_1", candidates: [] })),
    previewCodexThread: vi.fn(async () => preview),
    cancelPreview,
    approveImport,
    forgetSource,
  };
  return { client, approveImport, forgetSource, cancelPreview };
}

function createDetailEvents() {
  let revisionListener: (() => void) | undefined;
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
    emitRevision: () => revisionListener?.(),
    emitReset: () => resetListener?.(),
  };
}

const quietDetailEvents: DetailEventClient = {
  onRenderRevision: vi.fn(async () => vi.fn()),
  onDetailReset: vi.fn(async () => vi.fn()),
};

const quietDetailShell: DetailShellClient = {
  resetOnboarding: vi.fn(async () => ({
    schemaVersion: 1 as const,
    onboardingDismissed: false,
    alwaysOnTop: true,
  })),
};

describe("First real memory vertical slice", () => {
  test("browses one Codex work record, redacts content, binds exact consent, and forgets it", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    vi.mocked(fixture.client.listCodexThreads).mockResolvedValueOnce({
      catalogId: "catalog_safe_1",
      candidates: [
        {
          candidateId: "candidate_opaque_1",
          displayName: "Recent completed work · Aug 12",
          updatedAt: "2026-08-12T07:30:00Z",
          sourceKind: "Codex work record",
        },
      ],
    });
    vi.mocked(fixture.client.previewCodexThread).mockResolvedValueOnce(threadPreview);
    fixture.approveImport.mockResolvedValueOnce(approvedThreadState);

    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={fixture.client}
      />,
    );

    await user.click(
      await screen.findByRole("button", { name: "Browse local Codex work records" }),
    );
    expect(fixture.client.listCodexThreads).toHaveBeenCalledTimes(1);
    expect(screen.getByText("EXPERIMENTAL")).toBeInTheDocument();
    expect(
      screen.getByText(/without titles, summaries, paths, raw IDs, or transcript content/i),
    ).toBeInTheDocument();

    await user.click(
      screen.getByRole("radio", { name: /Recent completed work · Aug 12/ }),
    );
    await user.click(screen.getByRole("button", { name: "Review this record" }));
    expect(fixture.client.previewCodexThread).toHaveBeenCalledWith(
      "catalog_safe_1",
      "candidate_opaque_1",
    );
    expect(await screen.findByText("CONTENT HIDDEN FROM THE WEBVIEW")).toBeInTheDocument();
    expect(screen.getByText(/Category: completion · Characters: 428/)).toBeInTheDocument();
    expect(screen.getByText("user-confirmed-completion")).toBeInTheDocument();
    expect(screen.getByText("local-creature-derivation")).toBeInTheDocument();
    expect(screen.getByText(threadConsentScopeHash)).toBeInTheDocument();
    expect(screen.queryByText(/candidate_opaque_1|raw-thread-id|PRIVATE FINAL/)).not.toBeInTheDocument();

    await user.click(
      screen.getByRole("checkbox", {
        name: /I confirm this work record represents a completed outcome/,
      }),
    );
    await user.click(
      screen.getByRole("button", { name: "Approve & store 1 record locally" }),
    );
    expect(fixture.approveImport).toHaveBeenCalledWith({
      previewId: threadPreview.previewId,
      sourceId: threadSource.id,
      selectedRecordIds: ["record_work_1"],
      consentScopeHash: threadConsentScopeHash,
    });
    expect(
      await screen.findByText(
        "1 Codex work record active · durable memory access is off",
      ),
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Why did this happen?" }));
    expect(screen.getByText("Stored content hidden")).toBeInTheDocument();
    expect(screen.queryByText(/PRIVATE FINAL|raw-thread-id/)).not.toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Forget this source" }));
    await user.click(
      screen.getByRole("checkbox", {
        name: /I understand that the local imported record/,
      }),
    );
    await user.click(
      screen.getByRole("button", { name: "Forget source and remove mark" }),
    );
    expect(fixture.forgetSource).toHaveBeenCalledWith(threadSource.id);
    expect(
      await screen.findByText("Forgotten completely. Source access is off again."),
    ).toBeInTheDocument();
  });

  test("cancels a work-record catalog without reading a thread", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    vi.mocked(fixture.client.listCodexThreads).mockResolvedValueOnce({
      catalogId: "catalog_cancel",
      candidates: [
        {
          candidateId: "candidate_cancel",
          displayName: "Work record · Aug 11",
          updatedAt: "2026-08-11T04:00:00Z",
          sourceKind: "Codex work record",
        },
      ],
    });
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={fixture.client}
      />,
    );
    await user.click(
      await screen.findByRole("button", { name: "Browse local Codex work records" }),
    );
    expect(await screen.findByText("Work record · Aug 11")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Cancel browse" }));
    expect(screen.queryByText("Work record · Aug 11")).not.toBeInTheDocument();
    expect(fixture.cancelPreview).toHaveBeenCalledWith("catalog_cancel");
    expect(fixture.client.previewCodexThread).not.toHaveBeenCalled();
  });

  test("fails closed when the work-record catalog cannot be listed", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    vi.mocked(fixture.client.listCodexThreads).mockRejectedValueOnce(
      new Error("C:\\private\\codex\\state.db"),
    );
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={fixture.client}
      />,
    );
    await user.click(
      await screen.findByRole("button", { name: "Browse local Codex work records" }),
    );
    expect(
      await screen.findByText(
        "The local operation did not complete. No private error details were shown and no UI state was approved.",
      ),
    ).toBeInTheDocument();
    expect(screen.queryByText(/C:\\private|state\.db/i)).not.toBeInTheDocument();
    expect(fixture.client.previewCodexThread).not.toHaveBeenCalled();
  });

  test("moves from off to preview, persisted lineage, and complete forgetting", async () => {
    const user = userEvent.setup();
    const { client, approveImport, forgetSource } = createClient();
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={client}
      />,
    );

    expect(
      screen.getByRole("link", { name: "Memoryling home" }).querySelector("img"),
    ).toHaveAttribute("src", expect.stringContaining("memoryling-icon"));
    expect(screen.queryByTestId("derived-memory-mark")).not.toBeInTheDocument();
    expect(
      screen.getByText("Memory access is off · no approved sources"),
    ).toBeInTheDocument();

    const sourceRadio = await screen.findByRole("radio", {
      name: /Codex · First memory fixture/,
    });
    await user.click(sourceRadio);
    await user.click(screen.getByRole("button", { name: "Preview synthetic fixture" }));

    expect(
      await screen.findByText(
        "Shipped a local-first creature whose changes can always explain their source.",
      ),
    ).toBeInTheDocument();
    expect(approveImport).not.toHaveBeenCalled();
    expect(screen.queryByTestId("derived-memory-mark")).not.toBeInTheDocument();

    await user.click(
      screen.getByRole("checkbox", {
        name: /I approve storing the selected synthetic record/,
      }),
    );
    await user.click(
      screen.getByRole("button", { name: "Approve & store 1 record locally" }),
    );

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(approveImport).toHaveBeenCalledWith({
      previewId: preview.previewId,
      sourceId: source.id,
      selectedRecordIds: ["synthetic-memory-001"],
      consentScopeHash,
    });
    expect(
      screen.getByText("Fixture pilot active · real memory access is off"),
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Why did this happen?" }));
    expect(screen.getByText("Privacy-safe lineage")).toBeInTheDocument();
    expect(screen.getByText("Normalized memory event")).toBeInTheDocument();
    expect(screen.getByText("Creature world effect")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Forget this source" }));
    await user.click(
      screen.getByRole("checkbox", {
        name: /I understand that the local imported record/,
      }),
    );
    await user.click(
      screen.getByRole("button", { name: "Forget source and remove mark" }),
    );

    await waitFor(() => {
      expect(screen.queryByTestId("derived-memory-mark")).not.toBeInTheDocument();
    });
    expect(forgetSource).toHaveBeenCalledWith(source.id);
    expect(
      screen.getByText("Memory access is off · no approved sources"),
    ).toBeInTheDocument();
    expect(
      screen.getByText("Forgotten completely. Source access is off again."),
    ).toBeInTheDocument();
  });

  test("restores the persisted mark after a desktop restart", async () => {
    const { client } = createClient(approvedState);
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={client}
      />,
    );

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(screen.getByText("One approved synthetic memory left a completion star")).toBeInTheDocument();
  });

  test("keeps the honest off state in a browser and preserves bilingual parity", async () => {
    const user = userEvent.setup();
    const unavailableClient: MemoryClient = {
      available: false,
      listSources: vi.fn(),
      getState: vi.fn(),
      previewSource: vi.fn(),
      listCodexThreads: vi.fn(),
      previewCodexThread: vi.fn(),
      cancelPreview: vi.fn(),
      approveImport: vi.fn(),
      forgetSource: vi.fn(),
    };
    const resetOnboarding = vi.fn();
    render(
      <App
        browserPreview
        detailEvents={quietDetailEvents}
        detailShell={{ resetOnboarding }}
        memoryClient={unavailableClient}
      />,
    );

    expect(screen.getByText("Desktop runtime required")).toBeInTheDocument();
    expect(
      screen.getByText("Browser preview · memory access is off"),
    ).toBeInTheDocument();
    expect(screen.getByText("Floating pet is available in the Windows desktop app")).toBeInTheDocument();
    expect(screen.queryByTestId("pet-surface")).not.toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Show pet guide again" })).not.toBeInTheDocument();
    expect(resetOnboarding).not.toHaveBeenCalled();
    expect(unavailableClient.listCodexThreads).not.toHaveBeenCalled();
    expect(unavailableClient.previewCodexThread).not.toHaveBeenCalled();

    await user.click(screen.getByRole("button", { name: "繁中" }));
    expect(screen.getByText("從核准工作，到可解釋的印記")).toBeInTheDocument();
    expect(screen.getByText("需要桌面版執行環境")).toBeInTheDocument();
    expect(screen.getByText("瀏覽器預覽 · 記憶存取關閉")).toBeInTheDocument();
  });

  test("renders a defensive fallback for an invalid preview timestamp", async () => {
    const user = userEvent.setup();
    const { client } = createClient();
    client.previewSource = vi.fn(async () => ({
      ...preview,
      timeRange: { start: "not-a-date", end: "not-a-date" },
      records: [{ ...preview.records[0], sourceTimestamp: "not-a-date" }],
    }));
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={client}
      />,
    );

    await user.click(
      await screen.findByRole("radio", { name: /Codex · First memory fixture/ }),
    );
    await user.click(screen.getByRole("button", { name: "Preview synthetic fixture" }));

    expect((await screen.findAllByText("Invalid source timestamp")).length).toBeGreaterThan(0);
  });

  test("does not remove a persisted mark when forgetting fails", async () => {
    const user = userEvent.setup();
    const { client, forgetSource } = createClient(approvedState);
    forgetSource.mockRejectedValueOnce(new Error("private database detail"));
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={quietDetailShell}
        memoryClient={client}
      />,
    );

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Forget this source" }));
    await user.click(
      screen.getByRole("checkbox", {
        name: /I understand that the local imported record/,
      }),
    );
    await user.click(
      screen.getByRole("button", { name: "Forget source and remove mark" }),
    );

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(
      screen.getByText(
        "The local operation did not complete. No private error details were shown and no UI state was approved.",
      ),
    ).toBeInTheDocument();
    expect(screen.queryByText("private database detail")).not.toBeInTheDocument();
  });

  test("refetches complete detail state after a safe revision event", async () => {
    const fixture = createClient();
    const detailEvents = createDetailEvents();
    render(
      <App
        detailEvents={detailEvents.client}
        detailShell={quietDetailShell}
        memoryClient={fixture.client}
      />,
    );
    await waitFor(() => expect(fixture.client.getState).toHaveBeenCalledTimes(1));
    vi.mocked(fixture.client.getState).mockResolvedValueOnce(approvedState);
    detailEvents.emitRevision();
    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(fixture.client.getState).toHaveBeenCalledTimes(2);
  });

  test("resets pending detail UI without confusing close with forgetting", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    const detailEvents = createDetailEvents();
    render(
      <App
        detailEvents={detailEvents.client}
        detailShell={quietDetailShell}
        memoryClient={fixture.client}
      />,
    );
    await user.click(await screen.findByRole("radio", { name: /Codex · First memory fixture/ }));
    await user.click(screen.getByRole("button", { name: "Preview synthetic fixture" }));
    expect(await screen.findByText(/Shipped a local-first creature/)).toBeInTheDocument();
    detailEvents.emitReset();
    await waitFor(() =>
      expect(screen.queryByText(/Shipped a local-first creature/)).not.toBeInTheDocument(),
    );
    expect(fixture.client.forgetSource).not.toHaveBeenCalled();
  });

  test("drops a late work-record preview after the native detail session resets", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    const detailEvents = createDetailEvents();
    let resolvePreview: ((value: ImportPreview) => void) | undefined;
    vi.mocked(fixture.client.listCodexThreads).mockResolvedValueOnce({
      catalogId: "catalog_late",
      candidates: [
        {
          candidateId: "candidate_late",
          displayName: "Codex work record 01",
          updatedAt: "2026-08-12T07:30:00Z",
          sourceKind: "codex-work-record",
        },
      ],
    });
    vi.mocked(fixture.client.previewCodexThread).mockReturnValueOnce(
      new Promise((resolve) => {
        resolvePreview = resolve;
      }),
    );
    render(
      <App
        detailEvents={detailEvents.client}
        detailShell={quietDetailShell}
        memoryClient={fixture.client}
      />,
    );

    await user.click(
      await screen.findByRole("button", { name: "Browse local Codex work records" }),
    );
    await user.click(screen.getByRole("radio", { name: /Codex work record 01/ }));
    await user.click(screen.getByRole("button", { name: "Review this record" }));
    await waitFor(() =>
      expect(fixture.client.previewCodexThread).toHaveBeenCalledTimes(1),
    );

    detailEvents.emitReset();
    resolvePreview?.(threadPreview);
    await waitFor(() =>
      expect(screen.queryByText("CONTENT HIDDEN FROM THE WEBVIEW")).not.toBeInTheDocument(),
    );
    expect(fixture.approveImport).not.toHaveBeenCalled();
    expect(screen.queryByText("PRIVATE FINAL ANSWER MUST NOT RENDER")).not.toBeInTheDocument();
  });

  test("lets native detail reset the pet guide with honest error handling", async () => {
    const user = userEvent.setup();
    const fixture = createClient();
    const resetOnboarding = vi
      .fn()
      .mockRejectedValueOnce(new Error("private native path"))
      .mockResolvedValueOnce({
        schemaVersion: 1,
        onboardingDismissed: false,
        alwaysOnTop: true,
      });
    render(
      <App
        detailEvents={quietDetailEvents}
        detailShell={{ resetOnboarding }}
        memoryClient={fixture.client}
      />,
    );
    const button = screen.getByRole("button", { name: "Show pet guide again" });
    await user.click(button);
    expect(screen.getByText("The local setting did not change. Try again from the desktop app.")).toBeInTheDocument();
    expect(screen.queryByText(/private native path/)).not.toBeInTheDocument();
    await user.click(button);
    expect(screen.getByText("The pet guide will appear the next time the pet is shown.")).toBeInTheDocument();
  });
});
