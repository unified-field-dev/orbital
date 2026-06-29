import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-lazy-loading preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-lazy-loading");
    await expect(page.getByTestId("scheduler-timeline-lazy-loading-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-lazy-loading");
    await expectPreviewVariants(page, ["scheduler-timeline-lazy-loading-preview"]);
  });

  test("shows loading then events", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-lazy-loading");
    const preview = page.getByTestId("scheduler-timeline-lazy-loading-preview");

    await expect(preview.getByTestId("scheduler-loading-overlay")).toBeVisible({ timeout: 10_000 });
    await expect(preview.getByTestId("scheduler-loading-overlay")).toBeHidden({ timeout: 15_000 });
    await expect(preview.getByTestId("scheduler-event-evt-1")).toBeVisible({ timeout: 10_000 });
  });
});
