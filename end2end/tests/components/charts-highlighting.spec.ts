import { test, expect } from "@playwright/test";
import { hoverChartMark } from "../lib/preview/charts";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("charts-highlighting preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-highlighting");
    await expect(page.getByTestId("charts-highlighting-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "charts-highlighting");
    await expectPreviewVariants(page, ["charts-highlighting-preview"]);
  });

  test("hovering a bar fades sibling marks", async ({ page }) => {
    await openComponentPreview(page, "charts-highlighting");
    const chart = page.getByTestId("charts-highlighting-preview");
    await scrollIntoPreviewView(chart);
    const bars = chart.locator("[data-orbital-chart] svg .orb-bar-mark");
    const target = bars.first();
    await hoverChartMark(target);

    await expect
      .poll(
        async () => {
          const faded = await bars.evaluateAll((nodes) =>
            nodes.filter((node) => node.classList.contains("orb-bar-mark-faded")).length,
          );
          return faded;
        },
        { timeout: 5_000 },
      )
      .toBeGreaterThan(0);
  });
});
