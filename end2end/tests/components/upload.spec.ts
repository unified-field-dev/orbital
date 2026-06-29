import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("upload selection preview", () => {
  test("UP-01: basic upload renders trigger and file input", async ({ page }) => {
    await openComponentPreview(page, "upload");
    const preview = page.getByTestId("upload-preview");
    await expect(preview.getByRole("button", { name: "Choose file", exact: true })).toBeVisible();
    await expect(preview.locator('input[type="file"]')).toHaveCount(1);
  });

  test("UP-02: documented upload variants render", async ({ page }) => {
    await openComponentPreview(page, "upload");
    await expectPreviewVariants(page, [
      "upload-preview",
      "upload-multiple",
      "upload-accept",
      "upload-dragger",
      "upload-custom",
      "upload-callback",
    ]);
  });

  test("UP-03: multiple upload exposes multiple attribute", async ({ page }) => {
    await openComponentPreview(page, "upload");
    await page.getByTestId("upload-multiple").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("upload-multiple").locator('input[type="file"]')).toHaveAttribute(
      "multiple",
      "",
    );
  });
});
