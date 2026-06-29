import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-resources preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-resources");
    await expect(page.getByTestId("scheduler-timeline-resources-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-resources");
    await expectPreviewVariants(page, ["scheduler-timeline-resources-preview"]);
  });

  test("resource labels and scoped events render", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-resources");
    const wrapper = page.getByTestId("scheduler-timeline-resources-preview");
    await expect(wrapper).toBeVisible();

    await expect(wrapper.getByTestId("scheduler-resource-room-a")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-event-evt-1")).toBeVisible();
  });
});
