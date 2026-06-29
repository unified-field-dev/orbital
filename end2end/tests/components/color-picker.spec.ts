import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("color-picker primitive preview", () => {
  test("CP-01: option bind opens picker panel", async ({ page }) => {
    await openComponentPreview(page, "color-picker");
    await page.getByTestId("CP-01").getByRole("button").first().click();
    await expect(page.locator(".orbital-color-picker-panel").last()).toBeVisible();
  });

  test("CP-02: form bind updates color value via hue slider", async ({ page }) => {
    await openComponentPreview(page, "color-picker");
    const wrapper = page.getByTestId("CP-02");
    await wrapper.getByRole("button").first().click();
    const panel = page.locator(".orbital-color-picker-panel").last();
    await panel.locator(".orbital-color-picker-panel__hue").fill("180");
    await expect(wrapper.locator(".orbital-color-picker__label")).not.toHaveText("#000000");
  });

  test("CP-03: preselected color is shown in trigger label", async ({ page }) => {
    await openComponentPreview(page, "color-picker");
    await expect(page.getByTestId("CP-03").locator(".orbital-color-picker__label")).toHaveText("#3380E6");
  });

  test("CP-04: disabled picker prevents interaction", async ({ page }) => {
    await openComponentPreview(page, "color-picker");
    await expect(page.getByTestId("CP-04").getByRole("button").first()).toBeDisabled();
  });

  test("CP-05: field label associates with trigger id", async ({ page }) => {
    await openComponentPreview(page, "color-picker");
    const wrapper = page.getByTestId("CP-05");
    const labelFor = await wrapper.locator("label").getAttribute("for");
    const triggerId = await wrapper.getByRole("button").first().getAttribute("id");
    expect(labelFor).toEqual(triggerId);
  });
});
