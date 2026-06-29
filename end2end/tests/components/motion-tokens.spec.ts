import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("motion-tokens preview", () => {
  test("MT-01: duration and curve tables render", async ({ page }) => {
    await openComponentPreview(page, "motion-tokens");
    await expect(page.getByTestId("motion-tokens-preview").first()).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("motion-tokens-durations").first()).toBeVisible();
    await expect(page.getByTestId("motion-tokens-curves").first()).toBeVisible();
  });
});
