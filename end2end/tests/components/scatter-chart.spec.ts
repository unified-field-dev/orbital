import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scatter-chart preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scatter-chart");
    await expect(page.getByTestId("scatter-chart-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows scatter point marks", async ({ page }) => {
    await openComponentPreview(page, "scatter-chart");
    await expect(page.locator("[data-orbital-chart] svg .orb-scatter-point").first()).toBeVisible({ timeout: 30_000 });
    const count = await page.locator("[data-orbital-chart] svg .orb-scatter-point").count();
    expect(count).toBeGreaterThan(0);
  });

  test("hover highlights nearest point", async ({ page }) => {
    await openComponentPreview(page, "scatter-chart");
    await page.waitForLoadState("networkidle");
    const wrapper = page.getByTestId("scatter-chart-preview");
    const chart = wrapper.locator("[data-orbital-chart]");
    await expect(chart).toBeVisible({ timeout: 30_000 });

    const point = wrapper.locator(".orb-scatter-point").nth(2);
    await expect(point).toBeVisible();
    await point.hover({ force: true });
    await expect(wrapper.locator(".orb-scatter-point-highlighted").first()).toBeVisible({ timeout: 5_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "scatter-chart");
    await expectPreviewVariants(page, [
      "scatter-chart-preview",
      "scatter-chart-dataset-preview",
      "scatter-chart-biaxial-preview",
    ]);
  });

  test("animations enabled on load", async ({ page }) => {
    await openComponentPreview(page, "scatter-chart");
    const chart = page.locator("[data-orbital-chart]").first();
    await expect(chart).toHaveAttribute("data-orbital-chart-skip-animation", "false");
  });
});
