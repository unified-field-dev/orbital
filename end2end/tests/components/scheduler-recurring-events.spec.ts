import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-recurring-events preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-recurring-events");
    await expect(page.getByTestId("scheduler-recurring-events-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-recurring-events");
    await expectPreviewVariants(page, ["scheduler-recurring-events-preview"]);
  });

  test("expands weekly standup into multiple instances", async ({ page }) => {
    await openComponentPreview(page, "scheduler-recurring-events");
    const wrapper = page.getByTestId("scheduler-recurring-events-preview").first();
    await expect(wrapper).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-calendar-view-week")).toBeVisible();

    const instances = wrapper.locator('[data-testid^="scheduler-event-weekly-standup::"]');
    await expect(instances).toHaveCount(3, { timeout: 10_000 });
  });
});
