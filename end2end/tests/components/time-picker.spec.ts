import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("time-picker primitive preview", () => {
  test("TP-01: default 12-hour format includes AM/PM controls", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    await page.getByTestId("TP-01").getByRole("button").first().click();
    const panel = page.locator(".orbital-time-picker-panel").last();
    await expect(panel.getByRole("button", { name: "AM" })).toBeVisible();
    await expect(panel.getByRole("button", { name: "PM" })).toBeVisible();
  });

  test("TP-02: 24-hour format omits AM/PM controls", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    await page.getByTestId("TP-02").getByRole("button").first().click();
    const panel = page.locator(".orbital-time-picker-panel").last();
    await expect(panel.getByRole("button", { name: "AM" })).toHaveCount(0);
    await expect(panel.getByRole("button", { name: "PM" })).toHaveCount(0);
    await expect(panel.locator(".orbital-time-picker-panel__column").first()).toBeVisible();
    await expect(panel.locator(".orbital-time-picker-panel__column").nth(1)).toBeVisible();
    await expect(panel.locator(".orbital-time-picker-panel__column").nth(2)).toBeVisible();
    const scroll = panel.locator(".orbital-time-picker-panel__scroll").first();
    const scrollbarWidth = await scroll.evaluate((el) => getComputedStyle(el).scrollbarWidth);
    expect(scrollbarWidth).toBe("thin");
  });

  test("TP-03: preselected value is rendered on trigger", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    await expect(page.getByTestId("TP-03").getByRole("button").first()).not.toHaveText("--:--:--");
  });

  test("TP-04: selecting values and OK writes hidden unix value", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    const wrapper = page.getByTestId("TP-04");
    await wrapper.getByRole("button").first().click();
    const panel = page.locator(".orbital-time-picker-panel").last();
    await panel.locator(".orbital-time-picker-panel__column").first().getByRole("button", { name: "09" }).click();
    await panel.locator(".orbital-time-picker-panel__column").nth(1).getByRole("button", { name: "30" }).click();
    await panel.locator(".orbital-time-picker-panel__column").nth(2).getByRole("button", { name: "15" }).click();
    await panel.getByRole("button", { name: "OK" }).click();
    await expect(wrapper.locator("input[type=hidden]")).not.toHaveValue("");
  });

  test("TP-05: disabled picker cannot be opened", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    await expect(page.getByTestId("TP-05").getByRole("button").first()).toBeDisabled();
  });

  test("TP-06: field label associates with trigger id", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    const wrapper = page.getByTestId("TP-06");
    const labelFor = await wrapper.locator("label").getAttribute("for");
    const triggerId = await wrapper.getByRole("button").first().getAttribute("id");
    expect(labelFor).toEqual(triggerId);
  });

  test("TP-07: plain value bind renders initial time", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    await expect(page.getByTestId("TP-07").getByRole("button").first()).not.toHaveText("--:--:--");
  });

  test("TP-08: empty bind shows placeholder", async ({ page }) => {
    await openComponentPreview(page, "time-picker");
    await expect(page.getByTestId("TP-08").getByRole("button").first()).toHaveText("--:--:--");
  });
});
