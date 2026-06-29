import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("time-range-picker placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "time-range-picker");
    await expect(page.getByTestId("time-range-picker-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "time-range-picker");
    await expectPreviewVariants(page, ["time-range-picker-preview"]);
  });
});
