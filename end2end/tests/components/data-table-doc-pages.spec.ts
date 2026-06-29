import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
const DOC_PAGES: Array<[string, string]> = [
  ["data-table", "data-table-preview"],
  ["data-table-column-definition", "data-table-custom-columns-preview"],
  ["data-table-columns", "data-table-column-sizing-preview"],
  ["data-table-rows", "data-table-get-row-id-preview"],
  ["data-table-editing", "data-table-editing-preview"],
  ["data-table-sorting-filtering", "data-table-multi-sort-preview"],
  ["data-table-data-source", "data-table-server-preview"],
  ["data-table-data-source", "data-table-pagination-preview"],
  ["data-table-selection", "data-table-range-select-preview"],
  ["data-table-export", "data-table-export-preview"],
  ["data-table-rendering", "data-table-overlays-preview"],
  ["data-table-advanced", "data-table-tree-preview"],
  ["data-table-state", "data-table-controlled-preview"],
];

test.describe("data-table doc pages smoke", () => {
  for (const [slug, testId] of DOC_PAGES) {
    test(`${slug} (${testId}) renders default example`, async ({ page }) => {
      await openComponentPreview(page, slug, testId);
      await expect(page.getByTestId(testId)).toBeVisible();
    });
  }
});
