import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-playground preview", () => {
  test("knobs render and density select is present", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-playground", "date-pickers-playground-preview");
    const wrapper = page.getByTestId("date-pickers-playground-preview");
    await expect(wrapper.getByTestId("picker-preview-knobs")).toBeVisible();
    await expect(wrapper.getByTestId("date-pickers-playground-readout")).toContainText("none");
    await wrapper.locator('select[name="preview_density"]').selectOption("compact");
    await expect(wrapper.locator(".orb-picker-layout--density-compact").first()).toBeVisible();
  });
});
