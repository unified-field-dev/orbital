import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-custom-layout preview", () => {
  test("action bar clears calendar selection", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-custom-layout", "date-pickers-custom-layout-preview");
    const wrapper = page.getByTestId("date-pickers-custom-layout-preview");
    await wrapper.locator(".orbital-calendar-item").nth(10).click();
    await expect(wrapper.locator(".orbital-calendar-item--selected")).toBeVisible();
    await wrapper.getByTestId("date-pickers-custom-layout-clear").locator("button").click();
    await expect(wrapper.locator(".orbital-calendar-item--selected")).toHaveCount(0);
  });
});
