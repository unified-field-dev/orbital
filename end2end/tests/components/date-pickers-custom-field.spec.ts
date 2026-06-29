import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-custom-field preview", () => {
  test("custom summary field renders", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-custom-field", "date-pickers-custom-field-preview");
    const wrapper = page.getByTestId("date-pickers-custom-field-preview");
    await expect(wrapper.locator("input").first()).toHaveValue("Select a date range");
  });
});
