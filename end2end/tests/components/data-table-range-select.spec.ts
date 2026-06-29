import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table range selection", () => {
  test("shift-click selects contiguous rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-selection", "data-table-range-select-preview");
    const preview = page.getByTestId("data-table-range-select-preview");
    await preview.scrollIntoViewIfNeeded();

    const firstCheckbox = preview.locator("input[type=checkbox]").first();
    await firstCheckbox.click({ force: true });
    await expect(preview.getByTestId("data-table-row-1")).toHaveClass(/orbital-data-table__row--selected/);

    const fourthCheckbox = preview.locator("input[type=checkbox]").nth(3);
    await fourthCheckbox.click({ modifiers: ["Shift"], force: true });

    for (const id of ["1", "2", "3", "4"]) {
      await expect(preview.getByTestId(`data-table-row-${id}`)).toHaveClass(/orbital-data-table__row--selected/);
    }
    await expect(preview.getByTestId("data-table-row-5")).not.toHaveClass(/orbital-data-table__row--selected/);
  });
});
