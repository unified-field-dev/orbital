import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion integration preview", () => {
  test("renders integration preview", async ({ page }) => {
    await openComponentPreview(page, "discussion-integration");
    await expect(page.getByTestId("discussion-integration-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("discussion-hook-reply-count")).toBeVisible();
    await expect(page.getByTestId("discussion-composer-input")).toBeVisible();
  });

  test("submit via adapter mock adds reply to list", async ({ page }) => {
    await openComponentPreview(page, "discussion-integration");
    const preview = page.getByTestId("discussion-integration-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const initialCount = Number(await preview.getByTestId("discussion-hook-reply-count").innerText());

    const input = preview.locator('[data-testid="discussion-composer-input"] textarea');
    await input.fill("Integration adapter reply");
    await preview.locator('[data-testid="discussion-composer-send"] button').click();

    await expect(preview.locator(".orbital-discussion__reply-body").getByText("Integration adapter reply")).toBeVisible();
    await expect(preview.getByTestId("discussion-hook-reply-count")).toHaveText(String(initialCount + 1));
  });
});
