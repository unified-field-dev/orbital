import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("content-with-aside preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "content-with-aside");
  });

  test("CWA-01 renders two-column layout", async ({ page }) => {
    await expect(page.getByTestId("content-with-aside-preview")).toBeVisible();
  });

  test("CWA-02 sticky aside variant", async ({ page }) => {
    await expectPreviewVariants(page, ["content-with-aside-sticky"]);
    await expect(page.getByTestId("content-with-aside-sticky")).toBeVisible();
  });
});
