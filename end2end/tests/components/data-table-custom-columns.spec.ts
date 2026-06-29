import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table custom columns", () => {
  test("status column renders badge", async ({ page }) => {
    await openComponentPreview(page, "data-table-column-definition", "data-table-custom-columns-preview");
    const preview = page.getByTestId("data-table-custom-columns-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.locator(".orbital-badge").first()).toBeVisible();
    await expect(preview.getByText("Active")).toBeVisible();
  });
});
