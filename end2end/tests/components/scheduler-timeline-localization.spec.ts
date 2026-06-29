import { test, expect } from "@playwright/test";
import { selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("scheduler-timeline-localization preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-localization");
    await expect(page.getByTestId("scheduler-timeline-localization-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-localization");
    await expectPreviewVariants(page, ["scheduler-timeline-localization-preview"]);
  });

  test("switches to French resource column header", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-localization");
    const preview = page.getByTestId("scheduler-timeline-localization-preview");

    await selectPreviewOption(preview.getByTestId("scheduler-locale-select"), "fr");
    await expect(async () => {
      await expect(preview.locator(".orb-scheduler-timeline__resource-header")).toHaveText("Ressources");
    }).toPass({ timeout: 15_000 });
    await expect(preview.getByTestId("scheduler-timeline-nav-today")).toHaveText("Aujourd'hui");
  });
});
