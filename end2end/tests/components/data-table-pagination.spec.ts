import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table pagination footer", () => {
  test("range label, page navigation, and page size selector", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-pagination-preview");
    const preview = page.getByTestId("data-table-pagination-preview");
    await preview.scrollIntoViewIfNeeded();

    const range = preview.getByTestId("data-table-pagination-range");
    await expect(range).toContainText("1–5 of 25");

    await preview.getByTestId("data-table-pagination").getByRole("button", { name: "2" }).click();
    await expect(range).toContainText("6–10 of 25");

    await preview.getByTestId("data-table-page-size").locator("select").selectOption("10");
    await expect(range).toContainText("1–10 of 25");
    await expect(
      preview.getByTestId("data-table-pagination").getByRole("button", { name: "1" }),
    ).toHaveClass(/orbital-button--primary/);
  });
});
