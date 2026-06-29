import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("stack primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "stack");
  });

  test("ST-01 renders default horizontal cluster", async ({ page }) => {
    await expect(page.getByTestId("stack-item-1")).toBeVisible();
    await expect(page.getByTestId("stack-item-2")).toBeVisible();
    const stack = page.getByTestId("stack-preview").locator(".orbital-flex");
    await expect(stack).toHaveCSS("flex-direction", "row");
  });

  test("ST-02 vertical stack", async ({ page }) => {
    await expectPreviewVariants(page, ["stack-vertical"]);
    const stack = page.getByTestId("stack-vertical").locator(".orbital-flex");
    await expect(stack).toHaveCSS("flex-direction", "column");
  });

  test("ST-03 gap matrix varies spacing", async ({ page }) => {
    await expectPreviewVariants(page, ["stack-gap-matrix"]);
    const rows = page.getByTestId("stack-gap-matrix").locator(".orbital-flex");
    const smallGap = await rows.first().evaluate((el) => getComputedStyle(el).gap);
    const largeGap = await rows.last().evaluate((el) => getComputedStyle(el).gap);
    expect(smallGap).not.toEqual(largeGap);
  });
});
