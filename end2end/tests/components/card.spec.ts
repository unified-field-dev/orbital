import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectComputedStyle } from "../lib/assertions/style";
test.describe("card surface preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card");
    await expect(page.getByTestId("card-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card");
    await expectPreviewVariants(page, [
      "card-preview",
      "card-compound-preview",
      "card-with-header",
      "card-media-preview",
      "card-button-area-preview",
      "card-custom-hero-preview",
      "card-raised",
      "card-outlined",
    ]);
  });

  test("default card composes material surface", async ({ page }) => {
    await openComponentPreview(page, "card");
    const root = page.getByTestId("card-preview").locator(".orbital-card.orbital-material").first();
    await expect(root).toBeVisible();
    await expect(root).toHaveAttribute("data-material-variant", "solid");
    await expect(root).toHaveAttribute("data-material-elevation", "resting");
    const shadow = await root.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(shadow).not.toBe("none");
  });

  test("raised card uses material elevated shadow", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-raised").scrollIntoViewIfNeeded();
    const defaultRoot = page.getByTestId("card-preview").locator(".orbital-material").first();
    const raisedRoot = page.getByTestId("card-raised").locator(".orbital-material").first();
    const defaultShadow = await defaultRoot.evaluate((el) => getComputedStyle(el).boxShadow);
    const raisedShadow = await raisedRoot.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(raisedShadow).not.toEqual(defaultShadow);
    await expect(raisedRoot).toHaveAttribute("data-material-elevation", "raised");
  });

  test("compound card contains all slot regions", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-compound-preview").scrollIntoViewIfNeeded();
    const compound = page.getByTestId("card-compound-preview");
    await expect(compound.locator(".orbital-card-header")).toBeVisible();
    await expect(compound.locator("img.orbital-card-media")).toBeVisible();
    await expect(compound.locator(".orbital-card-content")).toBeVisible();
    await expect(compound.locator(".orbital-card-footer")).toBeVisible();
  });

  test("media card shows image at expected height", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-media-preview").scrollIntoViewIfNeeded();
    const img = page.getByTestId("card-media-preview").locator(".orbital-card-media");
    await expect(img).toBeVisible();
    const height = await img.evaluate((el) => getComputedStyle(el).height);
    expect(parseFloat(height)).toBeCloseTo(140, 0);
    await expect(page.getByTestId("card-media-preview").locator(".orbital-card-content")).toBeVisible();
    await expect(page.getByTestId("card-media-preview").locator(".orbital-card-footer")).toBeVisible();
  });

  test("outlined card has border and no shadow", async ({ page }) => {
    await openComponentPreview(page, "card");
    await page.getByTestId("card-outlined").scrollIntoViewIfNeeded();
    const root = page.getByTestId("card-outlined").locator(".orbital-material").first();
    await expect(root).toHaveAttribute("data-material-variant", "outlined");
    await expectComputedStyle(page, "card-outlined", { "box-shadow": "none" }, {
      childSelector: ".orbital-material",
    });
    const borderWidth = await root.evaluate((el) => getComputedStyle(el).borderTopWidth);
    expect(parseFloat(borderWidth)).toBeGreaterThan(0);
  });
});
