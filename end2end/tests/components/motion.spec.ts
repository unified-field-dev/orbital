import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("motion overview preview", () => {
  test("MO-01: motion overview renders stack and try panel", async ({ page }) => {
    await openComponentPreview(page, "motion");
    await expect(page.getByTestId("motion-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("motion-overview-stack")).toBeVisible();
    await expect(page.getByTestId("motion-overview-try")).toBeVisible();
  });
});
