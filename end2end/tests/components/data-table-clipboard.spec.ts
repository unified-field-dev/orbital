import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.use({
  permissions: ["clipboard-read", "clipboard-write"],
});

test.describe("data-table clipboard", () => {
  test("copy cell range writes TSV to clipboard", async ({ page, context }) => {
    await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    await openComponentPreview(page, "data-table-export", "data-table-clipboard-preview");
    const preview = page.getByTestId("data-table-clipboard-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-cell-1-name").click();
    await preview.getByTestId("data-table-cell-2-name").click({ modifiers: ["Shift"] });

    await preview.getByTestId("data-table-grid-focus").focus();
    await page.keyboard.press("ControlOrMeta+C");

    const clipboardText = await page.evaluate(async () => {
      return navigator.clipboard.readText();
    });

    expect(clipboardText).toContain("Ada");
    expect(clipboardText).toContain("Grace");
  });
});
