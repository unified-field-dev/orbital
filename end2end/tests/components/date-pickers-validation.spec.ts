import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-validation preview", () => {
  test("out-of-range typed date shows Field error on blur", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-validation", "date-pickers-validation-preview");
    const wrapper = page.getByTestId("date-pickers-validation-preview");
    const input = wrapper.locator("input");
    await input.fill("01/01/2020");
    await input.blur();
    await expect(wrapper.locator(".orbital-field__validation-message")).toBeVisible();
  });
});
