import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table custom slots", () => {
  test("custom toolbar footer and empty views render on root preview", async ({ page }) => {
    await openComponentPreview(page, "data-table", "data-table-slots-preview");
    const preview = page.getByTestId("data-table-slots-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("custom-toolbar")).toBeVisible();
    await expect(preview.getByTestId("custom-footer")).toBeVisible();
    await expect(preview.getByTestId("custom-empty")).toBeVisible();
    await expect(preview.getByTestId("data-table-toolbar")).toHaveCount(0);
  });

  test("slots doc hub preview renders custom chrome", async ({ page }) => {
    await openComponentPreview(page, "data-table-slots", "data-table-slots-preview");
    const preview = page.getByTestId("data-table-slots-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("custom-toolbar")).toBeVisible();
    await expect(preview.getByTestId("custom-footer")).toBeVisible();
    await expect(preview.getByTestId("custom-empty")).toBeVisible();
  });
});
