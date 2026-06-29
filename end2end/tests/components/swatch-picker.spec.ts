import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("swatch-picker selection", () => {
  test("SP-01: default row has exactly one selected swatch", async ({ page }) => {
    await openComponentPreview(page, "swatch-picker");
    const preview = page.getByTestId("swatch-picker-preview");
    const radios = preview.getByRole("radio");
    await expect(radios).toHaveCount(4);
    await expect(preview.getByRole("radio", { checked: true })).toHaveCount(1);
    await expect(preview.getByRole("radio", { name: "Blue" })).toHaveAttribute("aria-checked", "true");
  });

  test("SP-02: clicking another swatch moves selection", async ({ page }) => {
    await openComponentPreview(page, "swatch-picker");
    await page.getByTestId("swatch-picker-controlled").scrollIntoViewIfNeeded();
    const preview = page.getByTestId("swatch-picker-controlled");
    await expect(preview.getByRole("radio", { checked: true })).toHaveCount(1);
    await expect(preview.getByRole("radio", { name: "Blue" })).toHaveAttribute("aria-checked", "true");
    await preview.getByRole("radio", { name: "Magenta" }).click();
    await expect(preview.getByRole("radio", { name: "Blue" })).toHaveAttribute("aria-checked", "false");
    await expect(preview.getByRole("radio", { name: "Magenta" })).toHaveAttribute("aria-checked", "true");
  });

  test("SP-03: grid variant renders documented examples", async ({ page }) => {
    await openComponentPreview(page, "swatch-picker");
    await expectPreviewVariants(page, ["swatch-picker-preview", "swatch-picker-grid", "swatch-picker-controlled"]);
    const grid = page.getByTestId("swatch-picker-grid");
    await expect(grid.getByRole("radiogroup")).toBeVisible();
    await expect(grid.getByRole("radio")).toHaveCount(6);
    await expect(grid.getByRole("radio", { name: "Purple" })).toHaveAttribute("aria-checked", "true");
  });
});
