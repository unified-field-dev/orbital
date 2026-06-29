import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column picker", () => {
  test("picker toggles column visibility", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-picker-preview");
    const preview = page.getByTestId("data-table-column-picker-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-header-email")).toBeVisible();

    await preview.getByTestId("data-table-column-picker-trigger").click();
    await expect(page.getByTestId("data-table-column-picker-panel")).toBeVisible();
    await expect(page.getByTestId("data-table-column-picker-panel")).toContainText(
      "Show and reorder columns",
    );

    const emailToggle = page.getByTestId("data-table-column-picker-email");
    await emailToggle.locator("input[type=checkbox]").click();

    await expect(preview.getByTestId("data-table-header-email")).toHaveCount(0);
    await expect(preview.getByTestId("data-table-header-name")).toBeVisible();

    // Hidden columns stay in the picker so they can be shown again.
    await expect(page.getByTestId("data-table-column-picker-email")).toBeVisible();
    await expect(
      page.getByTestId("data-table-column-picker-email").locator("input[type=checkbox]"),
    ).not.toBeChecked();

    await page.getByTestId("data-table-column-picker-email").locator("input[type=checkbox]").click();
    await expect(preview.getByTestId("data-table-header-email")).toBeVisible();
  });
});
