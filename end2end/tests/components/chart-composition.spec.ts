import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("chart-composition preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "chart-composition");
    await expect(page.getByTestId("chart-composition-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("renders both bar and line plot types", async ({ page }) => {
    await openComponentPreview(page, "chart-composition");
    const chart = page.locator("[data-orbital-chart]").first();
    await expect(chart.locator("svg .orb-bar-mark").first()).toBeVisible({ timeout: 30_000 });
    await expect(chart.locator("svg .orb-line-stroke").first()).toBeVisible({ timeout: 30_000 });
    const barCount = await chart.locator("svg .orb-bar-mark").count();
    const lineCount = await chart.locator("svg .orb-line-stroke").count();
    expect(barCount).toBeGreaterThan(0);
    expect(lineCount).toBeGreaterThan(0);
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "chart-composition");
    await expectPreviewVariants(page, ["chart-composition-preview"]);
  });
});
