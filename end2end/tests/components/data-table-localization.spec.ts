import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table localization and RTL", () => {
  test("localized footer and RTL direction", async ({ page }) => {
    await openComponentPreview(page, "data-table-rendering", "data-table-localization-preview");
    const preview = page.getByTestId("data-table-localization-preview");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview.getByTestId("data-table-root")).toHaveAttribute("dir", "rtl");
    await expect(preview.getByTestId("data-table-footer")).toContainText("lignes");
  });
});
