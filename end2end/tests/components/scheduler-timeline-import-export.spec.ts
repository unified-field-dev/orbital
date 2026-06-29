import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-import-export placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-import-export");
    await expect(page.getByTestId("scheduler-timeline-import-export-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-import-export");
    await expectPreviewVariants(page, ["scheduler-timeline-import-export-preview"]);
  });
});
