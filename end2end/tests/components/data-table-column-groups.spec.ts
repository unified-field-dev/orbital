import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column groups", () => {
  test("group header row renders above leaf headers", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-groups-preview");
    const preview = page.getByTestId("data-table-column-groups-preview");
    await preview.scrollIntoViewIfNeeded();

    const groupRow = preview.locator(".orbital-data-table__group-header-row");
    await expect(groupRow).toBeVisible();
    await expect(groupRow.getByText("Personal")).toBeVisible();
    await expect(groupRow.getByText("Work")).toBeVisible();
    await expect(preview.getByTestId("data-table-header-name")).toBeVisible();
  });
});
