import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-drag-interactions preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-drag-interactions");
    await expect(page.getByTestId("scheduler-calendar-drag-interactions-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-drag-interactions");
    await expectPreviewVariants(page, ["scheduler-calendar-drag-interactions-preview"]);
  });

  test("dragging event updates start time", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-drag-interactions");
    const wrapper = page.getByTestId("scheduler-calendar-week-preview").first();
    await expect(wrapper).toBeVisible();

    const event = wrapper.getByTestId("scheduler-event-evt-1");
    await expect(event).toBeVisible();
    const startBefore = await event.getAttribute("data-start-unix");
    expect(startBefore).not.toBeNull();

    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    const x = box!.x + box!.width / 2;
    const y = box!.y + box!.height / 2;

    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x, y + 60, { steps: 8 });
    await page.mouse.up();

    await expect
      .poll(async () => event.getAttribute("data-start-unix"))
      .not.toBe(startBefore);

    await expect(page.getByTestId("scheduler-event-drag-ghost")).toHaveCount(0);
  });

  test("navigating away mid-drag does not panic", async ({ page }) => {
    const panics: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error" && (msg.text().includes("panicked") || msg.text().includes("unreachable"))) {
        panics.push(msg.text());
      }
    });

    await openComponentPreview(page, "scheduler-calendar-drag-interactions");
    const wrapper = page.getByTestId("scheduler-calendar-week-preview").first();
    await expect(wrapper).toBeVisible();

    const event = wrapper.getByTestId("scheduler-event-evt-1");
    await expect(event).toBeVisible();
    const box = await event.boundingBox();
    expect(box).not.toBeNull();
    const x = box!.x + box!.width / 2;
    const y = box!.y + box!.height / 2;

    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x, y + 60, { steps: 8 });
    await page.getByTestId("preview-catalog-nav").getByRole("link", { name: "Introduction" }).click();
    await page.mouse.up();

    await expect(page.getByTestId("preview-index")).toBeVisible();
    expect(panics).toEqual([]);
  });
});
