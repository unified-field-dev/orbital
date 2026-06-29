import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("split-button placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "split-button");
    await expect(page.getByTestId("split-button-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "split-button");
    await expectPreviewVariants(page, ["split-button-preview"]);
  });
});
