import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("data-table preview", () => {
  test("DT-01 default table renders rows", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    await expect(page.getByTestId("data-table-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("data-table-preview").getByText("Ada")).toBeVisible();
    await expect(page.getByTestId("data-table-preview").getByText("Grace")).toBeVisible();
  });

  test("DT-02 quick search narrows visible rows", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    const preview = page.getByTestId("data-table-preview");
    const search = preview.getByTestId("data-table-quick-search").locator("input");
    await search.fill("Ada");
    await expect(preview.getByText("Ada")).toBeVisible();
    await expect(preview.getByText("Grace")).toHaveCount(0);
    await expect(preview.getByTestId("data-table-footer")).toContainText("1–1 of 1");
  });

  test("DT-03 sort reorders rows", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    const preview = page.getByTestId("data-table-preview");
    await preview.getByText("Name").click();
    await expect(preview.getByTestId("data-table-sort-asc")).toBeVisible();
    await expect(preview.locator("tbody tr").first()).toContainText("Ada");
    await preview.getByText("Name").click();
    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();
    await expect(preview.locator("tbody tr").first()).toContainText("Grace");
  });

  test("DT-04 row selection", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    await expectPreviewVariants(page, ["data-table-selection"]);
    const preview = page.getByTestId("data-table-selection");
    await preview.scrollIntoViewIfNeeded();
    const firstCheckbox = preview.locator("input[type=checkbox]").first();
    await firstCheckbox.click({ force: true });
    await expect(preview.locator("tbody tr").first()).toHaveClass(/orbital-data-table__row--selected/);
  });

  test("DT-05 pagination footer uses Pagination control", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    const preview = page.getByTestId("data-table-preview");
    await expect(preview.getByTestId("data-table-footer")).toBeVisible();
    const pagination = preview.getByTestId("data-table-pagination");
    await expect(pagination.getByRole("button", { name: "1" })).toHaveClass(/orbital-button--primary/);
    const prev = pagination.locator(".orbital-pagination-item").first();
    await expect(prev).toHaveClass(/orbital-button--disabled/);
  });
});
