import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("button primitive preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "button");
    await expect(page.getByTestId("button-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows variant examples", async ({ page }) => {
    await openComponentPreview(page, "button");
    await expectPreviewVariants(page, ["button-secondary", "button-shapes", "button-disabled"]);
  });

  test("disabled button has disabled attribute", async ({ page }) => {
    await openComponentPreview(page, "button");
    const btn = page.getByTestId("button-disabled").getByRole("button").first();
    await expect(btn).toBeDisabled();
  });

  test("loading button exposes aria-busy", async ({ page }) => {
    await openComponentPreview(page, "button");
    await expect(page.getByTestId("button-loading").getByRole("button"))
      .toHaveAttribute("aria-busy", "true");
  });

  test("primary appearance uses brand background token", async ({ page }) => {
    await openComponentPreview(page, "button");
    await page.getByTestId("button-theme-brand").scrollIntoViewIfNeeded();
    const btn = page.getByTestId("button-theme-brand").getByRole("button");
    const bg = await btn.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(bg).toMatch(/rgb\(227,\s*0,\s*140\)|rgb\(227, 0, 140\)/);
  });

  test("secondary appearance differs from primary", async ({ page }) => {
    await openComponentPreview(page, "button");
    const primary = page.getByTestId("button-preview").getByRole("button");
    await page.getByTestId("button-secondary").scrollIntoViewIfNeeded();
    const secondary = page.getByTestId("button-secondary").getByRole("button");
    const primaryBg = await primary.evaluate((el) => getComputedStyle(el).backgroundColor);
    const secondaryBg = await secondary.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(primaryBg).not.toEqual(secondaryBg);
  });
});
