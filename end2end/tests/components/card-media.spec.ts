import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-media preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-media");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-media");
    await expectPreviewVariants(page, [
      "card-media-default",
      "card-media-tall",
      "card-media-in-card",
    ]);
  });

  test("default image is visible with alt text", async ({ page }) => {
    await openComponentPreview(page, "card-media");
    const img = page.getByTestId("card-media-default").locator("img.orbital-card-media");
    await expect(img).toBeVisible();
    await expect(img).toHaveAttribute("alt", "Sample card illustration");
  });

  test("height prop applies to image", async ({ page }) => {
    await openComponentPreview(page, "card-media");
    await page.getByTestId("card-media-tall").scrollIntoViewIfNeeded();
    const height = await page
      .getByTestId("card-media-tall")
      .locator("img.orbital-card-media")
      .evaluate((el) => getComputedStyle(el).height);
    expect(parseFloat(height)).toBeCloseTo(200, 0);
  });
});
