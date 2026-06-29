import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-views preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-views");
    await expect(page.getByTestId("scheduler-calendar-views-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-views");
    await expectPreviewVariants(page, ["scheduler-calendar-views-preview"]);
  });

  test("view switcher shows month layout", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-views");
    const wrapper = page.getByTestId("scheduler-calendar-views-preview");
    await expect(wrapper).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-calendar-view-week")).toBeVisible();

    const select = wrapper.getByTestId("scheduler-calendar-view-select").locator("select");
    await select.selectOption("month");
    await expect(wrapper.getByTestId("scheduler-calendar-view-month")).toBeVisible();
    await expect(wrapper.getByTestId("scheduler-calendar-view-week")).toHaveCount(0);
  });
});
