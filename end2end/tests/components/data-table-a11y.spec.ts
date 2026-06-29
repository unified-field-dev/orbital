import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table accessibility", () => {
  test("default preview passes axe audit", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    const preview = page.getByTestId("data-table-preview");
    await preview.scrollIntoViewIfNeeded();

    const results = await new AxeBuilder({ page })
      .include("[data-testid='data-table-preview']")
      .analyze();

    const serious = results.violations.filter(
      (v) => v.impact === "critical" || v.impact === "serious",
    );
    expect(serious).toEqual([]);
  });

  test("keyboard focus moves on arrow key in grid", async ({ page }) => {
    await openComponentPreview(page, "data-table");
    const preview = page.getByTestId("data-table-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-cell-1-name").click();
    await preview.getByTestId("data-table-grid-focus").focus();
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");

    const focused = preview.locator(".orbital-data-table__cell-focus[tabindex='0']");
    await expect(focused).toBeVisible();
  });
});
