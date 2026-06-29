import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("demo-box preview", () => {
  test("renders default dashed-border demo box", async ({ page }) => {
    await openComponentPreview(page, "demo-box");
    await expect(page.getByTestId("demo-box-preview")).toBeVisible();
  });

  test("shows labeled variant", async ({ page }) => {
    await openComponentPreview(page, "demo-box");
    await expectPreviewVariants(page, ["demo-box-labeled"]);
    await expect(page.getByTestId("demo-box-labeled")).toContainText("Main content area");
  });
});
