import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("floating-actions-menu preview", () => {
  test("FAM-01: main trigger opens actions", async ({ page }) => {
    await openComponentPreview(page, "floating-actions-menu");
    const preview = page.getByTestId("floating-actions-menu-preview");
    const trigger = preview.getByTestId("floating-actions-menu-trigger");
    await trigger.click();
    await expect(preview.getByTestId("fam-copy")).toBeVisible();
    await expect(preview.getByTestId("fam-print")).toBeVisible();
    await expect(preview.getByTestId("fam-share")).toBeVisible();
    await preview.getByTestId("fam-copy").click({ force: true });
    await expect(preview.getByTestId("fam-copy")).toBeHidden();
  });

  test("FAM-02: custom open icon visible when expanded", async ({ page }) => {
    await openComponentPreview(page, "floating-actions-menu", "floating-actions-menu-icons");
    const preview = page.getByTestId("floating-actions-menu-icons");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByTestId("floating-actions-menu-trigger").click();
    await expect(preview.locator(".orbital-floating-actions-menu--open")).toBeVisible();
    const box = await preview.getByTestId("floating-actions-menu-trigger").boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
  });

  test("FAM-03: controlled open from external button", async ({ page }) => {
    await openComponentPreview(page, "floating-actions-menu", "floating-actions-menu-controlled");
    const preview = page.getByTestId("floating-actions-menu-controlled");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByRole("button", { name: "Open menu" }).click();
    await expect(preview.locator(".orbital-floating-actions-menu__action").first()).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(preview.locator(".orbital-floating-actions-menu__action").first()).toBeHidden();
  });

  test("FAM-04: direction variants render in matrix", async ({ page }) => {
    await openComponentPreview(page, "floating-actions-menu", "floating-actions-menu-directions");
    const preview = page.getByTestId("floating-actions-menu-directions");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.locator(".orbital-floating-actions-menu--direction-up")).toBeVisible();
    await expect(preview.locator(".orbital-floating-actions-menu--direction-right")).toBeVisible();
    const upBox = await preview.locator(".orbital-floating-actions-menu--direction-up").boundingBox();
    expect(upBox).not.toBeNull();
    expect(upBox!.width).toBeGreaterThan(0);
  });

  test("FAM-05: persistent tooltips visible on the left when open", async ({ page }) => {
    await openComponentPreview(page, "floating-actions-menu", "floating-actions-menu-tooltips-left");
    const preview = page.getByTestId("floating-actions-menu-tooltips-left");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByTestId("floating-actions-menu-trigger").click();
    const tooltip = preview.locator(".orbital-floating-actions-menu__tooltip", { hasText: "Email" });
    await expect(tooltip).toBeVisible();
    const display = await tooltip.evaluate((el) => getComputedStyle(el).display);
    expect(display).not.toBe("none");
    const item = preview.locator(".orbital-floating-actions-menu__item").first();
    const tooltipBox = await tooltip.boundingBox();
    const actionBox = await item.locator(".orbital-floating-actions-menu__action").boundingBox();
    expect(tooltipBox).not.toBeNull();
    expect(actionBox).not.toBeNull();
    expect(tooltipBox!.x + tooltipBox!.width).toBeLessThanOrEqual(actionBox!.x + 1);
  });

  test("FAM-06: persistent tooltips visible on the right when open", async ({ page }) => {
    await openComponentPreview(page, "floating-actions-menu", "floating-actions-menu-tooltips-right");
    const preview = page.getByTestId("floating-actions-menu-tooltips-right");
    await preview.scrollIntoViewIfNeeded();
    await preview.getByTestId("floating-actions-menu-trigger").click();
    const tooltip = preview.locator(".orbital-floating-actions-menu__tooltip", { hasText: "Email" });
    await expect(tooltip).toBeVisible();
    const item = preview.locator(".orbital-floating-actions-menu__item").first();
    const tooltipBox = await tooltip.boundingBox();
    const actionBox = await item.locator(".orbital-floating-actions-menu__action").boundingBox();
    expect(tooltipBox).not.toBeNull();
    expect(actionBox).not.toBeNull();
    expect(actionBox!.x + actionBox!.width).toBeLessThanOrEqual(tooltipBox!.x + 1);
  });
});
