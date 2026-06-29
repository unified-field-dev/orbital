import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("radio behavior in radio-group previews", () => {
  test("RG-04: field wrapper associates field label and radios", async ({ page }) => {
    await openComponentPreview(page, "radio");
    await page.getByTestId("radio-group-field").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("radio-group-field");
    await expect(wrapper.locator(".orbital-field__label")).toHaveText("Digest frequency");
    const firstLabelFor = await wrapper.locator(".orbital-radio__label").first().getAttribute("for");
    const firstInputId = await wrapper.locator("input[type=radio]").first().getAttribute("id");
    expect(firstLabelFor).toEqual(firstInputId);
  });

  test("RG-05: disabled fieldset prevents changing selection", async ({ page }) => {
    await openComponentPreview(page, "radio");
    await page.getByTestId("radio-group-disabled").scrollIntoViewIfNeeded();
    const inputs = page.getByTestId("radio-group-disabled").locator("input[type=radio]");
    await expect(inputs.nth(0)).toBeChecked();
    await expect(inputs.nth(1)).toBeDisabled();
    await inputs.nth(1).click({ force: true });
    await expect(inputs.nth(0)).toBeChecked();
    await expect(inputs.nth(1)).not.toBeChecked();
  });

  test("RG-07: horizontal layout arranges radios in a row", async ({ page }) => {
    await openComponentPreview(page, "radio");
    await page.getByTestId("radio-group-horizontal").scrollIntoViewIfNeeded();
    const group = page.getByTestId("radio-group-horizontal").locator(".orbital-radio-group");
    await expect(group).toHaveClass(/orbital-radio-group--horizontal/);
    const flexDirection = await group.evaluate(
      (el) => window.getComputedStyle(el).flexDirection,
    );
    expect(flexDirection).toBe("row");
  });

  test("RG-06: keyboard navigation switches required example selection", async ({ page }) => {
    await openComponentPreview(page, "radio");
    await page.getByTestId("radio-group-required").scrollIntoViewIfNeeded();
    const wrapper = page.getByTestId("radio-group-required");
    const inputs = wrapper.locator("input[type=radio]");
    await expect(inputs.nth(0)).not.toBeChecked();
    await expect(inputs.nth(1)).not.toBeChecked();

    await inputs.nth(0).focus();
    await page.keyboard.press("ArrowDown");
    await expect(inputs.nth(1)).toBeChecked();
    await expect(inputs.nth(0)).not.toBeChecked();
  });
});
