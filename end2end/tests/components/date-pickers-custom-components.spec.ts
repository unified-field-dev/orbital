import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-custom-components preview", () => {
  test("custom day cells render and select a date", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-custom-components", "date-pickers-custom-components-preview");
    const wrapper = page.getByTestId("date-pickers-custom-components-preview");
    await expect(wrapper.locator(".orb-picker-custom-day").first()).toBeVisible();
    await wrapper.locator(".orbital-calendar-item").first().click();
    await expect(wrapper.locator(".orbital-calendar-item--selected")).toBeVisible();
  });
});
