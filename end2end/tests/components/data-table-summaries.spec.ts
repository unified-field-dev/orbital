import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table summaries", () => {
  test("footer shows aggregate sum for amount column", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-summaries-preview");
    const preview = page.getByTestId("data-table-summaries-preview");
    await preview.scrollIntoViewIfNeeded();

    const footer = preview.getByTestId("data-table-aggregation-footer");
    await expect(footer).toBeVisible();
    await expect(footer).toContainText("1200");
  });
});
