import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("scheduler-timezone preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timezone");
    await expect(page.getByTestId("scheduler-timezone-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timezone");
    await expectPreviewVariants(page, ["scheduler-timezone-preview"]);
  });

  test("switching display timezone shifts event label", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timezone");
    const wrapper = page.getByTestId("scheduler-timezone-preview").first();
    await expect(wrapper).toBeVisible();

    const label = wrapper.getByTestId("scheduler-timezone-event-label");
    await expect(label).toBeVisible();
    await expect(label).toContainText("14:");

    const select = wrapper.getByTestId("scheduler-timezone-select").locator("select");
    await select.selectOption("eastern");
    await expect(label).toContainText("09:");
  });
});
