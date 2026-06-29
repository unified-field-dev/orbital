import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("switch primitive preview", () => {

  test("SW-01: on switch is checked", async ({ page }) => {
    await openComponentPreview(page, "switch");
    await expect(page.getByTestId("switch-preview").locator("input[role=switch]")).toBeChecked();
  });

  test("SW-02: off switch is unchecked", async ({ page }) => {
    await openComponentPreview(page, "switch");
    await page.getByTestId("switch-off").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("switch-off").locator("input[role=switch]")).not.toBeChecked();
  });

  test("SW-03: disabled switch has disabled attribute", async ({ page }) => {
    await openComponentPreview(page, "switch");
    await page.getByTestId("switch-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("switch-disabled").locator("input[role=switch]")).toBeDisabled();
  });

  test("SW-04: field wrapper shows field label", async ({ page }) => {
    await openComponentPreview(page, "switch");
    await page.getByTestId("switch-field").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("switch-field").locator(".orbital-field__label")).toHaveText("Email alerts");
    const labelFor = await page.getByTestId("switch-field").locator(".orbital-switch__label").getAttribute("for");
    const inputId = await page.getByTestId("switch-field").locator("input[role=switch]").getAttribute("id");
    expect(labelFor).toEqual(inputId);
  });

  test("SW-05: keyboard Space toggles switch-keyboard example", async ({ page }) => {
    await openComponentPreview(page, "switch");
    await page.getByTestId("switch-keyboard").scrollIntoViewIfNeeded();
    const input = page.getByTestId("switch-keyboard").locator("input[role=switch]");
    await expect(input).not.toBeChecked();
    await input.focus();
    await page.keyboard.press("Space");
    await expect(input).toBeChecked();
  });
});
