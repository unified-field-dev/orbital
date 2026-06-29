import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-header preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-header");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-header");
    await expectPreviewVariants(page, [
      "card-header-title",
      "card-header-description",
      "card-header-action",
      "card-header-media",
    ]);
  });

  test("title-only header", async ({ page }) => {
    await openComponentPreview(page, "card-header");
    const header = page.getByTestId("card-header-title").locator(".orbital-card-header");
    await expect(header).toBeVisible();
    await expect(header.locator(".orbital-card-header__header")).toBeVisible();
  });

  test("header with description row", async ({ page }) => {
    await openComponentPreview(page, "card-header");
    await page.getByTestId("card-header-description").scrollIntoViewIfNeeded();
    const header = page.getByTestId("card-header-description").locator(".orbital-card-header");
    await expect(header).toHaveClass(/orbital-card-header-with-description/);
    await expect(header.locator(".orbital-card-header__description")).toBeVisible();
  });

  test("header with action slot", async ({ page }) => {
    await openComponentPreview(page, "card-header");
    await page.getByTestId("card-header-action").scrollIntoViewIfNeeded();
    const header = page.getByTestId("card-header-action").locator(".orbital-card-header");
    await expect(header.locator(".orbital-card-header__action")).toBeVisible();
  });

  test("header with media card", async ({ page }) => {
    await openComponentPreview(page, "card-header");
    await page.getByTestId("card-header-media").scrollIntoViewIfNeeded();
    const img = page.getByTestId("card-header-media").locator("img.orbital-card-media");
    await expect(img).toBeVisible();
    await expect(img).toHaveAttribute("alt", "Sample card illustration");
  });
});
