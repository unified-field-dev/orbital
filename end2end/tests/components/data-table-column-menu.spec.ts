import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column menu", () => {
  test("menu sort descending reorders rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByRole("button", { name: "Column menu" }).first().click();
    await page.getByRole("menuitem", { name: "Sort descending" }).click();

    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();
    await expect(preview.locator("tbody tr").first()).toContainText("Grace");
  });

  test("filter apply closes popover", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByRole("button", { name: "Filter column" }).first().click();
    await expect(page.getByRole("button", { name: "Apply" })).toBeVisible();

    await page.getByRole("button", { name: "Apply" }).click();
    await expect(page.getByRole("button", { name: "Apply" })).toHaveCount(0);
  });

  test("menu hide column removes header", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-menu-preview");
    const preview = page.getByTestId("data-table-column-menu-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByRole("button", { name: "Column menu" }).nth(1).click();
    await page.getByRole("menuitem", { name: "Hide column" }).click();

    await expect(preview.getByTestId("data-table-header-role")).toHaveCount(0);
  });
});
