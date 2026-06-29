import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-time-range-field placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-time-range-field");
    await expect(page.getByTestId("date-time-range-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-time-range-field");
    await expectPreviewVariants(page, ["date-time-range-field-preview"]);
  });
});
