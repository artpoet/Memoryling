import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import { App } from "./App";
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
      contentHash,
    },
  ],
  accessScope: {
    readOnly: true,
    sourceWriteAccess: false,
    networkAccess: false,
    arbitraryPathAccess: false,
  },
};

const approvedState: MemoryState = {
  storeSchemaVersion: 1,
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
          contentHash,
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
    cancelPreview,
    approveImport,
    forgetSource,
  };
  return { client, approveImport, forgetSource, cancelPreview };
}

describe("First real memory vertical slice", () => {
  test("moves from off to preview, persisted lineage, and complete forgetting", async () => {
    const user = userEvent.setup();
    const { client, approveImport, forgetSource } = createClient();
    render(<App memoryClient={client} />);

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
    await user.click(screen.getByRole("button", { name: "Preview selected source" }));

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
      screen.getByRole("button", { name: "Approve & store 1 memory locally" }),
    );

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(approveImport).toHaveBeenCalledWith({
      previewId: preview.previewId,
      sourceId: source.id,
      selectedRecordIds: ["synthetic-memory-001"],
    });
    expect(
      screen.getByText("Fixture pilot active · real memory access is off"),
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Why did this happen?" }));
    expect(screen.getByText("Machine-readable lineage")).toBeInTheDocument();
    expect(screen.getByText("Normalized memory event")).toBeInTheDocument();
    expect(screen.getByText("Creature world effect")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Forget this source" }));
    await user.click(
      screen.getByRole("checkbox", {
        name: /I understand that the local imported memory/,
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
      screen.getByText("Forgotten completely. Memory access is off again."),
    ).toBeInTheDocument();
  });

  test("restores the persisted mark after a desktop restart", async () => {
    const { client } = createClient(approvedState);
    render(<App memoryClient={client} />);

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(screen.getByText("One approved memory left a completion star")).toBeInTheDocument();
  });

  test("keeps the honest off state in a browser and preserves bilingual parity", async () => {
    const user = userEvent.setup();
    const unavailableClient: MemoryClient = {
      available: false,
      listSources: vi.fn(),
      getState: vi.fn(),
      previewSource: vi.fn(),
      cancelPreview: vi.fn(),
      approveImport: vi.fn(),
      forgetSource: vi.fn(),
    };
    render(<App memoryClient={unavailableClient} />);

    expect(screen.getByText("Desktop runtime required")).toBeInTheDocument();
    expect(
      screen.getByText("Browser preview · memory access is off"),
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "繁中" }));
    expect(screen.getByText("從核准來源，到可解釋的印記")).toBeInTheDocument();
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
    render(<App memoryClient={client} />);

    await user.click(
      await screen.findByRole("radio", { name: /Codex · First memory fixture/ }),
    );
    await user.click(screen.getByRole("button", { name: "Preview selected source" }));

    expect((await screen.findAllByText("Invalid source timestamp")).length).toBeGreaterThan(0);
  });

  test("does not remove a persisted mark when forgetting fails", async () => {
    const user = userEvent.setup();
    const { client, forgetSource } = createClient(approvedState);
    forgetSource.mockRejectedValueOnce(new Error("private database detail"));
    render(<App memoryClient={client} />);

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Forget this source" }));
    await user.click(
      screen.getByRole("checkbox", {
        name: /I understand that the local imported memory/,
      }),
    );
    await user.click(
      screen.getByRole("button", { name: "Forget source and remove mark" }),
    );

    expect(await screen.findByTestId("derived-memory-mark")).toBeInTheDocument();
    expect(
      screen.getByText(
        "The local operation did not complete. No UI state was changed; try again from the current step.",
      ),
    ).toBeInTheDocument();
    expect(screen.queryByText("private database detail")).not.toBeInTheDocument();
  });
});
