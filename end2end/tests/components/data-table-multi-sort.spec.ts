import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table multi sort", () => {
  test("ctrl-click stacks sort priorities and reorders rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-sorting-filtering", "data-table-multi-sort-preview");
    const preview = page.getByTestId("data-table-multi-sort-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.locator(".orbital-data-table__sortable", { hasText: "Role" }).click();
    await preview.locator(".orbital-data-table__sortable", { hasText: "Name" }).click({ modifiers: ["Control"] });

    await expect(preview.getByTestId("data-table-sort-asc")).toHaveCount(2);
    await expect(preview.locator("tbody tr").first()).toContainText("Ada");
    await expect(preview.locator("tbody tr").nth(1)).toContainText("Zed");
  });
});
