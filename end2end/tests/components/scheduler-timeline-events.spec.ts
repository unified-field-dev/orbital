import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-events preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-events");
    await expect(page.getByTestId("scheduler-timeline-events-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-events");
    await expectPreviewVariants(page, ["scheduler-timeline-events-preview"]);
  });

  test("static events render on timeline lanes", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-events");
    const wrapper = page.getByTestId("scheduler-timeline-events-preview");
    await expect(wrapper).toBeVisible();

    const event = wrapper.getByTestId("scheduler-event-evt-1");
    await expect(event).toBeVisible();
    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
  });
});
