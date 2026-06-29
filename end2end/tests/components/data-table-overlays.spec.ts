import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table overlays", () => {
  test("empty dataset shows empty overlay", async ({ page }) => {
    await openComponentPreview(page, "data-table-rendering", "data-table-overlays-preview");
    const preview = page.getByTestId("data-table-overlays-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByTestId("data-table-empty")).toBeVisible({ timeout: 10_000 });
  });
});
