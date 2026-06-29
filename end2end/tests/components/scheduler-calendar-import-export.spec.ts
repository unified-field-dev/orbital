import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-import-export placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-import-export");
    await expect(page.getByTestId("scheduler-calendar-import-export-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-import-export");
    await expectPreviewVariants(page, ["scheduler-calendar-import-export-preview"]);
  });
});
