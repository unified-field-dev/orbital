import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table inline editing", () => {
  test("double-click cell, edit, and commit with Enter", async ({ page }) => {
    await openComponentPreview(page, "data-table-editing", "data-table-editing-preview");
    const preview = page.getByTestId("data-table-editing-preview");
    await preview.scrollIntoViewIfNeeded();

    const cell = preview.getByTestId("data-table-cell-1-name");
    await cell.dblclick();

    const input = preview.getByTestId("data-table-cell-1-name-input").locator("input");
    await expect(input).toBeVisible();
    await input.fill("Updated");
    await input.press("Enter");

    await expect(cell).toContainText("Updated");
  });
});
