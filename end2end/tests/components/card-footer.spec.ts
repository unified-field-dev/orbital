import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-footer preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-footer");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-footer");
    await expectPreviewVariants(page, ["card-footer-single", "card-footer-actions"]);
  });

  test("single action footer", async ({ page }) => {
    await openComponentPreview(page, "card-footer");
    const footer = page.getByTestId("card-footer-single").locator(".orbital-card-footer");
    await expect(footer).toBeVisible();
    await expect(footer.locator("button")).toHaveCount(1);
  });

  test("multiple action footer", async ({ page }) => {
    await openComponentPreview(page, "card-footer");
    await page.getByTestId("card-footer-actions").scrollIntoViewIfNeeded();
    const footer = page.getByTestId("card-footer-actions").locator(".orbital-card-footer");
    await expect(footer.locator("button")).toHaveCount(2);
  });
});
