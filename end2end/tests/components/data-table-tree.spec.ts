import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table tree data", () => {
  test("expanding a branch reveals nested rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-tree-preview");
    const preview = page.getByTestId("data-table-tree-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-row-1")).toBeVisible();
    await expect(preview.getByTestId("data-table-row-2")).toHaveCount(0);

    await preview.getByTestId("data-table-tree-toggle-Org").click();

    await expect(preview.getByTestId("data-table-row-2")).toBeVisible();
  });
});
