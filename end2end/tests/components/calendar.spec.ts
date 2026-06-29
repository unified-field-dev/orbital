import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectHorizontallyBetween } from "../lib/assertions/layout";
import { expectDistinctStyle } from "../lib/assertions/style";
test.describe("calendar preview behaviors", () => {
  test("CA-01 renders default calendar and navigation order", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    const wrapper = page.getByTestId("calendar-preview");
    await expect(wrapper).toBeVisible({ timeout: 30_000 });

    const previous = wrapper.getByRole("button", { name: "Previous" });
    const today = wrapper.getByRole("button", { name: "Today" });
    const next = wrapper.getByRole("button", { name: "Next" });
    await expectHorizontallyBetween(previous, today, next);
  });

  test("CA-02 selected day has distinct selected bar style", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    const wrapper = page.getByTestId("CA-02");
    await expect(wrapper).toBeVisible();

    const selectedBar = wrapper.locator(".orbital-calendar-item--selected .orbital-calendar-item__bar");
    const firstOtherBar = wrapper
      .locator(".orbital-calendar-item:not(.orbital-calendar-item--selected) .orbital-calendar-item__bar")
      .first();

    await expectDistinctStyle(selectedBar, firstOtherBar, "background-color");
  });

  test("CA-03 clicking a day updates bound value text", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    const wrapper = page.getByTestId("CA-03");
    await expect(wrapper).toBeVisible();

    const value = page.getByTestId("CA-03-VALUE");
    await expect(value).toHaveText("none");

    await wrapper.locator(".orbital-calendar-item").nth(10).click();
    await expect(value).not.toHaveText("none");
  });

  test("CA-04 UTC example renders and allows selection", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    const wrapper = page.getByTestId("CA-04");
    await expect(wrapper).toBeVisible();

    const before = await wrapper.locator(".orbital-calendar-item--selected").count();
    await wrapper.locator(".orbital-calendar-item").nth(12).click();
    const after = await wrapper.locator(".orbital-calendar-item--selected").count();
    expect(before).toBeGreaterThan(0);
    expect(after).toBeGreaterThan(0);
  });

  test("CA-05 fixed offset example renders and keeps grid", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    const wrapper = page.getByTestId("CA-05");
    await expect(wrapper).toBeVisible();
    const cells = await wrapper.locator(".orbital-calendar-item").count();
    expect(cells).toBeGreaterThanOrEqual(28);
    expect(cells % 7).toBe(0);
  });

  test("CA-06 navigation changes visible month", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    const wrapper = page.getByTestId("CA-06");
    await expect(wrapper).toBeVisible();

    const title = wrapper.getByTestId("calendar-header-title");
    const initial = await title.innerText();
    await wrapper.getByRole("button", { name: "Next" }).click();
    await expect(title).not.toHaveText(initial);
  });

  test("CA-07 today action updates month and selection", async ({ page }) => {
    await openComponentPreview(page, "calendar");
    await expectPreviewVariants(page, ["CA-07"]);
    const wrapper = page.getByTestId("CA-07");

    await wrapper.getByRole("button", { name: "Previous" }).click();
    await wrapper.getByRole("button", { name: "Today" }).click();
    await expect(wrapper.locator(".orbital-calendar-item--selected")).toHaveCount(1);
    await expect(wrapper.locator(".orbital-calendar-item--today")).toHaveCount(1);
  });
});
