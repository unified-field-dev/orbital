import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("stepper preview", () => {
  test("ST-01: default vertical stepper shows pipeline steps", async ({ page }) => {
    await openComponentPreview(page, "stepper");
    const preview = page.getByTestId("stepper-preview");
    await expect(preview.getByText("Build Docker image")).toBeVisible();
    await expect(preview.getByText("Compressing layers...")).toBeVisible();
  });

  test("ST-02: failure variant shows error message", async ({ page }) => {
    await openComponentPreview(page, "stepper");
    await expect(page.getByText("Connection refused")).toBeVisible();
  });
});
