import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("tab-list primitive preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "tab-list");
    await expect(page.getByTestId("tab-list-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows variant examples", async ({ page }) => {
    await openComponentPreview(page, "tab-list");
    await expectPreviewVariants(page, ["tab-list-three", "tab-list-controlled"]);
  });
});
