import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("motion choreography stagger preview", () => {
  test("MC-01: add item increases visible tiles", async ({ page }) => {
    await openComponentPreview(page, "motion-choreography-stagger", "motion-choreography-stagger");
    const wrapper = page.getByTestId("motion-choreography-stagger").first();
    await expect(wrapper).toBeVisible({ timeout: 30_000 });
    const initial = await wrapper.locator("[data-testid^='motion-choreography-tile-']").count();
    expect(initial).toBeGreaterThanOrEqual(3);
    await wrapper.getByRole("button", { name: "Add item" }).click();
    await expect(wrapper.locator("[data-testid^='motion-choreography-tile-']")).toHaveCount(initial + 1);
  });

  test("MC-02: remove item decreases tile count", async ({ page }) => {
    await openComponentPreview(page, "motion-choreography-stagger", "motion-choreography-stagger");
    const wrapper = page.getByTestId("motion-choreography-stagger").first();
    const before = await wrapper.locator("[data-testid^='motion-choreography-tile-']").count();
    await wrapper.getByRole("button", { name: "Remove item" }).click();
    await expect(wrapper.locator("[data-testid^='motion-choreography-tile-']")).toHaveCount(before - 1);
  });

  test("MC-03: stagger control changes enter delay on second tile", async ({ page }) => {
    await openComponentPreview(page, "motion-choreography-stagger", "motion-choreography-stagger");
    const wrapper = page.getByTestId("motion-choreography-stagger").first();
    const secondTile = wrapper.locator("[data-testid^='motion-choreography-tile-']").nth(1);
    await expect(secondTile).toBeVisible();

    await wrapper.getByRole("button", { name: "Fast (50ms)" }).click();
    const fastDelay = await secondTile.evaluate((el) => {
      const style = getComputedStyle(el);
      return style.getPropertyValue("--orbital-motion-enter-delay").trim() || style.transitionDelay;
    });

    await wrapper.getByRole("button", { name: "Slow (300ms)" }).click();
    const slowDelay = await secondTile.evaluate((el) => {
      const style = getComputedStyle(el);
      return style.getPropertyValue("--orbital-motion-enter-delay").trim() || style.transitionDelay;
    });

    expect(fastDelay).not.toBe(slowDelay);
  });
});
