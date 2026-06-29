import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-quickstart placeholder preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-quickstart");
    await expect(page.getByTestId("scheduler-quickstart-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-quickstart");
    await expectPreviewVariants(page, ["scheduler-quickstart-preview"]);
  });
});
