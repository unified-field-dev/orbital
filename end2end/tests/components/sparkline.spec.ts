import { test, expect } from "@playwright/test";
import { hoverChartMark } from "../lib/preview/charts";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("sparkline preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "sparkline");
    await expect(page.getByTestId("sparkline-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows line sparkline mark", async ({ page }) => {
    await openComponentPreview(page, "sparkline");
    await expect(page.locator("[data-orbital-chart] svg .orb-sparkline-line").first()).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "sparkline");
    await expectPreviewVariants(page, [
      "sparkline-preview",
      "sparkline-bar-preview",
      "sparkline-y-range-preview",
      "sparkline-stat-embed-preview",
      "sparkline-tooltip-preview",
      "sparkline-highlight-preview",
    ]);
  });

  test("bar variant renders bar marks", async ({ page }) => {
    await openComponentPreview(page, "sparkline");
    await expect(page.getByTestId("sparkline-bar-preview").locator(".orb-sparkline-bar").first()).toBeVisible({ timeout: 30_000 });
  });

  test("tooltip variant shows tooltip on hover", async ({ page }) => {
    await openComponentPreview(page, "sparkline");
    const tooltipPreview = page.getByTestId("sparkline-tooltip-preview");
    await scrollIntoPreviewView(tooltipPreview);
    const chartRoot = tooltipPreview.locator("[data-orbital-chart]");
    await expect(chartRoot).toHaveAttribute("data-orbital-chart-marker-count", "8", { timeout: 10_000 });
    await chartRoot.focus();
    await page.keyboard.press("ArrowRight");
    await expect(page.locator(".orb-chart-tooltip").first()).toBeVisible({ timeout: 10_000 });
  });

  test("highlight variant fades non-hovered bars", async ({ page }) => {
    await openComponentPreview(page, "sparkline");
    const highlightPreview = page.getByTestId("sparkline-highlight-preview");
    await scrollIntoPreviewView(highlightPreview);
    const bars = highlightPreview.locator("[data-orbital-chart] svg .orb-sparkline-bar");
    await hoverChartMark(bars.first());
    await expect
      .poll(
        async () =>
          bars.evaluateAll((nodes) =>
            nodes.filter((node) => node.classList.contains("orb-sparkline-bar-faded")).length,
          ),
        { timeout: 15_000 },
      )
      .toBeGreaterThan(0);
  });
});
