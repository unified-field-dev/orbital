import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table row pinning", () => {
  test("pinned top row stays visible on vertical scroll", async ({ page }) => {
    await openComponentPreview(page, "data-table-rows", "data-table-row-pinning-preview");
    const preview = page.getByTestId("data-table-row-pinning-preview");
    await preview.scrollIntoViewIfNeeded();

    const scroll = preview.getByTestId("data-table-scroll");
    const pinnedRow = preview.locator("tr.orbital-data-table__row--pinned-top");
    const pinnedCell = pinnedRow.locator("td").first();
    await expect(pinnedRow).toBeVisible();

    const before = await pinnedCell.boundingBox();
    await scroll.evaluate((el) => {
      el.scrollTop = 200;
    });
    const after = await pinnedCell.boundingBox();

    expect(before).not.toBeNull();
    expect(after).not.toBeNull();
    expect(Math.abs(after!.y - before!.y)).toBeLessThan(5);
  });
});
