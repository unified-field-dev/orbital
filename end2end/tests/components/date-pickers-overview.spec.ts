import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-overview preview", () => {
  test("overview preview renders picker family", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-overview", "date-pickers-overview-preview");
    const wrapper = page.getByTestId("date-pickers-overview-preview");
    await expect(wrapper.locator(".orb-date-field, .orb-time-field").first()).toBeVisible();
  });

  test("overview includes getting started DatePicker example", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-overview", "date-pickers-overview-getting-started-preview");
    const wrapper = page.getByTestId("date-pickers-overview-getting-started-preview");
    await wrapper.locator('button[aria-label="Open calendar"]').click();
    await expect(page.locator('[data-testid="date-picker-panel"]')).toBeVisible();
  });

  test("overview documents common FAQ answers", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-overview");
    await page.getByRole("tab", { name: "Best Practices" }).click();
    await expect(page.getByTestId("preview-doc-content")).toContainText("OrbitalDateTime");
    await expect(page.getByTestId("preview-doc-content")).toContainText("LocalizationProvider");
  });
});
