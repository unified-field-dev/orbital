import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("date-time-picker preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-time-picker");
    await expect(page.getByTestId("date-time-picker-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-time-picker");
    await expectPreviewVariants(page, ["date-time-picker-preview"]);
  });

  test("selecting date and time updates bound value", async ({ page }) => {
    await openComponentPreview(page, "date-time-picker", "DTP-02");
    const wrapper = page.getByTestId("DTP-02");
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("DTP-02-VALUE");
    await expect(value).toHaveText("none");

    await wrapper.getByRole("button", { name: "Open calendar" }).click();
    await page.locator(".orbital-date-picker__day-button").nth(10).click();

    await wrapper.getByRole("button").nth(1).click();
    const panel = page.locator(".orbital-time-picker-panel").last();
    await expect(panel).toBeVisible();
    await panel.locator(".orbital-time-picker-panel__column").first().getByRole("button", { name: "09" }).click();
    await panel.locator(".orbital-time-picker-panel__column").nth(1).getByRole("button", { name: "30" }).click();
    await panel.locator(".orbital-time-picker-panel__column").nth(2).getByRole("button", { name: "00" }).click();
    await panel.getByRole("button", { name: "OK" }).click();

    await expect(value).not.toHaveText("none");
  });
});
