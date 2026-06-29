import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-timezone preview", () => {
  // Preview uses FixedOffset UTC-8 (not chrono Local — WASM uses host TZ, UTC on CI).
  test("switching timezone updates digital clock readout", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-timezone", "date-pickers-timezone-preview");
    const readout = page
      .getByTestId("date-pickers-timezone-clock")
      .locator(".orb-picker-digital-clock__readout");
    await expect(readout).toHaveText("7:30 AM");
    await page.getByTestId("date-pickers-timezone-utc").click();
    await expect(readout).toHaveText("3:30 PM");
    await page.getByTestId("date-pickers-timezone-local").click();
    await expect(readout).toHaveText("7:30 AM");
  });
});
