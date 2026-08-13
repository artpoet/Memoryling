import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import type {
  DailyScoutClient,
  DailyScoutState,
} from "./dailyScoutClient";
import { ProductSetup } from "./ProductSetup";
import type { ProductSetupClient } from "./productSetupClient";

const scoutOff: DailyScoutState = {
  enabled: false,
  hasApiKey: false,
  canEnable: false,
  provider: "openai",
  model: "gpt-5.6-terra",
  deliveryTime: "10:00",
  status: "off",
};

function createClients(hasApiKey = false) {
  const dailyScoutClient: DailyScoutClient = {
    available: true,
    getState: vi.fn(async () => ({ ...scoutOff, hasApiKey })),
    saveApiKey: vi.fn(async () => ({ ...scoutOff, hasApiKey: true })),
    testApiKey: vi.fn(async () => undefined),
    configure: vi.fn(async () => scoutOff),
    disable: vi.fn(async () => scoutOff),
    deleteApiKey: vi.fn(async () => scoutOff),
    clearHistory: vi.fn(async () => scoutOff),
    reset: vi.fn(async () => scoutOff),
    markRead: vi.fn(async () => scoutOff),
    openLink: vi.fn(async () => undefined),
  };
  const setupClient: ProductSetupClient = {
    available: true,
    getState: vi.fn(async () => ({
      schemaVersion: 1 as const,
      setupComplete: false,
    })),
    complete: vi.fn(async () => ({
      schemaVersion: 1 as const,
      setupComplete: true,
    })),
  };
  return { dailyScoutClient, setupClient };
}

describe("product setup", () => {
  test("creates a local pet without requesting or saving an API key", async () => {
    const user = userEvent.setup();
    const clients = createClients();
    const onComplete = vi.fn();
    render(
      <ProductSetup
        dailyScoutClient={clients.dailyScoutClient}
        locale="en"
        onComplete={onComplete}
        onLocaleChange={vi.fn()}
        setupClient={clients.setupClient}
      />,
    );

    expect(screen.queryByLabelText("OpenAI API key")).not.toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Wake up my Memoryling" }));
    expect(clients.dailyScoutClient.saveApiKey).not.toHaveBeenCalled();
    expect(clients.setupClient.complete).toHaveBeenCalledTimes(1);
    expect(onComplete).toHaveBeenCalledTimes(1);
  });

  test("saves a key without enabling or testing Daily Scout", async () => {
    const user = userEvent.setup();
    const clients = createClients();
    render(
      <ProductSetup
        dailyScoutClient={clients.dailyScoutClient}
        locale="en"
        onComplete={vi.fn()}
        onLocaleChange={vi.fn()}
        setupClient={clients.setupClient}
      />,
    );
    await waitFor(() => expect(clients.dailyScoutClient.getState).toHaveBeenCalled());
    await user.click(screen.getByRole("radio", { name: /Prepare Daily Memory Scout/ }));
    await user.type(screen.getByLabelText("OpenAI API key"), "synthetic-key-material-never-sent");
    await user.click(screen.getByRole("button", { name: "Wake up my Memoryling" }));

    expect(clients.dailyScoutClient.saveApiKey).toHaveBeenCalledWith(
      "synthetic-key-material-never-sent",
    );
    expect(clients.dailyScoutClient.configure).not.toHaveBeenCalled();
    expect(clients.dailyScoutClient.testApiKey).not.toHaveBeenCalled();
    expect(clients.setupClient.complete).toHaveBeenCalledTimes(1);
  });

  test("switches setup language immediately", async () => {
    const user = userEvent.setup();
    const clients = createClients();
    const onLocaleChange = vi.fn();
    render(
      <ProductSetup
        dailyScoutClient={clients.dailyScoutClient}
        locale="zh-TW"
        onComplete={vi.fn()}
        onLocaleChange={onLocaleChange}
        setupClient={clients.setupClient}
      />,
    );
    await user.click(screen.getByRole("button", { name: "English" }));
    expect(onLocaleChange).toHaveBeenCalledWith("en");
  });

  test("keeps browser preview local-only", () => {
    const clients = createClients();
    clients.dailyScoutClient.available = false;
    render(
      <ProductSetup
        dailyScoutClient={clients.dailyScoutClient}
        locale="en"
        onComplete={vi.fn()}
        onLocaleChange={vi.fn()}
        setupClient={clients.setupClient}
      />,
    );
    expect(
      screen.getByRole("radio", { name: /Prepare Daily Memory Scout/ }),
    ).toBeDisabled();
  });
});
