import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("code primitive preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "code");
    await expect(page.getByTestId("code-preview")).toBeVisible({ timeout: 30_000 });
  });
});
