import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { previewUrl, waitForPreviewShell } from "../helpers";

test.describe("discussion preview", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "discussion");
    await expect(page.getByTestId("discussion-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows fixture reply rows", async ({ page }) => {
    await openComponentPreview(page, "discussion");
    await expect(page.locator("[data-reply-id='r-root']")).toBeVisible({ timeout: 30_000 });
    await expect(page.locator("[data-reply-id='r-sam']")).toBeVisible();
  });

  test("thread root has discussion data attribute", async ({ page }) => {
    await openComponentPreview(page, "discussion");
    await expect(page.locator("[data-orbital-discussion]").first()).toBeVisible({ timeout: 30_000 });
  });

  test("renders author names and OP label", async ({ page }) => {
    await openComponentPreview(page, "discussion");
    await expect(page.getByText("Alex Chen")).toBeVisible({ timeout: 30_000 });
    await expect(page.locator("[data-reply-label='OP']").first()).toBeVisible();
  });

  test("renders markdown formatting when enabled", async ({ page }) => {
    await openComponentPreview(page, "discussion");
    const rootCard = page.locator(
      "[data-reply-id='r-root'].orbital-discussion__reply-node-inner > .orbital-discussion__reply-card",
    );
    await expect(rootCard.locator(".orbital-discussion__markdown strong").first()).toBeVisible({
      timeout: 30_000,
    });
  });

  test("renders card surface classes", async ({ page }) => {
    await openComponentPreview(page, "discussion");
    await expect(page.locator("[data-reply-id='r-root'][data-reply-surface='op']")).toBeVisible({
      timeout: 30_000,
    });
    await expect(page.locator("[data-reply-id='r-sam'][data-reply-surface='neutral']")).toBeVisible();
    await expect(page.locator("[data-reply-id='r-jordan'][data-reply-surface='viewer']")).toBeVisible();
    await expect(
      page.locator("[data-reply-id='r-root'] .orbital-discussion__reply-card--op").first(),
    ).toBeVisible();
    await expect(
      page.locator("[data-reply-id='r-jordan'] .orbital-discussion__reply-card--viewer").first(),
    ).toBeVisible();
  });
});

test.describe("discussion replies preview", () => {
  test("rich thread preview renders chrome", async ({ page }) => {
    await page.goto(previewUrl("/discussion-replies"));
    await waitForPreviewShell(page);
    await expectPreviewVariants(page, ["discussion-thread-preview"]);
    const preview = page.getByTestId("discussion-thread-preview");

    await expect(preview.getByText("Alex Chen")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator("[data-reply-label='OP']").first()).toBeVisible();
    await expect(preview.locator("[data-author-role='agent']").first()).toBeVisible();
    await expect(preview.getByText("(edited)").first()).toBeVisible();
    await expect(preview.locator(".orbital-discussion__markdown strong").first()).toBeVisible();
    await expect(preview.getByTestId("discussion-hook-reply-count")).toHaveText("6");
    await expect(preview.locator(".orbital-discussion__reply-card--op").first()).toBeVisible();
    await expect(preview.locator(".orbital-discussion__reply-card--viewer").first()).toBeVisible();
  });
});
