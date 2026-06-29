import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("charts-styling preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-styling");
    await expect(page.getByTestId("charts-styling-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "charts-styling");
    await expectPreviewVariants(page, ["charts-styling-preview", "charts-styling-gradient-preview"]);
  });

  test("applies custom series color override", async ({ page }) => {
    await openComponentPreview(page, "charts-styling");
    const chart = page.getByTestId("charts-styling-preview");
    const targetBars = chart.locator("[data-orbital-chart] svg .orb-bar-mark[fill='#f97316']");
    await expect(targetBars.first()).toBeVisible({ timeout: 5_000 });
    expect(await targetBars.count()).toBeGreaterThan(0);
  });

  test("gradient variant uses linear gradient fill", async ({ page }) => {
    await openComponentPreview(page, "charts-styling");
    const gradientPreview = page.getByTestId("charts-styling-gradient-preview");
    const bar = gradientPreview.locator("[data-orbital-chart] svg .orb-bar-mark").first();
    await expect(bar).toBeVisible({ timeout: 5_000 });
    await expect(bar).toHaveAttribute("fill", "url(#charts-styling-gradient)");
  });
});
