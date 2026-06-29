import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table typed sort", () => {
  test("number column sorts numerically", async ({ page }) => {
    await openComponentPreview(page, "data-table-column-definition", "data-table-typed-sort-preview");
    const preview = page.getByTestId("data-table-typed-sort-preview");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByText("Amount").click();
    await expect(preview.getByTestId("data-table-sort-asc")).toBeVisible();
    await expect(preview.locator("tbody tr").first()).toContainText("2");
    await expect(preview.locator("tbody tr").last()).toContainText("10");
  });
});
