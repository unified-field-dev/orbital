import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("gauge preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "gauge");
    await expect(page.getByTestId("gauge-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows value arc and meter role", async ({ page }) => {
    await openComponentPreview(page, "gauge");
    const meter = page.locator("[data-orbital-chart][role=meter]").first();
    await expect(meter).toBeVisible({ timeout: 30_000 });
    await expect(meter).toHaveAttribute("aria-valuenow", "75");
    await expect(meter).toHaveAttribute("aria-valuemin", "0");
    await expect(meter).toHaveAttribute("aria-valuemax", "100");
    await expect(page.locator(".orb-gauge-fill").first()).toBeVisible();
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "gauge");
    await expectPreviewVariants(page, [
      "gauge-preview",
      "gauge-semicircle-preview",
      "gauge-a11y-preview",
    ]);
  });

  test("a11y preview exposes aria-valuetext", async ({ page }) => {
    await openComponentPreview(page, "gauge");
    const meter = page.getByTestId("gauge-a11y-preview").locator("[role=meter]");
    await expect(meter).toHaveAttribute("aria-valuetext", "42 minutes of 60 remaining");
  });
});
