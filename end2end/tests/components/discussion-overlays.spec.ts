import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion overlays preview", () => {
  test("custom empty state renders", async ({ page }) => {
    await openComponentPreview(page, "discussion-overlays");
    const preview = page.getByTestId("discussion-overlays-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("discussion-custom-empty")).toBeVisible();
    await expect(preview.getByTestId("discussion-custom-empty")).toContainText("No replies yet");
  });

  test("loading overlay renders when toggled", async ({ page }) => {
    await openComponentPreview(page, "discussion-overlays");
    const preview = page.getByTestId("discussion-overlays-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await preview.getByRole("button", { name: "Loading" }).click();
    await expect(preview.getByTestId("discussion-thread-loading")).toBeVisible();
  });
});
