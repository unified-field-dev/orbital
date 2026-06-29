import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("heatmap preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "heatmap");
    await expect(page.getByTestId("heatmap-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows heatmap cells", async ({ page }) => {
    await openComponentPreview(page, "heatmap");
    const cells = page.locator("[data-orbital-chart] svg .orb-heatmap-cell");
    await expect(cells.first()).toBeVisible({ timeout: 30_000 });
    const count = await cells.count();
    expect(count).toBeGreaterThanOrEqual(20);
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "heatmap");
    await expectPreviewVariants(page, [
      "heatmap-preview",
      "heatmap-piecewise-preview",
      "heatmap-interaction-preview",
    ]);
  });

  test("uses svg renderer for preview grid", async ({ page }) => {
    await openComponentPreview(page, "heatmap");
    await expect(page.getByTestId("heatmap-preview").locator("[data-orbital-heatmap-renderer=svg]")).toBeVisible();
  });

  test("interaction variant shows color scale legend", async ({ page }) => {
    await openComponentPreview(page, "heatmap");
    const interactionPreview = page.getByTestId("heatmap-interaction-preview");
    await expect(interactionPreview.locator(".orb-color-scale-legend")).toBeVisible({ timeout: 5_000 });
  });

  test("interaction variant shows tooltip on cell hover", async ({ page }) => {
    await openComponentPreview(page, "heatmap");
    const interactionPreview = page.getByTestId("heatmap-interaction-preview");
    const cell = interactionPreview.locator("[data-orbital-chart] svg .orb-heatmap-cell").first();
    await cell.hover({ force: true });
    await expect(interactionPreview.locator(".orb-chart-tooltip").first()).toBeVisible({ timeout: 5_000 });
  });
});
