import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("bar-chart preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "bar-chart");
    await expect(page.getByTestId("bar-chart-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows grouped bar marks", async ({ page }) => {
    await openComponentPreview(page, "bar-chart");
    await expect(page.locator("[data-orbital-chart] svg .orb-bar-mark").first()).toBeVisible({ timeout: 30_000 });
    const count = await page.locator("[data-orbital-chart] svg .orb-bar-mark").count();
    expect(count).toBeGreaterThan(0);
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "bar-chart");
    await expectPreviewVariants(page, [
      "bar-chart-preview",
      "bar-chart-dataset-preview",
      "bar-chart-horizontal-preview",
    ]);
  });

  test("dataset variant renders bars from processed dataset", async ({ page }) => {
    await openComponentPreview(page, "bar-chart");
    const datasetPreview = page.getByTestId("bar-chart-dataset-preview");
    await expect(datasetPreview.locator("[data-orbital-chart] svg .orb-bar-mark").first()).toBeVisible({ timeout: 5_000 });
  });

  test("horizontal variant renders bars", async ({ page }) => {
    await openComponentPreview(page, "bar-chart");
    await expectPreviewVariants(page, ["bar-chart-horizontal-preview"]);
    const horizontalPreview = page.getByTestId("bar-chart-horizontal-preview");
    await scrollIntoPreviewView(horizontalPreview);
    await expect(horizontalPreview.locator("[data-orbital-chart] svg .orb-bar-mark").first()).toBeVisible({ timeout: 30_000 });
  });
});

test.describe("bar-chart-animation preview", () => {

  test("renders animation playground", async ({ page }) => {
    await openComponentPreview(page, "bar-chart-animation");
    await expect(page.getByTestId("bar-chart-animation-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("skip animation toggle sets data attribute", async ({ page }) => {
    await openComponentPreview(page, "bar-chart-animation");
    const chart = page.locator("[data-orbital-chart]").first();
    await expect(chart).toHaveAttribute("data-orbital-chart-skip-animation", "false");

    await page.getByRole("switch", { name: "Skip animation" }).click();
    await expect(chart).toHaveAttribute("data-orbital-chart-skip-animation", "true");
  });
});
