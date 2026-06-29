import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("charts-axis preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-axis");
    await expect(page.getByTestId("charts-axis-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "charts-axis");
    await expectPreviewVariants(page, ["charts-axis-preview"]);
  });

  test("renders axis ticks and grid lines", async ({ page }) => {
    await openComponentPreview(page, "charts-axis");
    const preview = page.getByTestId("charts-axis-preview");
    await expect(preview.locator(".orb-axis-tick-label").first()).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator(".orb-grid-line")).not.toHaveCount(0, { timeout: 10_000 });
  });
});
