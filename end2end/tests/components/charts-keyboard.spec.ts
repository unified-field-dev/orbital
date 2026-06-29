import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("charts-keyboard preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "charts-keyboard");
    await expect(page.getByTestId("charts-keyboard-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "charts-keyboard");
    await expectPreviewVariants(page, [
      "charts-keyboard-preview",
      "charts-keyboard-domain-preview",
    ]);
  });

  test("arrow key highlights a mark", async ({ page }) => {
    await openComponentPreview(page, "charts-keyboard");
    const chart = page.getByTestId("charts-keyboard-preview").locator("[data-orbital-chart]");
    await expect(chart).toBeVisible({ timeout: 30_000 });
    await expect(chart).toHaveAttribute("data-orbital-chart-marker-count", "4");
    await chart.focus();
    await page.keyboard.press("ArrowRight");

    await expect(chart).toHaveAttribute("data-orbital-chart-hovered", /revenue:0/, { timeout: 5_000 });
    await expect(chart.locator(".orb-keyboard-focus-ring").first()).toBeVisible({ timeout: 5_000 });
  });

  test("strict and nice x domains differ on horizontal line charts", async ({ page }) => {
    await openComponentPreview(page, "charts-keyboard");
    const strict = page.getByTestId("charts-keyboard-domain-strict-preview").locator("[data-orbital-chart]");
    const nice = page.getByTestId("charts-keyboard-domain-nice-preview").locator("[data-orbital-chart]");

    await expect(strict).toHaveAttribute("data-orbital-chart-x-domain-limit", "strict", { timeout: 30_000 });
    await expect(nice).toHaveAttribute("data-orbital-chart-x-domain-limit", "nice");

    const strictDomain = await strict.getAttribute("data-orbital-chart-x-domain");
    const niceDomain = await nice.getAttribute("data-orbital-chart-x-domain");
    expect(strictDomain).toBeTruthy();
    expect(niceDomain).toBeTruthy();
    expect(strictDomain).not.toEqual(niceDomain);
  });
});
