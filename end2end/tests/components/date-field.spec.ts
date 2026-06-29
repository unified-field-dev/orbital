import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-field preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-field");
    await expect(page.getByTestId("date-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-field");
    await expectPreviewVariants(page, ["date-field-preview"]);
  });

  test("typing segments updates bound value", async ({ page }) => {
    await openComponentPreview(page, "date-field");
    const wrapper = page.getByTestId("date-field-preview").first();
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("date-field-preview-VALUE");
    await expect(value).toHaveText("none");

    const month = wrapper.getByTestId("date-field-segment-month");
    const day = wrapper.getByTestId("date-field-segment-day");
    const year = wrapper.getByTestId("date-field-segment-year");

    await month.click();
    await month.fill("06");
    await month.press("Tab");
    await day.fill("14");
    await day.press("Tab");
    await year.fill("2025");
    await year.press("Tab");

    await expect(value).not.toHaveText("none");
  });
});
