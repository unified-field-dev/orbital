import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion view modes", () => {
  test("flat mode shows all replies without tree nesting", async ({ page }) => {
    await openComponentPreview(page, "discussion-view-modes");
    const preview = page.getByTestId("discussion-view-modes-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await expect(preview.locator("[data-reply-id='fd-root']")).toBeVisible();
    await expect(preview.locator("[data-reply-id='fd-y1']")).toBeVisible();
    await expect(preview.locator("[data-reply-id='fd-t1']")).toBeVisible();
    await expect(preview.locator(".orbital-discussion__reply-node--connector")).toHaveCount(0);
  });

  test("date dividers appear in flat mode", async ({ page }) => {
    await openComponentPreview(page, "discussion-view-modes");
    const preview = page.getByTestId("discussion-view-modes-preview");
    await expect(preview.locator("[data-date-divider]").first()).toBeVisible({ timeout: 30_000 });
  });

  test("compact mode applies compact list class", async ({ page }) => {
    await openComponentPreview(page, "discussion-view-modes");
    const preview = page.getByTestId("discussion-view-modes-preview");
    const select = preview.getByTestId("discussion-view-mode-select").locator("select");
    await select.selectOption("compact");
    await expect(preview.locator(".orbital-discussion__reply-list--compact")).toBeVisible({
      timeout: 10_000,
    });
  });
});
