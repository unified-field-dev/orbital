import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table row span", () => {
  test("merged qty column has fewer cells than rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-rows", "data-table-row-span-preview");
    const preview = page.getByTestId("data-table-row-span-preview");
    await preview.scrollIntoViewIfNeeded();

    const rowCount = await preview.locator("tbody tr").count();
    expect(rowCount).toBeGreaterThanOrEqual(6);

    const qtyCells = await preview.locator('tbody [data-testid^="data-table-cell-"][data-testid$="-qty"]').count();
    expect(qtyCells).toBeLessThan(rowCount);
    expect(qtyCells).toBe(3);
  });
});
