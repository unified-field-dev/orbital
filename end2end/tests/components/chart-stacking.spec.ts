import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("chart-stacking preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "chart-stacking");
    await expect(page.getByTestId("chart-stacking-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows stacked bar segments", async ({ page }) => {
    await openComponentPreview(page, "chart-stacking");
    const chart = page.locator("[data-orbital-chart]").first();
    await expect(chart.locator("svg .orb-bar-mark").first()).toBeVisible({ timeout: 30_000 });
    const barCount = await chart.locator("svg .orb-bar-mark").count();
    // 7 months × 3 stacked series
    expect(barCount).toBeGreaterThanOrEqual(21);
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "chart-stacking");
    await expectPreviewVariants(page, ["chart-stacking-preview"]);
  });
});
