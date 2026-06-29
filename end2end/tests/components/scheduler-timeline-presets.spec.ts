import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-presets preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    await expect(page.getByTestId("scheduler-timeline-presets-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    await expectPreviewVariants(page, ["scheduler-timeline-presets-preview"]);
  });

  test("switching preset changes header title", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-presets");
    const wrapper = page.getByTestId("scheduler-timeline-presets-preview");
    await expect(wrapper).toBeVisible();

    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    const initial = await title.innerText();

    const presetSelect = wrapper.getByTestId("scheduler-timeline-preset-select").locator("select");
    await presetSelect.selectOption("business_day");
    await expect(title).not.toHaveText(initial);

    const businessDayTitle = await title.innerText();
    await presetSelect.selectOption("week");
    await expect(title).not.toHaveText(businessDayTitle);
  });
});
