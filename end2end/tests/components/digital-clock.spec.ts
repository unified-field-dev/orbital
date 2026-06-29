import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("digital-clock preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "digital-clock");
    await expect(page.getByTestId("digital-clock-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "digital-clock");
    await expectPreviewVariants(page, ["digital-clock-preview"]);
  });

  test("selecting a time slot updates bound value", async ({ page }) => {
    await openComponentPreview(page, "digital-clock");
    const wrapper = page.getByTestId("digital-clock-preview");
    await expect(wrapper).toBeVisible();

    const label = page.getByTestId("digital-clock-preview-LABEL");
    await expect(label).toHaveText("none");

    await wrapper.locator(".orbital-list__item", { hasText: "9:30 AM" }).click();

    await expect(label).toHaveText("09:30 AM");
    await expect(page.getByTestId("digital-clock-preview-VALUE")).not.toHaveText("none");
  });
});
