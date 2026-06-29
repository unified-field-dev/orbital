import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-events preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-events");
    await expect(page.getByTestId("scheduler-calendar-events-preview").first()).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-events");
    await expectPreviewVariants(page, ["scheduler-calendar-events-preview"]);
  });

  test("static events render in week grid", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-events");
    const wrapper = page.getByTestId("scheduler-calendar-week-preview").first();
    await expect(wrapper).toBeVisible();

    const event = wrapper.getByTestId("scheduler-event-evt-1");
    await expect(event).toBeVisible();
    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.height).toBeGreaterThan(0);
  });
});
