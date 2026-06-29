import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-filtering placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-filtering");
    await expect(page.getByTestId("scheduler-timeline-filtering-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-filtering");
    await expectPreviewVariants(page, ["scheduler-timeline-filtering-preview"]);
  });
});
