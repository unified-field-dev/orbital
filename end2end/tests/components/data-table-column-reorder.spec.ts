import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table column reorder", () => {
  test.use({ viewport: { width: 1400, height: 900 } });

  test("drag header reorders columns", async ({ page }) => {
    await openComponentPreview(page, "data-table-columns", "data-table-column-reorder-preview");
    const preview = page.getByTestId("data-table-column-reorder-preview");
    await preview.scrollIntoViewIfNeeded();

    const headerTexts = () =>
      preview.locator("thead tr:last-child th").evaluateAll((cells) =>
        cells
          .map((c) => c.textContent?.replace(/[⋮⧩↑↓\s]/g, "").trim() ?? "")
          .filter((t) => t.length > 0)
      );

    const before = await headerTexts();
    expect(before[0]).toContain("Name");

    const source = preview.getByTestId("data-table-header-drag-dept");
    const target = preview.getByTestId("data-table-header-drag-name");

    const sourceBox = await source.boundingBox();
    const targetBox = await target.boundingBox();
    expect(sourceBox).not.toBeNull();
    expect(targetBox).not.toBeNull();

    const sx = sourceBox!.x + sourceBox!.width / 2;
    const sy = sourceBox!.y + sourceBox!.height / 2;
    const tx = targetBox!.x + targetBox!.width / 2;
    const ty = targetBox!.y + targetBox!.height / 2;

    const scroll = preview.locator("[data-testid='data-table-scroll']");
    await expect(scroll).toHaveAttribute("data-column-order", "name,role,dept");

    await page.mouse.move(sx, sy);
    await page.mouse.down();
    await expect(source).toHaveClass(/orbital-data-table__header--dragging/);
    await page.mouse.move(tx, ty, { steps: 5 });
    await page.mouse.up();

    await expect
      .poll(async () => scroll.getAttribute("data-column-order"))
      .toContain("dept,name");

    await expect
      .poll(async () => (await headerTexts())[0])
      .toContain("Department");

    await preview.getByTestId("data-table-column-picker-trigger").click();
    const pickerRows = page.locator("[data-column-picker-field]");
    await expect(pickerRows.first()).toHaveAttribute("data-column-picker-field", "dept");
  });
});
