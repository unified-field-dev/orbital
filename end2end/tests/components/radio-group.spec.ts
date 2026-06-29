import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("radio-group primitive behavior", () => {
  test("RG-01: default group has exactly one checked option", async ({ page }) => {
    await openComponentPreview(page, "radio");
    const inputs = page.getByTestId("radio-group-preview").locator("input[type=radio]");
    await expect(inputs).toHaveCount(2);
    await expect(inputs.nth(0)).toBeChecked();
    await expect(inputs.nth(1)).not.toBeChecked();
  });

  test("RG-02: unselected example starts with no checked radio", async ({ page }) => {
    await openComponentPreview(page, "radio");
    await page.getByTestId("radio-group-unselected").scrollIntoViewIfNeeded();
    const inputs = page.getByTestId("radio-group-unselected").locator("input[type=radio]");
    await expect(inputs).toHaveCount(2);
    await expect(inputs.nth(0)).not.toBeChecked();
    await expect(inputs.nth(1)).not.toBeChecked();
  });

  test("RG-03: clicking another option moves checked state", async ({ page }) => {
    await openComponentPreview(page, "radio");
    await page.getByTestId("radio-group-click").scrollIntoViewIfNeeded();
    const inputs = page.getByTestId("radio-group-click").locator("input[type=radio]");
    await expect(inputs.nth(0)).toBeChecked();
    await inputs.nth(1).click({ force: true });
    await expect(inputs.nth(0)).not.toBeChecked();
    await expect(inputs.nth(1)).toBeChecked();
  });
});
