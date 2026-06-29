import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table state export/restore", () => {
  test("export, reset, and restore round-trip sort state", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-state-export-preview");
    const preview = page.getByTestId("data-table-state-export-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByText("Name").click();
    await preview.getByText("Name").click();
    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();

    await preview.getByTestId("export-state").click();
    await expect(preview.getByTestId("serialized-state")).not.toBeEmpty();

    await preview.getByTestId("reset-state").click();
    await expect(preview.getByTestId("data-table-sort-desc")).toHaveCount(0);

    await preview.getByTestId("restore-state").click();
    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();
  });
});
