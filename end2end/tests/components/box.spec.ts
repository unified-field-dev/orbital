import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("box primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "box");
  });

  test("BX-01 renders default preview", async ({ page }) => {
    await expect(page.getByTestId("box-preview")).toBeVisible();
  });

  test("BX-02 shows surface and spacing variants", async ({ page }) => {
    await expectPreviewVariants(page, ["box-padding", "box-margin", "box-width", "box-surface"]);
  });

  test("BX-03 padding variant applies inset", async ({ page }) => {
    await page.getByTestId("box-padding").scrollIntoViewIfNeeded();
    const box = page.getByTestId("box-padding").locator("> div").first();
    const padding = await box.evaluate((el) => getComputedStyle(el).paddingTop);
    expect(parseFloat(padding)).toBeGreaterThan(0);
  });
});
