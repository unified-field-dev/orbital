import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table edit validation", () => {
  test("inline validator rejects invalid score and dialog rejects blocked name", async ({ page }) => {
    await openComponentPreview(page, "data-table-editing", "data-table-edit-validation-preview");
    const preview = page.getByTestId("data-table-edit-validation-preview");
    await preview.scrollIntoViewIfNeeded();

    const scoreCell = preview.getByTestId("data-table-cell-1-score");
    await scoreCell.dblclick();

    const scoreInput = preview.getByTestId("data-table-cell-1-score-input").locator("input");
    await scoreInput.fill("150");
    await scoreInput.press("Enter");

    await expect(preview.getByTestId("data-table-edit-error-1-score")).toContainText("100");

    await scoreInput.fill("80");
    await scoreInput.press("Enter");
    await expect(scoreCell).toContainText("80");

    const nameCell = preview.getByTestId("data-table-cell-1-name");
    await nameCell.dblclick();
    const nameInput = preview.getByTestId("data-table-cell-1-name-input").locator("input");
    await nameInput.fill("Blocked");
    await nameInput.press("Enter");

    await expect(page.getByTestId("data-table-edit-error-dialog")).toBeVisible({ timeout: 10_000 });
    await page.keyboard.press("Escape");
  });
});
