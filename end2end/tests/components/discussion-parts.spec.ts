import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion parts preview", () => {
  test("image file part renders preview", async ({ page }) => {
    await openComponentPreview(page, "discussion-parts");
    const preview = page.getByTestId("discussion-parts-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator('[data-file-kind="image"] img')).toBeVisible();
  });

  test("download file part renders link", async ({ page }) => {
    await openComponentPreview(page, "discussion-parts");
    const preview = page.getByTestId("discussion-parts-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator('[data-file-kind="download"] a')).toContainText("specification.pdf");
  });
});
