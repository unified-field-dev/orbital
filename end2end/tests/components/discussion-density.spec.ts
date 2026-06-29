import { test, expect } from "@playwright/test";
import { expectPreviewVariants } from "../lib/preview/navigation";
import { previewUrl, waitForPreviewShell } from "../helpers";

test.describe("discussion density", () => {
  test("density preview renders", async ({ page }) => {
    await page.goto(previewUrl("/discussion-replies"));
    await waitForPreviewShell(page);
    await expectPreviewVariants(page, ["discussion-density-preview"]);
    await expect(page.getByTestId("discussion-density-preview")).toBeVisible();
  });

  test("thread root exposes density CSS variables", async ({ page }) => {
    await page.goto(previewUrl("/discussion-replies"));
    await waitForPreviewShell(page);
    const preview = page.getByTestId("discussion-density-preview");
    const root = preview.locator("[data-orbital-discussion]").first();

    await expect(preview.getByTestId("theme-density-increase")).toBeVisible({ timeout: 30_000 });

    const readAvatarSize = () =>
      root.evaluate((el) =>
        getComputedStyle(el).getPropertyValue("--orbital-discussion-avatar-size").trim(),
      );

    const compactSize = await readAvatarSize();
    expect(["24px", "32px", "40px"]).toContain(compactSize);

    await preview.getByTestId("theme-density-increase").click();
    await expect(preview.getByTestId("theme-density-value")).not.toContainText("Compact");
    const tallerSize = await readAvatarSize();
    expect(tallerSize).not.toBe(compactSize);
  });
});
