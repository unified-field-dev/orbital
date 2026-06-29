import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-range-picker preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-range-picker");
    await expect(page.getByTestId("date-range-picker-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-range-picker");
    await expectPreviewVariants(page, ["date-range-picker-preview"]);
  });

  test("selecting start and end days in calendar updates bound value", async ({ page }) => {
    await openComponentPreview(page, "date-range-picker");
    const wrapper = page.getByTestId("date-range-picker-preview");
    await expect(wrapper).toBeVisible();

    const value = page.getByTestId("date-range-picker-preview-VALUE");
    await expect(value).toHaveText("none");

    await wrapper.getByRole("button", { name: "Open calendar" }).click();

    const days = page.locator(".orbital-calendar-item:not(.orbital-calendar-item--disabled)");
    await days.nth(10).click();
    await expect(page.locator(".orbital-calendar-item--range-start")).toHaveCount(1);

    await days.nth(15).click();
    await expect(page.locator(".orbital-calendar-item--in-range")).not.toHaveCount(0);
    await expect(page.locator(".orbital-calendar-item--range-end")).toHaveCount(1);

    await expect(value).not.toHaveText("none");
    await expect(value).toHaveText(/^\d+,\d+$/);
  });
});
