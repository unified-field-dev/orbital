import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("rating primitive preview", () => {
  test("RT-01: default rating exposes radiogroup role", async ({ page }) => {
    await openComponentPreview(page, "rating");
    const group = page.getByTestId("rating-preview").getByRole("radiogroup");
    await expect(group).toBeVisible();
    await expect(group.locator(".orbital-rating-item")).toHaveCount(5);
  });

  test("RT-02: half-step rating selects fractional value", async ({ page }) => {
    await openComponentPreview(page, "rating");
    await page.getByTestId("rating-half").scrollIntoViewIfNeeded();
    const group = page.getByTestId("rating-half").getByRole("radiogroup");
    const thirdStarHalf = group.locator(".orbital-rating-item").nth(2).locator(".orbital-rating-item__half-value-input");
    await thirdStarHalf.click({ force: true });
    await expect(thirdStarHalf).toBeChecked();
  });

  test("RT-02b: half-step rating half star matches full star dimensions", async ({ page }) => {
    await openComponentPreview(page, "rating");
    await page.getByTestId("rating-half").scrollIntoViewIfNeeded();
    const group = page.getByTestId("rating-half").getByRole("radiogroup");
    const fullStar = group.locator(".orbital-rating-item").nth(1);
    const halfStar = group.locator(".orbital-rating-item").nth(2);
    const fullBox = await fullStar.boundingBox();
    const halfBox = await halfStar.boundingBox();
    expect(fullBox).not.toBeNull();
    expect(halfBox).not.toBeNull();
    expect(halfBox!.width).toBe(fullBox!.width);
    expect(halfBox!.height).toBe(fullBox!.height);
  });

  test("RT-03: color variants render distinct items", async ({ page }) => {
    await openComponentPreview(page, "rating");
    await expectPreviewVariants(page, ["rating-colors"]);
    await expect(page.getByTestId("rating-colors").locator(".orbital-rating-item--brand").first()).toBeVisible();
    await expect(page.getByTestId("rating-colors").locator(".orbital-rating-item--marigold").first()).toBeVisible();
    await expect(page.getByTestId("rating-colors").locator(".orbital-rating-item--neutral").first()).toBeVisible();
  });

  test("RT-04: size matrix increases item dimensions", async ({ page }) => {
    await openComponentPreview(page, "rating");
    await expectPreviewVariants(page, ["rating-sizes"]);
    const small = page.getByTestId("rating-sizes").locator(".orbital-rating-item--small").first();
    const large = page.getByTestId("rating-sizes").locator(".orbital-rating-item--large").first();
    const smallBox = await small.boundingBox();
    const largeBox = await large.boundingBox();
    expect(smallBox).not.toBeNull();
    expect(largeBox).not.toBeNull();
    expect(largeBox!.width).toBeGreaterThan(smallBox!.width);
  });

  test("RT-05: required Field composes with rating validation", async ({ page }) => {
    await openComponentPreview(page, "rating");
    await page.getByTestId("rating-validation").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("rating-validation");
    await expect(wrapper.getByText("Quality")).toBeVisible();
    const radio = wrapper.locator(".orbital-rating-item__full-value-input").first();
    await radio.click({ force: true });
    await expect(radio).toBeChecked();
    await expect(wrapper.locator(".orbital-field--error")).not.toBeVisible();
  });

  test("RT-06: custom max renders ten stars", async ({ page }) => {
    await openComponentPreview(page, "rating");
    await expectPreviewVariants(page, ["rating-max"]);
    await expect(page.getByTestId("rating-max").locator(".orbital-rating-item")).toHaveCount(10);
  });
});
