import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("compound-button preview", () => {
  test("CB-01 default with secondary line", async ({ page }) => {
    await openComponentPreview(page, "compound-button");
    await expect(page.getByTestId("compound-button-preview").locator("button")).toBeVisible();
    await expect(
      page.getByTestId("compound-button-preview").locator(".orbital-compound-button__secondary"),
    ).toContainText("Create a new workspace");
  });

  test("CB-02 icon after label", async ({ page }) => {
    await openComponentPreview(page, "compound-button");
    await expectPreviewVariants(page, ["compound-button-icon-after"]);
    await expect(
      page.getByTestId("compound-button-icon-after").locator(".orbital-button__icon"),
    ).toBeVisible();
  });
});
