import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion localization", () => {
  test("locale switch changes composer placeholder", async ({ page }) => {
    await openComponentPreview(page, "discussion-localization");
    const preview = page.getByTestId("discussion-localization-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const textarea = preview.locator(".orbital-discussion__composer-textarea textarea");
    await expect(textarea).toHaveAttribute("placeholder", "Write a reply…");

    const localeSelect = preview.getByTestId("discussion-locale-select").locator("select");
    await localeSelect.selectOption("fr");

    await expect(textarea).toHaveAttribute("placeholder", "Écrire une réponse…", {
      timeout: 10_000,
    });
  });

  test("locale switch changes view mode toolbar label", async ({ page }) => {
    await openComponentPreview(page, "discussion-localization");
    const preview = page.getByTestId("discussion-localization-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await expect(preview.getByText("View", { exact: true })).toBeVisible();

    const localeSelect = preview.getByTestId("discussion-locale-select").locator("select");
    await localeSelect.selectOption("fr");

    await expect(preview.getByText("Affichage", { exact: true })).toBeVisible({
      timeout: 10_000,
    });
  });
});
