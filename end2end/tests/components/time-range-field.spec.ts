import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("time-range-field placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "time-range-field");
    await expect(page.getByTestId("time-range-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "time-range-field");
    await expectPreviewVariants(page, ["time-range-field-preview"]);
  });
});
