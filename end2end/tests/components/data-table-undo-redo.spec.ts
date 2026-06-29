import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table undo redo", () => {
  test("edit then undo restores original and redo re-applies", async ({ page }) => {
    await openComponentPreview(page, "data-table-editing", "data-table-undo-redo-preview");
    const preview = page.getByTestId("data-table-undo-redo-preview");
    await preview.scrollIntoViewIfNeeded();

    const cell = preview.getByTestId("data-table-cell-1-name");
    await expect(cell).toContainText("Alpha");

    await cell.dblclick();
    const input = preview.getByTestId("data-table-cell-1-name-input").locator("input");
    await input.fill("Changed");
    await input.press("Enter");
    await expect(cell).toContainText("Changed");

    await preview.getByTestId("data-table-undo").click();
    await expect(cell).toContainText("Alpha");

    await preview.getByTestId("data-table-redo").click();
    await expect(cell).toContainText("Changed");
  });
});
