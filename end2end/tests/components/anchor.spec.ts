import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("anchor preview", () => {
  test("AN-01 basic rail", async ({ page }) => {
    await openComponentPreview(page, "anchor");
    const rail = page.getByTestId("anchor-preview").locator(".orbital-anchor");
    await expect(rail).toBeVisible();
    await expect(page.getByTestId("anchor-link-1")).toBeVisible();
    await expect(page.getByTestId("anchor-link-2")).toBeVisible();
    await expect(page.getByTestId("anchor-link-3")).toBeVisible();
  });

  test("AN-02 nested links", async ({ page }) => {
    await openComponentPreview(page, "anchor");
    await expectPreviewVariants(page, ["anchor-nested"]);
    await expect(page.getByTestId("anchor-nested").locator(".orbital-anchor-link")).toHaveCount(3);
  });

  test("AN-03 custom scroll container", async ({ page }) => {
    await openComponentPreview(page, "anchor");
    await expectPreviewVariants(page, ["anchor-offset"]);
    await expect(page.locator("#anchor-scroll")).toBeVisible();
  });

  test("AN-04 active on scroll", async ({ page }) => {
    await openComponentPreview(page, "anchor");
    await expectPreviewVariants(page, ["anchor-active"]);
    const scroll = page.getByTestId("anchor-active-scroll");
    await scroll.evaluate((el) => {
      el.scrollTop = el.scrollHeight;
    });
    await expect(
      page.getByTestId("anchor-active").locator(".orbital-anchor-link--active"),
    ).toBeVisible({ timeout: 5000 });
  });

  test("AN-05 layout composition", async ({ page }) => {
    await openComponentPreview(page, "anchor");
    await expectPreviewVariants(page, ["anchor-layout"]);
    await expect(page.locator("#setup")).toBeVisible();
    await expect(page.locator("#deploy")).toBeVisible();
  });
});
