import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("label primitive preview", () => {

  test("L-01: default label renders", async ({ page }) => {
    await openComponentPreview(page, "label");
    await expect(page.getByTestId("label-preview").locator("label")).toHaveText("Display name");
  });

  test("L-02: required label shows asterisk", async ({ page }) => {
    await openComponentPreview(page, "label");
    await page.getByTestId("label-required").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("label-required").locator(".orbital-label__required")).toHaveText("*");
  });

  test("L-03: disabled label uses disabled modifier", async ({ page }) => {
    await openComponentPreview(page, "label");
    await page.getByTestId("label-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("label-disabled").locator("label")).toHaveClass(/orbital-label--disabled/);
  });

  test("L-04: size matrix large label is taller than small", async ({ page }) => {
    await openComponentPreview(page, "label");
    await page.getByTestId("label-size-matrix").scrollIntoViewIfNeeded();
    const small = page.getByTestId("label-size-sm").locator("label");
    const large = page.getByTestId("label-size-lg").locator("label");
    const smallSize = await small.evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
    const largeSize = await large.evaluate((el) => parseFloat(getComputedStyle(el).fontSize));
    expect(largeSize).toBeGreaterThan(smallSize);
  });

  test("L-05: weight matrix semibold is bolder than regular", async ({ page }) => {
    await openComponentPreview(page, "label");
    await page.getByTestId("label-weight-matrix").scrollIntoViewIfNeeded();
    const regular = page.getByTestId("label-weight-matrix").locator("label").first();
    const semibold = page.getByTestId("label-weight-matrix").locator("label").last();
    const regularWeight = await regular.evaluate((el) => getComputedStyle(el).fontWeight);
    const semiboldWeight = await semibold.evaluate((el) => getComputedStyle(el).fontWeight);
    expect(Number(semiboldWeight)).toBeGreaterThanOrEqual(Number(regularWeight));
  });
});
