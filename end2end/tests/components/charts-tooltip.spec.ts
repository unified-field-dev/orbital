import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("charts-tooltip preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-tooltip");
    await expect(page.getByTestId("charts-tooltip-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "charts-tooltip");
    await expectPreviewVariants(page, ["charts-tooltip-preview", "charts-tooltip-axis-preview"]);
  });

  test("hovering a bar shows item tooltip", async ({ page }) => {
    await openComponentPreview(page, "charts-tooltip");
    const chart = page.getByTestId("charts-tooltip-preview");
    await scrollIntoPreviewView(chart);
    const chartRoot = chart.locator("[data-orbital-chart]");
    await expect(chartRoot).toHaveAttribute("data-orbital-chart-marker-count", "8", { timeout: 10_000 });
    await chartRoot.focus();
    await page.keyboard.press("ArrowRight");

    const tooltip = page.locator(".orb-chart-tooltip").first();
    await expect(tooltip).toBeVisible({ timeout: 10_000 });
    await expect(tooltip).toContainText(/Revenue|Cost/i);
  });
});
