import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("time-field preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "time-field");
    await expect(page.getByTestId("time-field-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "time-field");
    await expectPreviewVariants(page, ["time-field-preview"]);
  });

  test("typing segments updates bound value", async ({ page }) => {
    await openComponentPreview(page, "time-field");
    const wrapper = page.getByTestId("time-field-preview").first();
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("time-field-preview-VALUE");
    await expect(value).toHaveText("none");

    await wrapper.getByTestId("time-field-segment-hour").fill("02");
    await wrapper.getByTestId("time-field-segment-minute").fill("30");
    await wrapper.getByTestId("time-field-segment-meridiem").fill("PM");
    await wrapper.getByTestId("time-field-segment-meridiem").blur();

    await expect(value).not.toHaveText("none");
  });
});
