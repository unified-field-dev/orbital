import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("card-preview slot preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "card-preview");
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "card-preview");
    await expectPreviewVariants(page, ["card-preview-slot-default", "card-preview-slot-in-card"]);
  });

  test("custom hero slot region", async ({ page }) => {
    await openComponentPreview(page, "card-preview");
    await expect(
      page.getByTestId("card-preview-slot-default").locator(".orbital-card-preview"),
    ).toBeVisible();
  });
});
