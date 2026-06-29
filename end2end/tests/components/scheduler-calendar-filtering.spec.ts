import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-filtering deferred preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-filtering");
    await expect(page.getByTestId("scheduler-calendar-filtering-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-filtering");
    await expectPreviewVariants(page, ["scheduler-calendar-filtering-preview"]);
  });

  test("shows deferred feature notice", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-filtering");
    await expect(page.getByTestId("scheduler-deferred-SC-12")).toBeVisible();
  });
});
