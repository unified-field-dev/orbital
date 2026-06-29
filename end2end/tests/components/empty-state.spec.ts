import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("empty-state preview", () => {
  test("ES-01: illustrated empty state shows message", async ({ page }) => {
    await openComponentPreview(page, "empty-state");
    await expect(page.getByText("No items found")).toBeVisible();
  });

  test("ES-02: search empty state shows guidance", async ({ page }) => {
    await openComponentPreview(page, "empty-state");
    await expect(page.getByText("Try adjusting your search or filters.")).toBeVisible();
  });
});
