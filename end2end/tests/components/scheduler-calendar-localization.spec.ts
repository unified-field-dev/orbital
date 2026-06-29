import { test, expect } from "@playwright/test";
import { selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-localization preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-localization");
    await expect(page.getByTestId("scheduler-calendar-localization-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-localization");
    await expectPreviewVariants(page, ["scheduler-calendar-localization-preview"]);
  });

  test("switches toolbar labels to French", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-localization");
    const preview = page.getByTestId("scheduler-calendar-localization-preview");

    await selectPreviewOption(preview.getByTestId("scheduler-locale-select"), "fr");
    await expect(async () => {
      await expect(preview.getByTestId("scheduler-calendar-nav-today")).toHaveText("Aujourd'hui");
    }).toPass({ timeout: 15_000 });
    await expect(preview.getByTestId("scheduler-calendar-nav-previous")).toHaveText("Précédent");
    await expect(preview.getByTestId("scheduler-calendar-nav-next")).toHaveText("Suivant");
  });
});
