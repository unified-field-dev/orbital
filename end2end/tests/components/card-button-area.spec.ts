import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-button-area preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-button-area");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-button-area");
    await expectPreviewVariants(page, [
      "card-button-area-default",
      "card-button-area-media",
      "card-button-area-states",
    ]);
  });

  test("click target is the button area element", async ({ page }) => {
    await openComponentPreview(page, "card-button-area");
    await page.getByTestId("card-button-area-default").scrollIntoViewIfNeeded();
    await expect(
      page.getByTestId("card-button-area-default").locator(".orbital-card-button-area"),
    ).toBeVisible();
  });

  test("media variant wraps image and content", async ({ page }) => {
    await openComponentPreview(page, "card-button-area");
    await page.getByTestId("card-button-area-media").scrollIntoViewIfNeeded();
    const area = page.getByTestId("card-button-area-media").locator(".orbital-card-button-area");
    await expect(area.locator("img.orbital-card-media")).toBeVisible();
    await expect(area.locator(".orbital-card-content")).toBeVisible();
  });

  test("interactive card footer click does not activate button area", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-button-area-preview").scrollIntoViewIfNeeded();
    const compound = page.getByTestId("card-button-area-preview");
    await expect(compound.locator(".orbital-card-button-area")).toBeVisible();
    await expect(compound.locator(".orbital-card-footer button")).toBeVisible();
    const footerBtn = compound.locator(".orbital-card-footer button").first();
    await footerBtn.click();
    await expect(compound.locator(".orbital-card-footer")).toBeVisible();
  });
});
