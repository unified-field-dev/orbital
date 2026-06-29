import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timeline-navigation preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-navigation");
    await expect(page.getByTestId("scheduler-timeline-navigation-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-navigation");
    await expectPreviewVariants(page, ["scheduler-timeline-navigation-preview"]);
  });

  test("navigation changes visible date header", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-navigation");
    const wrapper = page.getByTestId("scheduler-timeline-navigation-preview");
    await expect(wrapper).toBeVisible();

    const title = wrapper.getByTestId("scheduler-timeline-header-title");
    const initial = await title.innerText();
    await wrapper.getByTestId("scheduler-timeline-nav-next").click();
    await expect(title).not.toHaveText(initial);
  });
});
