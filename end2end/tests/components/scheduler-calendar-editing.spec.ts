import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-calendar-editing preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-editing");
    await expect(page.getByTestId("scheduler-calendar-editing-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-editing");
    await expectPreviewVariants(page, ["scheduler-calendar-editing-preview"]);
  });

  test("creates event via dialog and shows chip in grid", async ({ page }) => {
    await openComponentPreview(page, "scheduler-calendar-editing");
    const wrapper = page.getByTestId("scheduler-calendar-editing-preview").first();
    await expect(wrapper).toBeVisible();

    await wrapper.getByTestId("scheduler-event-dialog-open").click();
    await expect(page.getByTestId("scheduler-event-dialog")).toBeVisible();

    const title = "Orbital standup";
    await page.getByTestId("scheduler-event-dialog-title").locator("input").fill(title);
    await page.getByTestId("scheduler-event-dialog-save").click();

    await expect(page.getByTestId("scheduler-event-dialog")).toBeHidden({ timeout: 10_000 });
    await expect(wrapper.getByText(title)).toBeVisible({ timeout: 10_000 });
  });
});
