import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
const VARIANTS = [
  "navigation-preview",
  "navigation-categories",
  "navigation-sections",
  "navigation-app-item",
  "navigation-links",
  "navigation-target-blank",
  "navigation-density-compact",
  "navigation-collapsible",
];

test.describe("navigation primitive preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "navigation");
    await expect(page.getByTestId("navigation-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "navigation");
    await expectPreviewVariants(page, VARIANTS);
  });

  test("compact density rows are not double-padded", async ({ page }) => {
    await openComponentPreview(page, "navigation");
    const compact = page.getByTestId("navigation-density-compact");
    const item = compact.getByRole("button", { name: "Compact row" });
    await expect(item).toBeVisible();

    const itemHeight = await item.evaluate((el) => el.getBoundingClientRect().height);
    expect(itemHeight).toBeGreaterThan(20);
    expect(itemHeight).toBeLessThan(40);
  });

  test("link rows are not taller than category headers", async ({ page }) => {
    await openComponentPreview(page, "navigation");
    const categories = page.getByTestId("navigation-categories");
    const categoryHeader = categories.getByRole("button", { name: "Tools" });
    const subItem = categories.getByRole("button", { name: "Tool A" });
    await expect(categoryHeader).toBeVisible();
    await expect(subItem).toBeVisible();

    const headerHeight = await categoryHeader.evaluate((el) => el.getBoundingClientRect().height);
    const itemHeight = await subItem.evaluate((el) => el.getBoundingClientRect().height);
    expect(itemHeight).toBeLessThanOrEqual(headerHeight + 2);
  });
});
