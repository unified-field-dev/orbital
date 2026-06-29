import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar");
    await expect(page.getByTestId("scheduler-calendar-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar");
    await expectPreviewVariants(page, ["scheduler-calendar-preview"]);
  });

  test("navigation changes visible date header", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar");
    const wrapper = page.getByTestId("scheduler-calendar-preview");
    await expect(wrapper).toBeVisible();

    const title = wrapper.getByTestId("scheduler-calendar-header-title");
    const initial = await title.innerText();
    await wrapper.getByTestId("scheduler-calendar-nav-next").click();
    await expect(title).not.toHaveText(initial);
  });
});
