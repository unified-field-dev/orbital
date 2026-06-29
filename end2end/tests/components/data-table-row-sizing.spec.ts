import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table row sizing", () => {
  test("auto row height exceeds density min-height", async ({ page }) => {
    await openComponentPreview(page, "data-table-rows", "data-table-row-sizing-preview");
    const preview = page.getByTestId("data-table-row-sizing-preview");
    await preview.scrollIntoViewIfNeeded();

    const root = preview.locator(".orbital-data-table").first();
    await expect(root).toHaveClass(/orbital-data-table--auto-row-height/);

    const minHeight = await root.evaluate((el) => {
      const raw = getComputedStyle(el).getPropertyValue("--orbital-data-table-row-height").trim();
      return parseFloat(raw) || 40;
    });

    const row = preview.locator("tbody tr").first();
    const box = await row.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.height).toBeGreaterThan(minHeight + 4);
  });
});
