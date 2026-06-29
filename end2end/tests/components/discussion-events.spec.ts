import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion events preview", () => {
  test("renders event log panel", async ({ page }) => {
    await openComponentPreview(page, "discussion-events");
    await expect(page.getByTestId("discussion-events-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("discussion-events-log")).toBeVisible();
  });
});
