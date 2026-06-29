import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
const ATOM_TEST_IDS = [
  "motion-atom-fade",
  "motion-atom-fade-scale",
  "motion-atom-scale",
  "motion-atom-slide-bottom",
  "motion-atom-slide-top",
  "motion-atom-slide-left",
  "motion-atom-slide-right",
  "motion-atom-collapse",
  "motion-atom-blur",
  "motion-atom-rotate",
] as const;

test.describe("motion atoms gallery", () => {
  test("MA-01: gallery renders all atom cells", async ({ page }) => {
    await openComponentPreview(page, "motion-atoms");
    const gallery = page.getByTestId("motion-atoms-gallery");
    await expect(gallery).toBeVisible({ timeout: 30_000 });
    for (const testId of ATOM_TEST_IDS) {
      await expect(gallery.getByTestId(testId)).toBeVisible();
    }
  });

  for (const testId of ATOM_TEST_IDS) {
    test(`MA-${ATOM_TEST_IDS.indexOf(testId) + 2}: toggle ${testId}`, async ({ page }) => {
      await openComponentPreview(page, "motion-atoms");
      const gallery = page.getByTestId("motion-atoms-gallery");
      const shape = gallery.getByTestId(testId);
      await expect(shape).toBeVisible();
      const cell = shape.locator(
        "xpath=ancestor::div[contains(@class,'orbital-motion-demo-cell')][1]",
      );
      const toggle = cell.getByRole("button");
      await toggle.click();
      await expect(shape).toBeHidden();
      await toggle.click();
      await expect(shape).toBeVisible();
    });
  }
});
