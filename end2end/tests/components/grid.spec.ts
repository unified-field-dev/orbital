import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectGridColumnCount } from "../lib/assertions/layout";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
const gridItem = (page: import("@playwright/test").Page, testId: string) =>
  page.getByTestId(testId).locator("xpath=ancestor::*[contains(@class,'orbital-grid-item')]").first();

test.describe("grid primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "grid");
  });

  test("GR-01 three-column grid layout", async ({ page }) => {
    const grid = page.getByTestId("grid-preview").locator(".orbital-grid");
    await expectGridColumnCount(grid, 3);
    await expect(page.getByTestId("grid-cell-1")).toBeVisible();
    await expect(page.getByTestId("grid-cell-2")).toBeVisible();
    await expect(page.getByTestId("grid-cell-3")).toBeVisible();
  });

  test("GR-02 two columns with distinct gaps", async ({ page }) => {
    await expectPreviewVariants(page, ["grid-two-col"]);
    const grid = page.getByTestId("grid-two-col").locator(".orbital-grid");
    await expectGridColumnCount(grid, 2);
    const columnGap = await grid.evaluate((el) => getComputedStyle(el).columnGap);
    const rowGap = await grid.evaluate((el) => getComputedStyle(el).rowGap);
    expect(columnGap).not.toEqual(rowGap);
  });

  test("GR-03 spanning columns", async ({ page }) => {
    await expectPreviewVariants(page, ["grid-span"]);
    await expect(gridItem(page, "grid-wide")).toHaveCSS("grid-column", /span 2/);
    await expect(gridItem(page, "grid-full")).toHaveCSS("grid-column", /span 3/);
  });

  test("GR-04 offset item", async ({ page }) => {
    await expectPreviewVariants(page, ["grid-offset"]);
    const item = gridItem(page, "grid-offset-cell");
    await expect(page.getByTestId("grid-offset-cell")).toBeVisible();
    const marginLeft = await item.evaluate((el) => getComputedStyle(el).marginLeft);
    expect(marginLeft).not.toBe("0px");
  });

  test("GR-05 theme token on cell", async ({ page }) => {
    await expectPreviewVariants(page, ["grid-theme"]);
    await expectNonEmptyResolvedStyle(page, "grid-theme-cell", "border-color");
  });

  test("GR-06 form columns with Field", async ({ page }) => {
    await expectPreviewVariants(page, ["grid-form"]);
    await expect(page.getByText("First name")).toBeVisible();
    await expect(page.getByTestId("grid-form").getByText("Last name")).toBeVisible();
  });

  test("GR-07 single column stack", async ({ page }) => {
    await expectPreviewVariants(page, ["grid-single-col"]);
    const grid = page.getByTestId("grid-single-col").locator(".orbital-grid");
    await expectGridColumnCount(grid, 1);
    await expect(page.getByTestId("grid-single-1")).toBeVisible();
    await expect(page.getByTestId("grid-single-2")).toBeVisible();
  });
});
