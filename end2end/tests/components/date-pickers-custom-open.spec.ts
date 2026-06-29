import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-custom-open preview", () => {
  test("custom open button opens calendar panel", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-custom-open", "date-pickers-custom-open-preview");
    const wrapper = page.getByTestId("date-pickers-custom-open-preview");
    await wrapper.getByTestId("date-pickers-custom-open-button").click();
    await expect(page.locator(".orb-picker-range-calendar")).toBeVisible();
  });
});
