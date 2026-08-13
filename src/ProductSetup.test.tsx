import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, test, vi } from "vitest";
import { ProductSetup } from "./ProductSetup";
import type { ProductSetupClient } from "./productSetupClient";

function createClient(): ProductSetupClient {
  return {
    available: true,
    getState: vi.fn(async () => ({ schemaVersion: 1 as const, setupComplete: false })),
    complete: vi.fn(async () => ({ schemaVersion: 1 as const, setupComplete: true })),
  };
}

describe("product setup", () => {
  test("explains the Agent-operated route without API setup", () => {
    render(
      <ProductSetup
        locale="en"
        onComplete={vi.fn()}
        onLocaleChange={vi.fn()}
        setupClient={createClient()}
      />,
    );

    expect(screen.getByText(/Run Memoryling/)).toBeInTheDocument();
    expect(screen.getByText(/No API key is needed/)).toBeInTheDocument();
    expect(screen.queryByLabelText(/API key/i)).not.toBeInTheDocument();
  });

  test("persists setup and completes", async () => {
    const user = userEvent.setup();
    const setupClient = createClient();
    const onComplete = vi.fn();
    render(
      <ProductSetup
        locale="en"
        onComplete={onComplete}
        onLocaleChange={vi.fn()}
        setupClient={setupClient}
      />,
    );

    await user.click(screen.getByRole("button", { name: "Wake up my Memoryling" }));
    expect(setupClient.complete).toHaveBeenCalledTimes(1);
    expect(onComplete).toHaveBeenCalledTimes(1);
  });

  test("switches setup language immediately", async () => {
    const user = userEvent.setup();
    const onLocaleChange = vi.fn();
    render(
      <ProductSetup
        locale="zh-TW"
        onComplete={vi.fn()}
        onLocaleChange={onLocaleChange}
        setupClient={createClient()}
      />,
    );

    expect(screen.getByText(/運作 Memoryling/)).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "English" }));
    expect(onLocaleChange).toHaveBeenCalledWith("en");
  });
});
