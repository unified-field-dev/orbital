import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("data-table density", () => {
  test("DT-06 density variant preview renders", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    await expectPreviewVariants(page, ["data-table-density"]);
    await expect(page.getByTestId("data-table-density")).toBeVisible();
  });

  test("DT-07 table root exposes density CSS variable", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    const preview = page.getByTestId("data-table-density");
    const root = preview.locator(".orbital-data-table").first();

    await expect(preview.getByTestId("theme-density-increase")).toBeVisible();

    const readRowHeight = () =>
      root.evaluate((el) =>
        getComputedStyle(el).getPropertyValue("--orbital-data-table-row-height").trim(),
      );

    const compactHeight = await readRowHeight();
    expect(["32px", "40px", "48px"]).toContain(compactHeight);
    await expect(root).toHaveClass(/orbital-data-table--density-/);

    await preview.getByTestId("theme-density-increase").click();
    await expect(preview.getByTestId("theme-density-value")).not.toContainText("Compact");
    const tallerHeight = await readRowHeight();
    expect(tallerHeight).not.toBe(compactHeight);
  });
});
