import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("rating-display primitive preview", () => {
  test("RD-01: default display exposes img role and value text", async ({ page }) => {
    await openComponentPreview(page, "rating-display");
    const display = page.getByTestId("rating-display-preview");
    await expect(display.getByRole("img")).toBeVisible();
    await expect(display.locator(".orbital-rating-display__value-text")).toHaveText("4");
    await expect(display.locator(".orbital-rating-item")).toHaveCount(5);
    await expect(display.locator("input[type='radio']")).toHaveCount(0);
  });

  test("RD-02: size matrix increases item dimensions", async ({ page }) => {
    await openComponentPreview(page, "rating-display");
    await expectPreviewVariants(page, ["rating-display-sizes"]);
    const small = page.getByTestId("rating-display-sizes").locator(".orbital-rating-item--small").first();
    const large = page.getByTestId("rating-display-sizes").locator(".orbital-rating-item--large").first();
    const smallBox = await small.boundingBox();
    const largeBox = await large.boundingBox();
    expect(smallBox).not.toBeNull();
    expect(largeBox).not.toBeNull();
    expect(largeBox!.width).toBeGreaterThan(smallBox!.width);
  });

  test("RD-03: color variants apply modifier classes", async ({ page }) => {
    await openComponentPreview(page, "rating-display");
    await expectPreviewVariants(page, ["rating-display-colors"]);
    await expect(page.getByTestId("rating-display-colors").locator(".orbital-rating-item--brand").first()).toBeVisible();
    await expect(page.getByTestId("rating-display-colors").locator(".orbital-rating-item--marigold").first()).toBeVisible();
    await expect(page.getByTestId("rating-display-colors").locator(".orbital-rating-item--neutral").first()).toBeVisible();
  });

  test("RD-04: half value matches full star dimensions", async ({ page }) => {
    await openComponentPreview(page, "rating-display");
    await expectPreviewVariants(page, ["rating-display-half"]);
    const display = page.getByTestId("rating-display-half");
    await expect(display.locator(".orbital-rating-display__value-text")).toHaveText("3.5");
    const fullStar = display.locator(".orbital-rating-item").nth(2);
    const halfStar = display.locator(".orbital-rating-item").nth(3);
    const fullBox = await fullStar.boundingBox();
    const halfBox = await halfStar.boundingBox();
    expect(fullBox).not.toBeNull();
    expect(halfBox).not.toBeNull();
    expect(halfBox!.width).toBe(fullBox!.width);
    expect(halfBox!.height).toBe(fullBox!.height);
  });

  test("RD-05: custom max renders ten stars", async ({ page }) => {
    await openComponentPreview(page, "rating-display");
    await expectPreviewVariants(page, ["rating-display-max"]);
    await expect(page.getByTestId("rating-display-max").locator(".orbital-rating-item")).toHaveCount(10);
    await expect(page.getByTestId("rating-display-max").locator(".orbital-rating-display__value-text")).toHaveText("8");
  });
});
