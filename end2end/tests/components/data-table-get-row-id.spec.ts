import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table get row id", () => {
  test("custom get_row_id drives row testids", async ({ page }) => {
    await openComponentPreview(page, "data-table-rows", "data-table-get-row-id-preview");
    const preview = page.getByTestId("data-table-get-row-id-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-row-SKU-42")).toBeVisible();
    await expect(preview.getByTestId("data-table-row-SKU-99")).toBeVisible();
    await expect(preview.getByTestId("data-table-row-rec-1")).toHaveCount(0);
  });
});
