import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("button-group primitive behavior", () => {
  test("BG-01: default group renders two adjacent buttons", async ({ page }) => {
    await openComponentPreview(page, "button-group");
    const buttons = page.getByTestId("button-group-preview").getByRole("button");
    await expect(buttons).toHaveCount(2);
    await expect(buttons.nth(0)).toHaveText("Save");
    await expect(buttons.nth(1)).toHaveText("Cancel");
  });

  test("BG-02: vertical variant applies vertical class", async ({ page }) => {
    await openComponentPreview(page, "button-group");
    const wrapper = page.getByTestId("button-group-vertical").locator(".orbital-button-group");
    await expect(wrapper).toHaveClass(/orbital-button-group--vertical/);
  });

  test("BG-03..BG-05: example variants are visible", async ({ page }) => {
    await openComponentPreview(page, "button-group");
    await expectPreviewVariants(page, [
      "button-group-toolbar",
      "button-group-icons",
      "button-group-disabled",
    ]);
  });
});
