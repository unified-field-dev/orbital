import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table virtualization", () => {
  test("large dataset keeps DOM row count small and scrolls to mid-list", async ({ page }) => {
    await openComponentPreview(page, "data-table-rendering", "data-table-virtualization-preview");
    const preview = page.getByTestId("data-table-virtualization-preview");
    await preview.scrollIntoViewIfNeeded();

    const dataRows = preview.locator("tbody tr:not(.orbital-data-table__virtual-spacer)");
    const rowCount = await dataRows.count();
    expect(rowCount).toBeLessThan(50);

    const scroll = preview.getByTestId("data-table-scroll");
    await scroll.evaluate((el) => {
      el.scrollTop = 200_000;
    });

    await expect(preview.getByTestId("data-table-row-5000")).toBeVisible({ timeout: 10_000 });
    await expect(preview.getByTestId("data-table-row-0")).toHaveCount(0);
  });
});
