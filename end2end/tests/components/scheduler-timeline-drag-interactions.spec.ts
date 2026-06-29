import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-drag-interactions preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-drag-interactions");
    await expect(page.getByTestId("scheduler-timeline-drag-interactions-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-drag-interactions");
    await expectPreviewVariants(page, ["scheduler-timeline-drag-interactions-preview"]);
  });

  test("dragging event updates start time", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-drag-interactions");
    const preview = page.getByTestId("scheduler-timeline-drag-interactions-preview");
    const event = preview.locator(".orb-scheduler-event--draggable").first();
    await expect(event).toBeVisible({ timeout: 10_000 });

    const startBefore = await event.getAttribute("data-start-unix");
    expect(startBefore).not.toBeNull();

    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    const x = box!.x + box!.width / 2;
    const y = box!.y + box!.height / 2;

    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x + 80, y, { steps: 8 });
    await page.mouse.up();

    await expect
      .poll(async () => event.getAttribute("data-start-unix"))
      .not.toBe(startBefore);
    await expect(page.getByTestId("scheduler-event-drag-ghost")).toHaveCount(0);
  });
});
