import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-resources preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-resources");
    await expect(page.getByTestId("scheduler-calendar-resources-preview").first()).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-resources");
    await expectPreviewVariants(page, ["scheduler-calendar-resources-preview"]);
  });

  test("resource labels and scoped events render", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-resources");
    const wrapper = page.getByTestId("scheduler-calendar-resources-preview").first();
    await expect(wrapper).toBeVisible();

    await expect(wrapper.getByText("Conference Room A")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-event-evt-1")).toBeVisible();
  });
});
