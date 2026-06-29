import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
const VARIANTS = [
  "app-bar-preview",
  "app-bar-with-menu",
  "app-bar-with-nav",
  "app-bar-with-search",
  "app-bar-search-first",
  "app-bar-compact",
  "app-bar-expanded",
  "app-bar-shell-flat",
  "app-bar-raised",
  "app-bar-sticky",
  "app-bar-fixed",
];

test.describe("app-bar preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
    await expectPreviewVariants(page, VARIANTS.slice(1));
  });

  test("standard shell has app bar regions", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
    const bar = page.getByTestId("app-bar-preview").locator("[data-testid='app-bar']");
    await expect(bar).toBeVisible();
    await expect(bar.locator(".orbital-app-bar__leading")).toBeVisible();
    await expect(bar.locator(".orbital-app-bar__trailing")).toBeVisible();
  });

  test("shell flat material attrs", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
    await page.getByTestId("app-bar-shell-flat").scrollIntoViewIfNeeded();
    const material = page
      .getByTestId("app-bar-shell-flat")
      .locator(".orbital-app-bar__material");
    await expect(material).toHaveAttribute("data-material-variant", "shell");
    await expect(material).toHaveAttribute("data-material-elevation", "flat");
  });

  test("raised elevation applies shadow", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
    await page.getByTestId("app-bar-raised").scrollIntoViewIfNeeded();
    const material = page
      .getByTestId("app-bar-raised")
      .locator(".orbital-app-bar__material");
    await expect(material).toHaveAttribute("data-material-elevation", "raised");
    const shadow = await material.evaluate((el) => getComputedStyle(el).boxShadow);
    expect(shadow).not.toBe("none");
  });

  test("density heights", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
    await page.getByTestId("app-bar-compact").scrollIntoViewIfNeeded();
    const compactRow = page.getByTestId("app-bar-compact").locator(".orbital-app-bar__row");
    await expect(compactRow).toHaveCSS("height", "48px");

    await page.getByTestId("app-bar-expanded").scrollIntoViewIfNeeded();
    const expandedRow = page.getByTestId("app-bar-expanded").locator(".orbital-app-bar__row");
    await expect(expandedRow).toHaveCSS("height", "96px");

    const standardRow = page.getByTestId("app-bar-preview").locator(".orbital-app-bar__row");
    await expect(standardRow).toHaveCSS("height", "56px");
  });

  test("position attrs on sticky and fixed", async ({ page }) => {
    await openComponentPreview(page, "app-bar");
    await page.getByTestId("app-bar-sticky").scrollIntoViewIfNeeded();
    const sticky = page.getByTestId("app-bar-sticky").locator("[data-testid='app-bar']");
    await expect(sticky).toHaveAttribute("data-app-bar-position", "sticky");

    await page.getByTestId("app-bar-fixed").scrollIntoViewIfNeeded();
    const fixed = page.getByTestId("app-bar-fixed").locator("[data-testid='app-bar']");
    await expect(fixed).toHaveAttribute("data-app-bar-position", "fixed");
  });
});
