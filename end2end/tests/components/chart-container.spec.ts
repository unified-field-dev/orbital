import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("chart-container preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "chart-container");
    await expect(page.getByTestId("chart-container-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "chart-container");
    await expectPreviewVariants(page, ["chart-container-preview"]);
  });

  test("renders chart shell with axes", async ({ page }) => {
    await openComponentPreview(page, "chart-container");
    const preview = page.getByTestId("chart-container-preview");
    await scrollIntoPreviewView(preview);
    const root = preview.locator("[data-orbital-chart]").first();
    await expect(root).toBeVisible({ timeout: 30_000 });
    const svg = preview.locator("svg.orb-chart-svg").first();
    await expect(svg).toBeVisible();
    const box = await svg.boundingBox();
    expect(box?.width ?? 0).toBeGreaterThan(0);
    expect(box?.height ?? 0).toBeGreaterThan(0);
    await expect(preview.locator(".orb-axis")).toHaveCount(2, { timeout: 10_000 });
  });
});
