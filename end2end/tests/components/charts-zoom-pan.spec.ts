import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("charts-zoom-pan preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-zoom-pan");
    await expect(page.getByTestId("charts-zoom-pan-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "charts-zoom-pan");
    await expectPreviewVariants(page, ["charts-zoom-pan-preview"]);
  });

  test("wheel zoom narrows visible domain", async ({ page }) => {
    await openComponentPreview(page, "charts-zoom-pan");
    const preview = page.getByTestId("charts-zoom-pan-preview");
    const chart = preview.locator("[data-orbital-chart]").first();
    await expect(chart).toBeVisible({ timeout: 30_000 });

    const before = await chart.getAttribute("data-orbital-chart-zoom-window");
    expect(before).toContain("x:0-100");

    const plot = chart.locator(".orb-zoom-layer");
    await plot.dispatchEvent("wheel", { deltaY: -120 });

    await expect.poll(async () => {
      const attr = await chart.getAttribute("data-orbital-chart-zoom-window");
      return attr !== before && attr !== "x:0-100";
    }).toBe(true);
  });

  test("reset button restores full range", async ({ page }) => {
    await openComponentPreview(page, "charts-zoom-pan");
    const preview = page.getByTestId("charts-zoom-pan-preview");
    const chart = preview.locator("[data-orbital-chart]").first();
    const plot = chart.locator(".orb-zoom-layer");

    await plot.dispatchEvent("wheel", { deltaY: -200 });
    await expect.poll(async () => {
      const attr = await chart.getAttribute("data-orbital-chart-zoom-window");
      return attr !== "x:0-100";
    }).toBe(true);

    await preview.getByRole("button", { name: "Reset zoom" }).click();
    await expect(chart).toHaveAttribute("data-orbital-chart-zoom-window", /x:0-100/);
  });
});
