import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
function iconSvg(page: import("@playwright/test").Page, testId: string) {
  return page.getByTestId(testId).locator("svg.orbital-icon").first();
}

test.describe("icon primitive preview", () => {
  test("I-01: renders preview page", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await expect(iconSvg(page, "icon-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await expectPreviewVariants(page, [
      "icon-preview",
      "icon-size-matrix",
      "icon-color-matrix",
      "icon-style-preview",
      "icon-clickable",
      "icon-base-vs-core",
      "icon-in-button",
    ]);
  });

  test("I-02: default size inherits 1em", async ({ page }) => {
    await openComponentPreview(page, "icon");
    const svg = iconSvg(page, "icon-preview");
    const box = await svg.boundingBox();
    expect(box).toBeTruthy();
    // 1em ≈ 16px at default root font size
    expect(box!.width).toBeGreaterThan(12);
    expect(box!.width).toBeLessThan(22);
    expect(box!.height).toBeGreaterThan(12);
    expect(box!.height).toBeLessThan(22);
  });

  test("I-03: size matrix pixel dimensions", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await page.getByTestId("icon-size-matrix").scrollIntoViewIfNeeded();

    const size16 = await iconSvg(page, "icon-size-16").boundingBox();
    const size24 = await iconSvg(page, "icon-size-24").boundingBox();
    const size32 = await iconSvg(page, "icon-size-32").boundingBox();

    expect(size16!.width).toBeCloseTo(16, 0);
    expect(size16!.height).toBeCloseTo(16, 0);
    expect(size24!.width).toBeCloseTo(24, 0);
    expect(size24!.height).toBeCloseTo(24, 0);
    expect(size32!.width).toBeCloseTo(32, 0);
    expect(size32!.height).toBeCloseTo(32, 0);
  });

  test("I-04: color inheritance via currentColor", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await page.getByTestId("icon-color-matrix").scrollIntoViewIfNeeded();

    const brandParent = page.getByTestId("icon-color-brand");
    const brandIcon = iconSvg(page, "icon-color-brand");
    const parentColor = await brandParent.evaluate((el) => getComputedStyle(el).color);
    const fill = await brandIcon.evaluate((el) => getComputedStyle(el).fill);
    expect(fill).toBe(parentColor);
  });

  test("I-05: style override applies", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await page.getByTestId("icon-style-preview").scrollIntoViewIfNeeded();
    const svg = iconSvg(page, "icon-style-preview");
    const opacity = await svg.evaluate((el) => getComputedStyle(el).opacity);
    expect(parseFloat(opacity)).toBeLessThan(1);
  });

  test("I-06: clickable wrapper is accessible", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await page.getByTestId("icon-clickable").scrollIntoViewIfNeeded();
    const control = page.getByTestId("icon-clickable");
    await expect(control).toHaveAttribute("aria-label", "Search");
    await expect(iconSvg(page, "icon-clickable")).toBeVisible();
  });

  test("I-07: core Icon uses inline-block layout", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await page.getByTestId("icon-base-vs-core").scrollIntoViewIfNeeded();

    const coreDisplay = await iconSvg(page, "icon-core-cell").evaluate(
      (el) => getComputedStyle(el).display,
    );
    const baseDisplay = await iconSvg(page, "icon-base-cell").evaluate(
      (el) => getComputedStyle(el).display,
    );
    expect(coreDisplay).toBe("inline-block");
    expect(baseDisplay).not.toBe("inline-block");
  });

  test("I-08: button composition shows icon", async ({ page }) => {
    await openComponentPreview(page, "icon");
    await page.getByTestId("icon-in-button").scrollIntoViewIfNeeded();
    const button = page.getByTestId("icon-in-button").getByRole("button", { name: "Save" });
    await expect(button).toBeVisible();
    await expect(button.locator("svg.orbital-icon")).toBeVisible();
  });
});
