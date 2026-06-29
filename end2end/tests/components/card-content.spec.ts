import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-content preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-content");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-content");
    await expectPreviewVariants(page, ["card-content-default", "card-content-stacked"]);
  });

  test("body padding region inside card", async ({ page }) => {
    await openComponentPreview(page, "card-content");
    const content = page.getByTestId("card-content-default").locator(".orbital-card-content");
    await expect(content).toBeVisible();
    const padding = await content.evaluate((el) => getComputedStyle(el).padding);
    expect(padding).not.toBe("0px");
  });

  test("stacked content below media", async ({ page }) => {
    await openComponentPreview(page, "card-content");
    await page.getByTestId("card-content-stacked").scrollIntoViewIfNeeded();
    const stacked = page.getByTestId("card-content-stacked");
    await expect(stacked.locator("img.orbital-card-media")).toBeVisible();
    await expect(stacked.locator(".orbital-card-content")).toBeVisible();
    await expect(stacked.locator(".orbital-card-footer")).toBeVisible();
  });
});
