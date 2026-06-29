import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("pie-chart preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "pie-chart");
    await expect(page.getByTestId("pie-chart-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows pie slice marks", async ({ page }) => {
    await openComponentPreview(page, "pie-chart");
    await expect(page.locator("[data-orbital-chart] svg .orb-pie-slice").first()).toBeVisible({ timeout: 30_000 });
    const count = await page.locator("[data-orbital-chart] svg .orb-pie-slice").count();
    expect(count).toBeGreaterThanOrEqual(4);
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "pie-chart");
    await expectPreviewVariants(page, [
      "pie-chart-preview",
      "pie-chart-donut-preview",
      "pie-chart-center-preview",
      "pie-chart-highlight-preview",
    ]);
  });

  test("animations enabled on load", async ({ page }) => {
    await openComponentPreview(page, "pie-chart");
    const chart = page.locator("[data-orbital-chart]").first();
    await expect(chart).toHaveAttribute("data-orbital-chart-skip-animation", "false");
  });
});
