import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table row grouping", () => {
  test("expanding a group reveals child data rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-grouping-preview");
    const preview = page.getByTestId("data-table-grouping-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.locator(".orbital-data-table__group-row")).toHaveCount(2);
    await expect(preview.getByTestId("data-table-row-1")).toHaveCount(0);

    const acmeToggle = preview.locator("[data-testid^='data-table-group-toggle-']").first();
    await acmeToggle.click();

    await expect(preview.getByTestId("data-table-row-1")).toBeVisible();
    await expect(preview.getByTestId("data-table-row-2")).toBeVisible();
  });
});
