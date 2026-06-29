import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("motion reduced motion preview", () => {
  test("MR-01: prefers-reduced-motion uses instant transitions when respected", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    await openComponentPreview(page, "motion-reduced-motion", "motion-reduced-motion-preview");

    const respect = page.getByTestId("motion-reduced-respect");
    const ignore = page.getByTestId("motion-reduced-ignore");
    await expect(respect).toBeVisible({ timeout: 30_000 });
    await expect(ignore).toBeVisible();

    await page.getByRole("button", { name: "Toggle" }).click();
    await expect(respect).toBeHidden();
    await page.getByRole("button", { name: "Toggle" }).click();
    await expect(respect).toBeVisible();

    const respectDuration = await respect.evaluate((el) => getComputedStyle(el).transitionDuration);
    const ignoreDuration = await ignore.evaluate((el) => getComputedStyle(el).transitionDuration);

    expect(respectDuration).toMatch(/0\.001s|0s|1ms/);
    expect(ignoreDuration).toMatch(/0\.2s|200ms/);
  });
});
