import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("scheduler-timeline-virtualization preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-virtualization");
    await expect(page.getByTestId("scheduler-timeline-virtualization-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-virtualization");
    await expectPreviewVariants(page, ["scheduler-timeline-virtualization-preview"]);
  });

  test("large resource set keeps DOM lane count bounded", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-virtualization");
    const preview = page.getByTestId("scheduler-timeline-virtualization-preview");
    await expect(preview).toBeVisible();

    const lanes = preview.locator("[data-testid^='scheduler-timeline-lane-']");
    const laneCount = await lanes.count();
    expect(laneCount).toBeLessThan(50);

    const scrollTimeArea = preview.locator(".orb-scheduler-timeline__time-scroll");
    await scrollIntoPreviewView(scrollTimeArea);
    const rowHeight = await preview
      .locator(".orb-scheduler-timeline__resource-cell")
      .first()
      .evaluate((el) => el.getBoundingClientRect().height);
    const target = preview.getByTestId("scheduler-resource-resource-60");

    await scrollTimeArea.hover({ force: true });

    await expect(async () => {
      await scrollTimeArea.evaluate((el, top) => {
        el.scrollTop = top;
        el.dispatchEvent(new Event("scroll", { bubbles: true }));
      }, 60 * rowHeight);
      await expect(target).toBeVisible({ timeout: 500 });
    }).toPass({ timeout: 10_000 });
  });
});
