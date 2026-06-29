import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("area-chart preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "area-chart");
    await expect(page.getByTestId("area-chart-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows area fill marks", async ({ page }) => {
    await openComponentPreview(page, "area-chart");
    await expect(page.locator("[data-orbital-chart] svg .orb-area-fill").first()).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "area-chart");
    await expectPreviewVariants(page, [
      "area-chart-preview",
      "area-chart-percent-preview",
    ]);
  });

  test("percent stacked variant renders area fills", async ({ page }) => {
    await openComponentPreview(page, "area-chart");
    const percentPreview = page.getByTestId("area-chart-percent-preview");
    await expect(percentPreview.locator("[data-orbital-chart] svg .orb-area-fill").first()).toBeVisible({ timeout: 5_000 });
  });
});
