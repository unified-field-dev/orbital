import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectOverlayNonZeroSize } from "../lib/preview/overlays";
test.describe("info-label primitive preview", () => {
  test("IL-01: renders default preview", async ({ page }) => {
    await openComponentPreview(page, "info-label");
    await expect(page.getByTestId("info-label-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("IL-02..IL-05: variant examples are visible", async ({ page }) => {
    await openComponentPreview(page, "info-label");
    await expectPreviewVariants(page, [
      "info-label-required",
      "info-label-sizes",
      "info-label-weights",
      "info-label-disabled",
      "info-label-field",
      "info-label-custom-aria",
    ]);
  });

  test("IL-01: hovering info icon opens helper popover", async ({ page }) => {
    await openComponentPreview(page, "info-label");
    const trigger = page.getByTestId("info-label-preview").locator(".orbital-info-label__info-button");
    await trigger.hover();
    await expect(page.getByText("Shown once at creation")).toBeVisible();
    await expectOverlayNonZeroSize(page, ".orbital-popover-surface");
  });

  test("IL-07: custom info_aria_label sets accessible name on info button", async ({ page }) => {
    await openComponentPreview(page, "info-label");
    await page.getByTestId("info-label-custom-aria").scrollIntoViewIfNeeded();
    const trigger = page
      .getByTestId("info-label-custom-aria")
      .locator(".orbital-info-label__info-button");
    await expect(trigger).toHaveAccessibleName("Retention policy details");
  });

  test("IL-06: info label html_for matches input id", async ({ page }) => {
    await openComponentPreview(page, "info-label");
    await page.getByTestId("info-label-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("info-label-field").locator("label").getAttribute("for");
    const inputId = await page.getByTestId("info-label-field").locator("input").getAttribute("id");
    expect(labelFor).toEqual(inputId);
  });
});
