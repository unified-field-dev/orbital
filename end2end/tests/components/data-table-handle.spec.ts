import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table programmatic handle", () => {
  test("handle sort, filter, search, and scroll", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-handle-preview");
    const preview = page.getByTestId("data-table-handle-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("handle-sort-name").click();
    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();
    await expect(preview.locator("tbody tr").first()).toContainText("Row 9");

    const scroll = preview.getByTestId("data-table-scroll");
    await scroll.evaluate((el) => {
      el.scrollTop = 0;
    });
    await preview.getByTestId("handle-scroll-row-25").click();
    await expect(preview.getByTestId("data-table-row-25")).toBeVisible({ timeout: 10_000 });

    await expect
      .poll(async () => scroll.evaluate((el) => el.scrollTop), { timeout: 20_000 })
      .toBeGreaterThan(0);

    await preview.getByTestId("handle-filter-admin").click();
    await expect(preview.locator("tbody tr")).toHaveCount(11);

    await preview.getByTestId("handle-search-ada").click();
    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.getByText("Row 5")).toBeVisible();
  });
});
