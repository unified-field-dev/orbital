import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table list view", () => {
  test("renders card layout instead of table rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-list-view")).toBeVisible();
    await expect(preview.getByTestId("data-table-list-card-1")).toBeVisible();
    await expect(preview.getByTestId("data-table-list-card-2")).toBeVisible();
    await expect(preview.locator("tbody tr[data-testid^='data-table-row-']")).toHaveCount(0);
  });

  test("fires on_row_click when a list card is clicked", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-list-view-preview");
    const preview = page.getByTestId("data-table-list-view-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-list-card-1").click();
    await expect(preview.getByTestId("data-table-list-view-log")).toHaveText("row_click:1");
  });
});
