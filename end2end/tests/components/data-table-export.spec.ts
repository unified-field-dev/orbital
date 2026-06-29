import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import fs from "node:fs";

test.describe("data-table CSV export", () => {
  test("export menu downloads CSV with headers and row data", async ({ page }) => {
    await openComponentPreview(page, "data-table-export", "data-table-export-preview");
    const preview = page.getByTestId("data-table-export-preview");
    await preview.scrollIntoViewIfNeeded();

    const downloadPromise = page.waitForEvent("download");
    await preview.getByTestId("data-table-export-menu").click();
    await page.getByRole("menuitem", { name: "Download CSV" }).click();

    const download = await downloadPromise;
    expect(download.suggestedFilename()).toBe("export.csv");

    const path = await download.path();
    expect(path).toBeTruthy();
    const content = fs.readFileSync(path!, "utf8");
    expect(content).toContain("Name");
    expect(content).toContain("Role");
    expect(content).toContain("Ada");
    expect(content).toContain("Admin");
    expect(content).toContain("Grace");
    expect(content).toContain("Editor");
  });
});
