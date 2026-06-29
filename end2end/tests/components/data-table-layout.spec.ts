import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table layout and sticky header", () => {
  test("sticky header stays visible while scrolling body", async ({ page }) => {
    await openComponentPreview(page, "data-table", "data-table-layout-preview");
    const preview = page.getByTestId("data-table-layout-preview");
    await preview.scrollIntoViewIfNeeded();

    const scroll = preview.getByTestId("data-table-scroll");
    const headerCell = preview.locator("thead th").first();
    await expect(headerCell).toBeVisible();

    const before = await headerCell.boundingBox();
    await scroll.evaluate((el) => {
      el.scrollTop = 300;
    });
    const after = await headerCell.boundingBox();

    expect(before).not.toBeNull();
    expect(after).not.toBeNull();
    expect(Math.abs(after!.y - before!.y)).toBeLessThan(5);
  });
});
