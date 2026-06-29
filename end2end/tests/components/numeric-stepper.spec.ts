import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("numeric-stepper primitive preview", () => {
  test("NS-01: default numeric stepper increments and decrements", async ({ page }) => {
    await openComponentPreview(page, "numeric-stepper");
    const wrapper = page.getByTestId("numeric-stepper-preview");
    const input = wrapper.locator("input[role=spinbutton]");
    await expect(input).toHaveValue("0");
    await wrapper.getByRole("button", { name: "Increment value" }).click();
    await expect(input).toHaveValue("1");
    await wrapper.getByRole("button", { name: "Decrement value" }).click();
    await expect(input).toHaveValue("0");
  });

  test("NS-02: bounded range clamps on increment", async ({ page }) => {
    await openComponentPreview(page, "numeric-stepper");
    const wrapper = page.getByTestId("numeric-stepper-bounded");
    const input = wrapper.locator("input[role=spinbutton]");
    const inc = wrapper.getByRole("button", { name: "Increment value" });
    for (let i = 0; i < 10; i += 1) {
      await inc.click();
    }
    await expect(input).toHaveValue("10");
  });

  test("NS-03: custom step increments by five", async ({ page }) => {
    await openComponentPreview(page, "numeric-stepper");
    const wrapper = page.getByTestId("numeric-stepper-step");
    const input = wrapper.locator("input[role=spinbutton]");
    await expect(input).toHaveValue("10");
    await wrapper.getByRole("button", { name: "Increment value" }).click();
    await expect(input).toHaveValue("15");
  });

  test("NS-04: disabled numeric stepper blocks interaction", async ({ page }) => {
    await openComponentPreview(page, "numeric-stepper");
    const wrapper = page.getByTestId("numeric-stepper-disabled");
    const input = wrapper.locator("input[role=spinbutton]");
    await expect(input).toBeDisabled();
    await expect(wrapper.getByRole("button", { name: "Increment value" })).toBeDisabled();
    await expect(wrapper.getByRole("button", { name: "Decrement value" })).toBeDisabled();
  });

  test("NS-05: small variant applies size class", async ({ page }) => {
    await openComponentPreview(page, "numeric-stepper");
    await expect(page.getByTestId("numeric-stepper-small").locator(".orbital-numeric-stepper--small")).toBeVisible();
  });
});
