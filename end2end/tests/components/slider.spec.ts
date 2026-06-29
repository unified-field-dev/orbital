import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("slider primitive preview", () => {
  test("SL-01: default slider renders and has range role", async ({ page }) => {
    await openComponentPreview(page, "slider");
    await expect(page.getByTestId("slider-preview")).toBeVisible({ timeout: 30_000 });
    await expect(page.getByTestId("slider-preview").locator("input[type=range]")).toBeVisible();
  });

  test("SL-02: min/max constrain value", async ({ page }) => {
    await openComponentPreview(page, "slider");
    const input = page.getByTestId("slider-range").locator("input[type=range]");
    await input.evaluate((el) => {
      const range = el as HTMLInputElement;
      range.value = "5";
      range.dispatchEvent(new Event("input", { bubbles: true }));
    });
    await expect(input).toHaveValue("10");
    await input.evaluate((el) => {
      const range = el as HTMLInputElement;
      range.value = "35";
      range.dispatchEvent(new Event("input", { bubbles: true }));
    });
    await expect(input).toHaveValue("35");
  });

  test("SL-03: step slider snaps to increments", async ({ page }) => {
    await openComponentPreview(page, "slider");
    const input = page.getByTestId("slider-step").locator("input[type=range]");
    await input.evaluate((el) => {
      const range = el as HTMLInputElement;
      range.value = "37";
      range.dispatchEvent(new Event("input", { bubbles: true }));
    });
    await expect(input).toHaveValue("40");
  });

  test("SL-04: show_stops=false removes stop markers", async ({ page }) => {
    await openComponentPreview(page, "slider");
    const rail = page.getByTestId("slider-stops").locator(".orbital-slider__rail");
    const style = await rail.evaluate((el) => getComputedStyle(el).getPropertyValue("--orbital-slider--steps-percent"));
    expect(style.trim()).toBe("");
  });

  test("SL-05: vertical slider uses vertical class", async ({ page }) => {
    await openComponentPreview(page, "slider");
    await expect(page.getByTestId("slider-vertical").locator(".orbital-slider--vertical")).toBeVisible();
  });

  test("SL-06: slider labels render at key values", async ({ page }) => {
    await openComponentPreview(page, "slider");
    const labels = page.getByTestId("slider-labels").locator(".orbital-slider-label");
    await expect(labels).toHaveCount(3);
    await expect(page.getByTestId("slider-labels")).toContainText("0");
    await expect(page.getByTestId("slider-labels")).toContainText("50");
    await expect(page.getByTestId("slider-labels")).toContainText("100");
  });
});
