import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import { DailyScoutPanel } from "./DailyScoutPanel";
import {
  type ConfigureDailyScoutRequest,
  type DailyScoutClient,
  type DailyScoutState,
} from "./dailyScoutClient";

const context = {
  schemaVersion: 1 as const,
  workDomains: ["agent-assisted software development"],
  publicToolsAndModels: ["Codex", "Tauri", "React"],
  currentGoals: ["improve a verified coding workflow"],
  nonSensitiveConstraints: ["Windows desktop delivery"],
  evidenceWindow: { startDate: "2026-08-10", endDate: "2026-08-13" },
  preferredInsightCategories: ["workflow update"],
};

const readyToConfigure: DailyScoutState = {
  enabled: false,
  hasApiKey: false,
  canEnable: false,
  provider: "openai",
  model: "gpt-5.6-terra",
  deliveryTime: "10:00",
  status: "off",
  contextPreview: context,
};

function clientWith(state: DailyScoutState): DailyScoutClient {
  return {
    available: true,
    getState: vi.fn(async () => state),
    saveApiKey: vi.fn(async (): Promise<DailyScoutState> => ({
      ...state,
      hasApiKey: true,
      canEnable: true,
      status: "off",
    })),
    testApiKey: vi.fn(async () => undefined),
    configure: vi.fn(
      async (request: ConfigureDailyScoutRequest): Promise<DailyScoutState> => ({
        ...state,
        enabled: true,
        hasApiKey: true,
        canEnable: true,
        deliveryTime: request.deliveryTime,
        status: "scheduled",
      }),
    ),
    disable: vi.fn(async () => state),
    deleteApiKey: vi.fn(async () => state),
    clearHistory: vi.fn(async () => state),
    reset: vi.fn(async () => state),
    markRead: vi.fn(async () => state),
    openLink: vi.fn(async () => undefined),
  };
}

describe("Daily Memory Scout panel", () => {
  test("keeps browser preview offline and explains that the ordinary pet needs no API", () => {
    const client = { ...clientWith(readyToConfigure), available: false };
    render(<DailyScoutPanel client={client} locale="en" />);

    expect(screen.getByText("The ordinary local pet does not need an API.")).toBeInTheDocument();
    expect(screen.getByText(/browser preview makes no request/i)).toBeInTheDocument();
    expect(client.getState).not.toHaveBeenCalled();
  });

  test("clears the key field, previews the exact coarse context, and requires consent", async () => {
    const user = userEvent.setup();
    const client = clientWith(readyToConfigure);
    render(<DailyScoutPanel client={client} locale="en" />);

    expect(await screen.findByText("agent-assisted software development")).toBeInTheDocument();
    expect(screen.getByText(/prompts, final-answer text, paths/i)).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /Get an OpenAI API key/ })).toHaveAttribute(
      "href",
      "https://platform.openai.com/api-keys",
    );
    await user.click(screen.getByRole("link", { name: /Get an OpenAI API key/ }));
    expect(client.openLink).toHaveBeenCalledWith("api-keys", undefined);

    const enable = screen.getByRole("button", { name: "Enable Daily Scout" });
    expect(enable).toBeDisabled();
    const keyInput = screen.getByLabelText("Paste an OpenAI API key");
    await user.type(keyInput, "synthetic-key-material-never-sent");
    await user.click(screen.getByRole("button", { name: "Save key securely" }));
    await waitFor(() => expect(keyInput).toHaveValue(""));
    expect(client.saveApiKey).toHaveBeenCalledWith("synthetic-key-material-never-sent");

    await user.click(screen.getByRole("checkbox"));
    expect(enable).toBeEnabled();
    await user.click(enable);
    expect(client.configure).toHaveBeenCalledWith({
      locale: "en",
      deliveryTime: "10:00",
      consentAccepted: true,
    });
    expect(await screen.findByText(/Ready to check once today/)).toBeInTheDocument();
  });

  test("renders only validated citations and marks a returned insight read", async () => {
    const user = userEvent.setup();
    const state: DailyScoutState = {
      ...readyToConfigure,
      enabled: true,
      hasApiKey: true,
      canEnable: true,
      status: "ready",
      latestInsight: {
        id: "insight-1",
        localDate: "2026-08-13",
        provider: "openai",
        model: "gpt-5.6-terra",
        petMessage: "Psst—this official workflow update may save you a verification step today.",
        strength: "practical",
        relevanceReason: "It matches your recent verified agent-coding work.",
        searchedAt: "2026-08-13T03:00:00Z",
        read: false,
        citations: [
          { title: "Official guide", url: "https://example.com/official-guide" },
        ],
      },
    };
    const client = clientWith(state);
    vi.mocked(client.markRead).mockResolvedValue({
      ...state,
      latestInsight: { ...state.latestInsight!, read: true },
    });
    render(<DailyScoutPanel client={client} locale="en" />);

    expect(await screen.findByText(state.latestInsight!.petMessage)).toBeInTheDocument();
    expect(screen.getByRole("link", { name: /Official guide/ })).toHaveAttribute(
      "href",
      "https://example.com/official-guide",
    );
    await user.click(screen.getByRole("link", { name: /Official guide/ }));
    expect(client.openLink).toHaveBeenCalledWith(
      "citation",
      "https://example.com/official-guide",
    );
    await user.click(screen.getByRole("button", { name: "Got it" }));
    expect(client.markRead).toHaveBeenCalledTimes(1);
    await waitFor(() =>
      expect(screen.queryByRole("button", { name: "Got it" })).not.toBeInTheDocument(),
    );
  });
});
