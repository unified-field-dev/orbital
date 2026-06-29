import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table cell range selection", () => {
  test("shift-click selects a rectangular cell range", async ({ page }) => {
    await openComponentPreview(page, "data-table-selection", "data-table-range-selection-preview");
    const preview = page.getByTestId("data-table-range-selection-preview");
    await preview.scrollIntoViewIfNeeded();

    const start = preview.getByTestId("data-table-cell-1-name");
    const end = preview.getByTestId("data-table-cell-2-role");

    await start.click();
    await end.click({ modifiers: ["Shift"] });

    await expect(start).toHaveClass(/orbital-data-table__cell--range-selected/);
    await expect(end).toHaveClass(/orbital-data-table__cell--range-selected/);
    await expect(preview.getByTestId("data-table-cell-1-role")).toHaveClass(
      /orbital-data-table__cell--range-selected/,
    );
    await expect(preview.getByTestId("data-table-cell-2-name")).toHaveClass(
      /orbital-data-table__cell--range-selected/,
    );
  });

  test("arrow keys move the focused cell", async ({ page }) => {
    await openComponentPreview(page, "data-table-selection", "data-table-range-selection-preview");
    const preview = page.getByTestId("data-table-range-selection-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-cell-1-name").click();
    await preview.getByTestId("data-table-grid-focus").focus();
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("ArrowDown");

    await expect(preview.getByTestId("data-table-cell-2-role")).toHaveClass(
      /orbital-data-table__cell--range-focus/,
    );
  });
});
