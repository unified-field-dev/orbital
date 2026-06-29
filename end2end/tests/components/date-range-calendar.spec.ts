import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-range-calendar preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-range-calendar");
    await expect(page.getByTestId("date-range-calendar-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-range-calendar");
    await expectPreviewVariants(page, ["date-range-calendar-preview"]);
  });

  test("completing a two-click range updates bound value", async ({ page }) => {
    await openComponentPreview(page, "date-range-calendar");
    const wrapper = page.getByTestId("date-range-calendar-preview");
    await expect(wrapper).toBeVisible();

    const value = page.getByTestId("date-range-calendar-preview-VALUE");
    await expect(value).toHaveText("none");

    const days = wrapper.locator(".orbital-calendar-item:not(.orbital-calendar-item--disabled)");
    await days.nth(10).click();
    await expect(wrapper.locator(".orbital-calendar-item--range-start")).toHaveCount(1);

    await days.nth(15).click();
    await expect(wrapper.locator(".orbital-calendar-item--in-range")).not.toHaveCount(0);
    await expect(wrapper.locator(".orbital-calendar-item--range-end")).toHaveCount(1);

    await expect(value).not.toHaveText("none");
    await expect(value).toHaveText(/^\d+,\d+$/);
  });
});
