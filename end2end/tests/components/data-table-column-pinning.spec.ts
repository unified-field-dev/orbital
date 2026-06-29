import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column pinning", () => {
  test("pinned name column stays visible on horizontal scroll", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-pinning-preview");
    const preview = page.getByTestId("data-table-column-pinning-preview");
    await preview.scrollIntoViewIfNeeded();

    const scroll = preview.getByTestId("data-table-scroll");
    const nameHeader = preview.getByTestId("data-table-header-name");
    await expect(nameHeader).toHaveClass(/orbital-data-table__pinned-left/);

    await expect
      .poll(async () => scroll.evaluate((el) => el.scrollWidth > el.clientWidth))
      .toBe(true);

    const before = await nameHeader.boundingBox();
    await scroll.evaluate((el) => {
      el.scrollLeft = 200;
    });
    const after = await nameHeader.boundingBox();

    expect(before).not.toBeNull();
    expect(after).not.toBeNull();
    expect(Math.abs(after!.x - before!.x)).toBeLessThan(2);
  });
});
