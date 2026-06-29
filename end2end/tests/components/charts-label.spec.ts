import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("charts-label preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-label");
    await expect(page.getByTestId("charts-label-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "charts-label");
    await expectPreviewVariants(page, ["charts-label-preview", "charts-label-pie-preview"]);
  });

  test("legend and tooltip use different location formatters", async ({ page }) => {
    await openComponentPreview(page, "charts-label");
    const chart = page.getByTestId("charts-label-preview");
    await scrollIntoPreviewView(chart);
    await expect(chart.locator(".orb-legend-label", { hasText: "Rev" })).toBeVisible();
    await expect(chart.locator(".orb-legend-label", { hasText: "Cost" })).toBeVisible();

    const chartRoot = chart.locator("[data-orbital-chart]");
    await expect(chartRoot).toHaveAttribute("data-orbital-chart-marker-count", "8", { timeout: 10_000 });
    await chartRoot.focus();
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("ArrowRight");

    const tooltip = page.locator(".orb-chart-tooltip").first();
    await expect(tooltip).toBeVisible({ timeout: 10_000 });
    await expect(tooltip).toContainText("Revenue (USD)");
    await expect(tooltip).not.toHaveText(/^Rev$/);
  });

  test("pie variant renders arc labels", async ({ page }) => {
    await openComponentPreview(page, "charts-label");
    const piePreview = page.getByTestId("charts-label-pie-preview");
    await scrollIntoPreviewView(piePreview);
    await expect(piePreview.locator(".orb-pie-label").first()).toBeVisible({ timeout: 5_000 });
    await expect(piePreview.locator(".orb-pie-label", { hasText: "North" })).toBeVisible();
  });
});
