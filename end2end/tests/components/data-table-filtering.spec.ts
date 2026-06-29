import { test, expect } from "@playwright/test";
import { fillPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table filtering", () => {
  test("filter panel greater-than operator reduces rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-filtering-preview");
    const preview = page.getByTestId("data-table-filtering-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await expect(panel).toBeVisible();

    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "score");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "greater_than");
    await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "20");
    await panel.getByRole("button", { name: "Apply" }).click();

    await expect(preview.locator("tbody tr")).toHaveCount(1);
    await expect(preview.locator("tbody tr").first()).toContainText("Beta");
  });
});
