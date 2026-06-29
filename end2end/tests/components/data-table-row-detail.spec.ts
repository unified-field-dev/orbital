import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table row detail", () => {
  test("expand and collapse detail panel", async ({ page }) => {
    await openComponentPreview(page, "data-table-rows", "data-table-row-detail-preview");
    const preview = page.getByTestId("data-table-row-detail-preview");
    await preview.scrollIntoViewIfNeeded();

    const toggle = preview.getByTestId("data-table-detail-toggle-1");
    const panel = preview.getByTestId("data-table-detail-panel-1");

    await expect(panel).toHaveCount(0);
    await toggle.click();
    await expect(panel).toBeVisible();
    await expect(panel).toContainText("Ships next week");

    await toggle.click();
    await expect(panel).toHaveCount(0);
  });
});
