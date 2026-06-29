import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("auto-grid preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "auto-grid");
  });

  test("AG-01 renders responsive card grid", async ({ page }) => {
    await expect(page.getByText("Item 1", { exact: true })).toBeVisible();
    await expect(page.getByText("Item 2", { exact: true })).toBeVisible();
    await expect(page.getByText("Item 3", { exact: true })).toBeVisible();
  });

  test("AG-02 grid container uses auto-fill layout", async ({ page }) => {
    const grid = page.locator("div[style*='grid-template-columns']").first();
    await expect(grid).toBeVisible();
    const columns = await grid.evaluate((el) => getComputedStyle(el).gridTemplateColumns);
    expect(columns).toMatch(/\d+/);
  });
});
