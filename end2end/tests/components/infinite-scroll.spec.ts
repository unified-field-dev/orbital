import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("infinite-scroll preview", () => {
  test("IS-01: infinite scroll container renders", async ({ page }) => {
    await openComponentPreview(page, "infinite-scroll");
    const preview = page.getByTestId("infinite-scroll-preview");
    await expect(preview.getByTestId("orbital-infinite-scroll")).toBeVisible({ timeout: 30_000 });
  });
});
