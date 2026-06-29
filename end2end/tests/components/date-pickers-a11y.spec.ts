import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-a11y preview", () => {
  test("calendar grid roles and segment spinbuttons", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-a11y", "date-pickers-a11y-preview");
    const wrapper = page.getByTestId("date-pickers-a11y-preview");
    await expect(wrapper.locator('[role="grid"]').first()).toBeVisible();
    const segment = wrapper.locator('[role="spinbutton"]').first();
    await expect(segment).toBeVisible();
    await segment.fill("06");
    await expect(segment).toHaveAttribute("aria-valuenow");
    await wrapper.locator('[role="gridcell"]').first().focus();
    await page.keyboard.press("ArrowRight");
    await expect(wrapper.locator('[role="gridcell"][tabindex="0"]')).toBeVisible();
  });
});
