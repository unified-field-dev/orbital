import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("orbital-presence preview", () => {
  test("OP-01: presence toggle demo renders", async ({ page }) => {
    await openComponentPreview(page, "orbital-presence");
    await expect(page.getByTestId("orbital-presence-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("orbital-presence-shape")).toBeVisible();
  });

  test("OP-02: appear-on-mount demo renders", async ({ page }) => {
    await openComponentPreview(page, "orbital-presence");
    await expect(page.getByTestId("orbital-presence-appear")).toBeVisible();
  });
});
