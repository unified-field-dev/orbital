import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("combobox primitive preview", () => {

  test("CB-01: basic combobox preview renders", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await expect(page.getByTestId("combobox-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("CB-02: typing opens listbox options", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const input = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await input.fill("ap");
    await expect(page.locator(".orbital-listbox .orbital-combobox-option").first()).toBeVisible();
  });

  test("CB-03: arrow down and enter select active option", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const input = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await input.click();
    await input.press("ArrowDown");
    await expect(page.locator(".orbital-combobox-option[data-activedescendant-focusvisible]").first())
      .toBeVisible();
    await input.press("Enter");
    await expect(input).toHaveValue("Apple");
  });

  test("CB-04: clearable combobox clears current selection", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-clearable").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("combobox-clearable");
    await expect(wrapper.locator(".orbital-combobox__clear-icon")).toBeVisible();
    await wrapper.locator(".orbital-combobox__clear-icon").click();
    await expect(wrapper.locator("input[role='combobox']")).toHaveValue("");
  });

  test("CB-05: disabled combobox cannot be edited", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("combobox-disabled").locator("input[role='combobox']")).toBeDisabled();
  });

  test("CB-06: multiselect toggles selected options", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-multiselect").scrollIntoViewIfNeeded();
    const input = page.getByTestId("combobox-multiselect").locator("input[role='combobox']");
    await input.click();
    await input.press("ArrowDown");
    await input.press(" ");
    await expect(page.locator(".orbital-combobox-option--selected").first()).toBeVisible();
  });

  test("CB-07: field label is associated with combobox input", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    await page.getByTestId("combobox-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("combobox-field").locator("label").getAttribute("for");
    const inputId = await page.getByTestId("combobox-field")
      .locator("input[role='combobox']")
      .getAttribute("id");
    expect(labelFor).toEqual(inputId);
  });

  test("CB-08: typing filters options visually", async ({ page }) => {
    await openComponentPreview(page, "combobox");
    const input = page.getByTestId("combobox-preview").locator("input[role='combobox']");
    await input.fill("ban");
    await expect(
      page.locator(".orbital-combobox-option:not(.orbital-combobox-option--hidden)", { hasText: "Apple" }),
    ).toHaveCount(0);
    await expect(
      page.locator(".orbital-combobox-option:not(.orbital-combobox-option--hidden)", { hasText: "Banana" }),
    ).toBeVisible();
  });
});
