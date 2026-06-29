import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-lazy-loading preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-lazy-loading");
    await expect(page.getByTestId("scheduler-calendar-lazy-loading-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-lazy-loading");
    await expectPreviewVariants(page, ["scheduler-calendar-lazy-loading-preview"]);
  });

  test("shows loading then events", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-lazy-loading");
    const preview = page.getByTestId("scheduler-calendar-lazy-loading-preview");

    await expect(preview.getByTestId("scheduler-loading-overlay")).toBeVisible({ timeout: 10_000 });
    await expect(preview.getByTestId("scheduler-loading-overlay")).toBeHidden({ timeout: 15_000 });
    await expect(preview.getByTestId("scheduler-event-evt-1")).toBeVisible({ timeout: 10_000 });
  });

  test("shows error overlay when simulate error is enabled", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-lazy-loading");
    const preview = page.getByTestId("scheduler-calendar-lazy-loading-preview");

    await preview.getByRole("switch", { name: "Simulate error" }).click();
    await expect(preview.getByTestId("scheduler-error-overlay")).toBeVisible({ timeout: 15_000 });
    await expect(preview.getByTestId("scheduler-event-evt-1")).toHaveCount(0);
  });
});
