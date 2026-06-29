import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-range-field placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-range-field");
    await expect(page.getByTestId("date-range-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-range-field");
    await expectPreviewVariants(page, ["date-range-field-preview"]);
  });
});
