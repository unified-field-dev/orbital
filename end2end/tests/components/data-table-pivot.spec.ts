import { test, expect } from "@playwright/test";
import { selectPreviewOption } from "../lib/preview/forms";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table pivot", () => {
  test("pivot panel applies dynamic columns", async ({ page }) => {
    await openComponentPreview(page, "data-table-advanced", "data-table-pivot-preview");
    const preview = page.getByTestId("data-table-pivot-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-pivot-trigger").click();
    const panel = page.getByTestId("data-table-pivot-panel");
    await expect(panel).toBeVisible();

    await selectPreviewOption(panel.getByTestId("data-table-pivot-row-field"), "region");
    await selectPreviewOption(panel.getByTestId("data-table-pivot-value-field"), "amount");
    await panel.getByTestId("data-table-pivot-apply").getByRole("button", { name: "Apply pivot" }).click();

    await expect(preview.getByTestId("data-table-row-pivot:East")).toBeVisible({
      timeout: 15_000,
    });
    await expect(preview.locator("th").filter({ hasText: /amount|East|West/i })).toHaveCount(1, {
      timeout: 15_000,
    });
  });
});
