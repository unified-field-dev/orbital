import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table controlled models", () => {
  test("external controls and header sort sync with parent state", async ({ page }) => {
    await openComponentPreview(page, "data-table-state", "data-table-controlled-preview");
    const preview = page.getByTestId("data-table-controlled-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("controlled-sort-desc").click();
    await expect(preview.getByTestId("controlled-sort-label")).toContainText("name desc");
    await expect(preview.getByTestId("data-table-sort-desc")).toBeVisible();

    await preview.getByText("Name↓").click();
    await expect(preview.getByTestId("controlled-sort-label")).toContainText("name asc");

    await preview.getByTestId("controlled-filter-admin").click();
    await expect(preview.getByText("Ada")).toBeVisible();
    await expect(preview.getByText("Grace")).toHaveCount(0);

    await preview.getByTestId("controlled-page-2").click();
    await expect(preview.getByTestId("data-table-pagination").getByRole("button", { name: "2" })).toHaveClass(
      /orbital-button--primary/,
    );
  });
});
