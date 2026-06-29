import { test, expect } from "@playwright/test";
import { fillPreviewInput, selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("data-table-charts-integration preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "data-table-charts-integration");
    await expect(page.getByTestId("data-table-charts-integration-preview").first()).toBeVisible({ timeout: 30_000 });
  });

  test("dataset binding renders bar marks", async ({ page }) => {
    await openComponentPreview(page, "data-table-charts-integration");
    await expect(page.locator("[data-orbital-chart] svg .orb-bar-mark").first()).toBeVisible({ timeout: 30_000 });
  });

  test("filter changes update chart bar marks", async ({ page }) => {
    await openComponentPreview(page, "data-table-charts-integration");
    const preview = page.getByTestId("data-table-charts-integration-preview").first();
    const bars = preview.locator("[data-orbital-chart] svg .orb-bar-mark");
    await expect(bars.first()).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("charts-integration-bar-count")).toHaveText("4");

    await preview.getByTestId("data-table-filter-panel-trigger").click();
    const panel = page.getByTestId("data-table-filter-panel");
    await expect(panel).toBeVisible({ timeout: 10_000 });
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-field"), "score");
    await selectPreviewOption(panel.getByTestId("data-table-filter-rule-0-operator"), "greater_than");
    await fillPreviewInput(panel.getByTestId("data-table-filter-rule-0-value"), "40");
    await panel.getByRole("button", { name: "Apply" }).click();

    await expect(bars).toHaveCount(2);
    await expect(preview.getByTestId("charts-integration-bar-count")).toHaveText("2");
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "data-table-charts-integration");
    await expectPreviewVariants(page, ["data-table-charts-integration-preview"]);
  });
});
