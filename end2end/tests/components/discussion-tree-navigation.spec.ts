import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion tree navigation", () => {
  test("show-more drill-in and go back", async ({ page }) => {
    await openComponentPreview(page, "discussion-tree-navigation");
    const preview = page.getByTestId("discussion-tree-navigation-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const showMore = preview.getByText(/Show \d+ more replies/);
    await expect(showMore).toBeVisible();
    await showMore.click();

    await expect(preview.locator("[data-reply-id='d-l4']")).toBeVisible();
    await expect(preview.locator("[data-reply-id='d-root']")).not.toBeVisible();

    await preview.getByText("Go back to parent thread").click();
    await expect(preview.locator("[data-reply-id='d-root']")).toBeVisible({ timeout: 10_000 });
  });

  test("collapse hides reply body", async ({ page }) => {
    await openComponentPreview(page, "discussion-tree-navigation");
    const preview = page.getByTestId("discussion-tree-navigation-preview");
    const samCard = preview.locator("[data-reply-id='d-l1']").first();
    await expect(samCard).toBeVisible({ timeout: 30_000 });

    await samCard.locator(".orbital-discussion__collapse-toggle").first().click();
    await expect(samCard).toHaveAttribute("data-collapsed", "true");
    await expect(samCard.locator(".orbital-discussion__reply-body")).not.toBeVisible();
  });
});
