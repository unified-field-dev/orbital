import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column visibility", () => {
  test("initial state hides role column", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-visibility-preview");
    const preview = page.getByTestId("data-table-column-visibility-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-header-name")).toBeVisible();
    await expect(preview.getByTestId("data-table-header-role")).toHaveCount(0);
  });
});
