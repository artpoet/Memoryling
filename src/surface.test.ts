import { describe, expect, test } from "vitest";
import { resolveSurface } from "./surface";

describe("surface routing", () => {
  test("keeps browser preview honest even when the URL asks for a pet", () => {
    expect(resolveSurface({ native: false, requestedSurface: "pet" })).toBe("browser");
  });

  test("uses the native label as the authority", () => {
    expect(
      resolveSurface({ native: true, windowLabel: "pet", requestedSurface: "detail" }),
    ).toBe("pet");
    expect(
      resolveSurface({ native: true, windowLabel: "main", requestedSurface: "pet" }),
    ).toBe("detail");
  });

  test("fails closed for an unknown native label", () => {
    expect(resolveSurface({ native: true, windowLabel: "unknown" })).toBe(
      "unsupported-native",
    );
  });
});
