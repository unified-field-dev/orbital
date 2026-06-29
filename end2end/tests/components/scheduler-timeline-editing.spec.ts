import { test, expect } from "@playwright/test";
import { selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("scheduler-timeline-editing preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-editing");
    await expect(page.getByTestId("scheduler-timeline-editing-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-editing");
    await expectPreviewVariants(page, ["scheduler-timeline-editing-preview"]);
  });

  test("creates event via shared dialog", async ({ page }) => {
    await openComponentPreview(page, "scheduler-timeline-editing");
    const wrapper = page.getByTestId("scheduler-timeline-editing-preview").first();
    await expect(wrapper).toBeVisible();

    const openButton = wrapper.getByTestId("scheduler-event-dialog-open");
    await scrollIntoPreviewView(openButton);
    await openButton.click({ force: true });
    await expect(page.getByTestId("scheduler-event-dialog")).toBeVisible({ timeout: 10_000 });

    const title = "Timeline planning sync";
    await expect(page.getByTestId("scheduler-event-dialog-start")).toBeVisible();
    await page.getByTestId("scheduler-event-dialog-title").locator("input").fill(title);
    await selectPreviewOption(page.getByTestId("scheduler-event-dialog-resource"), "room-a");
    await page.getByTestId("scheduler-event-dialog-save").click();

    await expect(page.getByTestId("scheduler-event-dialog")).toBeHidden({ timeout: 10_000 });
    await scrollIntoPreviewView(wrapper.getByTestId("scheduler-timeline-scroll"));
    await expect(async () => {
      await expect(wrapper.getByText(title)).toBeVisible({ timeout: 1_000 });
    }).toPass({ timeout: 15_000 });
  });
});
