import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("time-clock preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "time-clock");
    await expect(page.getByTestId("time-clock-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "time-clock");
    await expectPreviewVariants(page, ["time-clock-preview"]);
  });

  test("selecting hour and minute updates bound value", async ({ page }) => {
    await openComponentPreview(page, "time-clock", "TC-02");
    const wrapper = page.getByTestId("TC-02");
    await expect(wrapper).toBeVisible();

    const value = wrapper.getByTestId("TC-02-VALUE");
    await expect(value).toHaveText("none");

    await wrapper.getByRole("button", { name: "3 hours", exact: true }).click();
    await wrapper.getByRole("button", { name: "30 minutes", exact: true }).click();

    await expect(value).not.toHaveText("none");
  });

  test("minute view shows five-minute labels only", async ({ page }) => {
    await openComponentPreview(page, "time-clock", "TC-02");
    const wrapper = page.getByTestId("TC-02");
    await wrapper.getByRole("button", { name: "3 hours", exact: true }).click();

    await expect(wrapper.getByRole("button", { name: "00 minutes", exact: true })).toBeVisible();
    await expect(wrapper.getByRole("button", { name: "55 minutes", exact: true })).toBeVisible();
    await expect(wrapper.getByRole("button", { name: "01 minutes", exact: true })).toHaveCount(0);
    await expect(wrapper.getByRole("button", { name: "07 minutes", exact: true })).toHaveCount(0);
  });

  test("dragging on clock face selects hour and minute", async ({ page }) => {
    await openComponentPreview(page, "time-clock", "TC-02");
    const wrapper = page.getByTestId("TC-02");
    const value = wrapper.getByTestId("TC-02-VALUE");
    await expect(value).toHaveText("none");

    const face = wrapper.locator(".orb-picker-time-clock__face");
    const box = await face.boundingBox();
    if (!box) {
      throw new Error("time clock face not found");
    }

    const cx = box.x + box.width / 2;
    const cy = box.y + box.height / 2;
    const radius = box.width * 0.35;

    // Drag toward 3 o'clock to select hour 3, then toward 6 o'clock for minute 30.
    await page.mouse.move(cx + radius, cy);
    await page.mouse.down();
    await page.mouse.up();

    await page.mouse.move(cx, cy + radius);
    await page.mouse.down();
    await page.mouse.up();

    await expect(value).not.toHaveText("none");
  });
});
