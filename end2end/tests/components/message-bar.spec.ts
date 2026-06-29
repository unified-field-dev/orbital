import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("message-bar primitive preview", () => {

  test("MS-01: intent matrix shows all four severity rows", async ({ page }) => {
    await openComponentPreview(page, "message-bar");
    await page.getByTestId("message-bar-intents").scrollIntoViewIfNeeded();
    const intents = page.getByTestId("message-bar-intents").locator(".orbital-message-bar");
    await expect(intents).toHaveCount(4);
    await expect(intents.nth(0)).toHaveClass(/orbital-message-bar--info/);
    await expect(intents.nth(1)).toHaveClass(/orbital-message-bar--success/);
    await expect(intents.nth(2)).toHaveClass(/orbital-message-bar--warning/);
    await expect(intents.nth(3)).toHaveClass(/orbital-message-bar--error/);
    await expect(intents.first().locator(".orbital-message-bar__icon svg")).toBeVisible();
  });

  test("MS-02: with title preview renders title and body for each intent", async ({ page }) => {
    await openComponentPreview(page, "message-bar");
    await page.getByTestId("message-bar-with-title").scrollIntoViewIfNeeded();
    const bars = page.getByTestId("message-bar-with-title").locator(".orbital-message-bar");
    await expect(bars).toHaveCount(4);
    await expect(bars.nth(0)).toHaveClass(/orbital-message-bar--info/);
    await expect(bars.nth(1)).toHaveClass(/orbital-message-bar--success/);
    await expect(bars.nth(2)).toHaveClass(/orbital-message-bar--warning/);
    await expect(bars.nth(3)).toHaveClass(/orbital-message-bar--error/);
    for (const bar of await bars.all()) {
      await expect(bar.locator(".orbital-message-bar-title")).toBeVisible();
      await expect(bar.locator(".orbital-message-bar-body")).toBeVisible();
    }
  });

  test("MS-03: multiline preview uses multiline layout for each intent", async ({ page }) => {
    await openComponentPreview(page, "message-bar");
    await page.getByTestId("message-bar-multiline").scrollIntoViewIfNeeded();
    const bars = page.getByTestId("message-bar-multiline").locator(".orbital-message-bar");
    await expect(bars).toHaveCount(4);
    for (const bar of await bars.all()) {
      await expect(bar).toHaveClass(/orbital-message-bar--multiline/);
      await expect(bar.locator(".orbital-message-bar-title")).toBeVisible();
      await expect(bar.locator(".orbital-message-bar-body")).toBeVisible();
    }
    await expect(bars.nth(3)).toHaveClass(/orbital-message-bar--error/);
  });

  test("MS-05: actions preview footer buttons are clickable", async ({ page }) => {
    await openComponentPreview(page, "message-bar");
    await page.getByTestId("message-bar-actions").scrollIntoViewIfNeeded();
    const retry = page.getByTestId("message-bar-actions").getByRole("button", { name: "Retry" });
    await expect(retry).toBeVisible();
    await retry.click();
    await expect(retry).toBeEnabled();
  });

  test("MS-06: theme preview border uses computed stroke color", async ({ page }) => {
    await openComponentPreview(page, "message-bar");
    await page.getByTestId("message-bar-theme").scrollIntoViewIfNeeded();
    const borderColor = await page.getByTestId("message-bar-theme").locator(".orbital-message-bar").evaluate(
      (el) => getComputedStyle(el).borderTopColor,
    );
    expect(borderColor).toMatch(/rgb\(\d+,\s*\d+,\s*\d+\)/);
  });
});
