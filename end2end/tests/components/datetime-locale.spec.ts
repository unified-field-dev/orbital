import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("datetime-locale preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "datetime-locale");
    await expect(page.getByTestId("datetime-locale-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "datetime-locale");
    await expectPreviewVariants(page, ["datetime-locale-preview"]);
  });

  test("uses localized weekday labels from locale provider", async ({ page }) => {
    await openComponentPreview(page, "datetime-locale");
    await expect(page.getByTestId("datetime-locale-weekday-sample")).toHaveText("lun.");
    await expect(page.locator(".orbital-calendar__weekday").first()).toHaveText("lun.");
  });
});
