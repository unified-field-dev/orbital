import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion custom areas", () => {
  test("meta_view renders custom content", async ({ page }) => {
    await openComponentPreview(page, "discussion-custom-areas");
    const preview = page.getByTestId("discussion-custom-areas-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("discussion-custom-meta").first()).toHaveText("12 likes");
  });

  test("reply overflow menu fires on_reply_action", async ({ page }) => {
    await openComponentPreview(page, "discussion-custom-areas");
    const preview = page.getByTestId("discussion-custom-areas-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const menu = preview.locator("[data-testid^='discussion-reply-menu-']").first();
    await expect(menu).toBeVisible();
    await menu.getByRole("button", { name: "Reply actions" }).click();
    await page.getByTestId("discussion-reply-action-report_tos").click();

    await expect(page.getByTestId("discussion-custom-action-log")).toContainText("report_tos");
  });
});
