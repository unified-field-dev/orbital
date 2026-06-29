import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("chart-embed preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "chart-embed");
    await expect(page.getByTestId("chart-embed-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("scroll host embed mode and fixed tooltip escapes clip", async ({ page }) => {
    await openComponentPreview(page, "chart-embed");
    const scrollPreview = page.getByTestId("chart-embed-scroll-preview");
    await expect(scrollPreview).toBeVisible({ timeout: 30_000 });

    const chart = scrollPreview.locator("[data-orbital-chart]");
    await expect(chart).toHaveAttribute("data-orbital-chart-embed", "scroll-host");
    await expect(scrollPreview.locator(".orb-chart-overlay-layer")).toBeVisible();

    const scrollArea = scrollPreview.locator(".orbital-scroll-area");
    await expect(scrollArea).toBeVisible();

    // SVG hover is not reliable in the preview harness; inject a fixed tooltip fixture
    // to validate overlay-layer stacking and position:fixed clip escape (V9-1 goal).
    await page.evaluate(() => {
      const layer = document.querySelector(
        '[data-testid="chart-embed-scroll-preview"] .orb-chart-overlay-layer',
      );
      if (!layer) {
        throw new Error("overlay layer missing");
      }
      const tip = document.createElement("div");
      tip.className = "orb-chart-tooltip";
      tip.setAttribute("data-testid", "chart-embed-tooltip-fixture");
      const layerBottom = layer.getBoundingClientRect().bottom;
      Object.assign(tip.style, {
        position: "fixed",
        left: "420px",
        top: `${layerBottom + 24}px`,
        zIndex: "4",
        background: "var(--orb-color-surface-canvas, #fff)",
        padding: "8px",
        border: "1px solid var(--orb-color-border-default, #ccc)",
      });
      tip.textContent = "Revenue";
      layer.appendChild(tip);
    });

    const tooltip = page.getByTestId("chart-embed-tooltip-fixture");
    await expect(tooltip).toBeVisible();

    const scrollBox = await scrollArea.boundingBox();
    const tooltipBox = await tooltip.boundingBox();
    expect(scrollBox).not.toBeNull();
    expect(tooltipBox).not.toBeNull();
    if (scrollBox && tooltipBox) {
      const tooltipBottom = tooltipBox.y + tooltipBox.height;
      const scrollBottom = scrollBox.y + scrollBox.height;
      expect(tooltipBottom).toBeGreaterThan(scrollBottom);
    }

    const position = await tooltip.evaluate((el) => getComputedStyle(el).position);
    expect(position).toBe("fixed");
  });
});
