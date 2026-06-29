import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("loading-bar primitive preview", () => {

  test("LB-01: start button shows teleported loading bar", async ({ page }) => {
    await openComponentPreview(page, "loading-bar", "loading-bar-preview");
    await page.getByTestId("loading-bar-preview").getByRole("button", { name: "Start loading" }).click();
    const bar = page.locator(".orbital-loading-bar-container").filter({ visible: true });
    await expect(bar).toBeVisible();
    await expect(bar.locator(".orbital-loading-bar")).toBeVisible();
  });

  test("LB-02: finish flow completes bar to full width", async ({ page }) => {
    await openComponentPreview(page, "loading-bar", "loading-bar-finish");
    await page.getByTestId("loading-bar-finish").getByRole("button", { name: "Start" }).click();
    await expect(page.locator(".orbital-loading-bar-container").filter({ visible: true })).toBeVisible();
    await page.getByTestId("loading-bar-finish").getByRole("button", { name: "Finish" }).click();
    const bar = page.locator(".orbital-loading-bar-container").filter({ visible: true }).locator(".orbital-loading-bar");
    await expect(bar).toHaveCSS("max-width", "100%");
  });

  test("LB-03: error flow uses danger semantic color", async ({ page }) => {
    await openComponentPreview(page, "loading-bar", "loading-bar-error");
    await page.getByTestId("loading-bar-error").getByRole("button", { name: "Trigger error" }).click();
    const bar = page.locator(".orbital-loading-bar-container").filter({ visible: true }).locator(".orbital-loading-bar");
    await expect(bar).toHaveCSS("background-color", /rgb/);
    await expect(bar).toHaveCSS("max-width", "100%");
  });
});
