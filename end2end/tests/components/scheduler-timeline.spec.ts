import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline");
    await expect(page.getByTestId("scheduler-timeline-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline");
    await expectPreviewVariants(page, ["scheduler-timeline-preview"]);
  });

  test("timeline product root and lanes render", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline");
    const wrapper = page.getByTestId("scheduler-timeline-preview");
    await expect(wrapper).toBeVisible();
    await expect(wrapper.locator("[data-orbital-scheduler-timeline]")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-timeline-header-title")).toBeVisible();
  });
});
