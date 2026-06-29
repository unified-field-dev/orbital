import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("fixture-button macro preview", () => {
  test("manual fixture preview renders", async ({ page }) => {
    await openComponentPreview(page, "fixture-button");
    await expect(page.getByTestId("fixture-button")).toBeVisible();
  });
});
