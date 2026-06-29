import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("line-chart preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "line-chart");
    await expect(page.getByTestId("line-chart-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows line stroke marks", async ({ page }) => {
    await openComponentPreview(page, "line-chart");
    await expect(page.locator("[data-orbital-chart] svg .orb-line-stroke").first()).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "line-chart");
    await expectPreviewVariants(page, [
      "line-chart-preview",
      "line-chart-connect-nulls-preview",
      "line-chart-reference-preview",
    ]);
  });

  test("reference line renders", async ({ page }) => {
    await openComponentPreview(page, "line-chart");
    const refPreview = page.getByTestId("line-chart-reference-preview");
    await expect(refPreview.locator(".orb-reference-line")).toBeVisible({ timeout: 30_000 });
  });

  test("connect nulls variant renders line stroke", async ({ page }) => {
    await openComponentPreview(page, "line-chart");
    const nullsPreview = page.getByTestId("line-chart-connect-nulls-preview");
    await expect(nullsPreview.locator("[data-orbital-chart] svg .orb-line-stroke").first()).toBeVisible({ timeout: 5_000 });
  });
});
