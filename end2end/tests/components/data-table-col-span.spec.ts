import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table col span", () => {
  test("summary cell spans two columns", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-col-span-preview");
    const preview = page.getByTestId("data-table-col-span-preview");
    await preview.scrollIntoViewIfNeeded();

    const row = preview.locator("tbody tr").first();
    await expect(row.locator("td")).toHaveCount(2);
    await expect(row.locator("td").first()).toContainText("Merged summary cell");
  });
});
