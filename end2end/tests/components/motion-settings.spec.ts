import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("motion settings preview", () => {
  test("MS-01: changing duration updates motion timing tokens on demo shape", async ({ page }) => {
    await openComponentPreview(page, "motion-settings", "motion-settings-demo");
    const shape = page.getByTestId("motion-settings-shape");
    await expect(shape).toBeVisible({ timeout: 30_000 });

    const enterDuration = () =>
      shape.evaluate((el) =>
        getComputedStyle(el).getPropertyValue("--orbital-motion-enter-duration").trim(),
      );

    await page.getByTestId("motion-settings-duration-normal").getByRole("button", { name: "Normal" }).click();
    await expect.poll(enterDuration).toBe("200ms");

    await page.getByTestId("motion-settings-duration-fast").getByRole("button", { name: "UltraFast" }).click();
    await expect.poll(enterDuration).toBe("50ms");

    await page.getByTestId("motion-settings-duration-slow").getByRole("button", { name: "Slow" }).click();
    await expect.poll(enterDuration).toBe("300ms");
  });
});
