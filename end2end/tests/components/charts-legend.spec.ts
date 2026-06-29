import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("charts-legend preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-legend");
    await expect(page.getByTestId("charts-legend-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "charts-legend");
    await expectPreviewVariants(page, ["charts-legend-preview"]);
  });

  test("legend checkbox toggles series visibility", async ({ page }) => {
    await openComponentPreview(page, "charts-legend");
    const chart = page.getByTestId("charts-legend-preview");
    const bars = chart.locator("[data-orbital-chart] svg .orb-bar-mark");
    const initialCount = await bars.count();
    expect(initialCount).toBeGreaterThan(0);

    const costLegendItem = chart.locator(".orb-legend-item", { hasText: "Cost" });
    await costLegendItem.locator("input[type='checkbox']").click();

    await expect
      .poll(async () => bars.count(), { timeout: 5_000 })
      .toBeLessThan(initialCount);
  });
});
