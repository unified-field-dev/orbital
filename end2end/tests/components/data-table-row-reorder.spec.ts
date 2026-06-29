import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
async function dragRow(
  page: import("@playwright/test").Page,
  source: import("@playwright/test").Locator,
  target: import("@playwright/test").Locator,
  rowId: string,
) {
  await source.evaluate((el, draggedId) => {
    const dt = new DataTransfer();
    dt.setData("text/plain", draggedId);
    el.dispatchEvent(
      new DragEvent("dragstart", { bubbles: true, cancelable: true, dataTransfer: dt }),
    );
  }, rowId);

  await target.evaluate((el, draggedId) => {
    const dt = new DataTransfer();
    dt.setData("text/plain", draggedId);
    el.dispatchEvent(
      new DragEvent("dragover", { bubbles: true, cancelable: true, dataTransfer: dt }),
    );
    el.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer: dt }));
  }, rowId);

  await source.evaluate((el) => {
    el.dispatchEvent(new DragEvent("dragend", { bubbles: true, cancelable: true }));
  });
}

test.describe("data-table row reorder", () => {
  test("drag row reorders visible rows", async ({ page }) => {
    await openComponentPreview(page, "data-table-rows", "data-table-row-reorder-preview");
    const preview = page.getByTestId("data-table-row-reorder-preview");
    await preview.scrollIntoViewIfNeeded();

    const firstRowText = () =>
      preview.locator("tbody tr").first().locator("td").last().innerText();

    expect(await firstRowText()).toContain("Alpha");

    const source = preview.getByTestId("data-table-row-drag-3");
    const target = preview.getByTestId("data-table-row-1");

    await dragRow(page, source, target, "3");

    await expect.poll(firstRowText).toContain("Gamma");
  });
});
