import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import fs from "node:fs";

test.describe("data-table Excel export", () => {
  test("export menu shows Excel option and downloads xlsx zip", async ({ page }) => {
    await openComponentPreview(page, "data-table-export", "data-table-excel-export-preview");
    const preview = page.getByTestId("data-table-excel-export-preview");
    await preview.scrollIntoViewIfNeeded();

    await preview.getByTestId("data-table-export-menu").click();
    await expect(page.getByRole("menuitem", { name: "Download Excel" })).toBeVisible();

    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("menuitem", { name: "Download Excel" }).click();

    const download = await downloadPromise;
    expect(download.suggestedFilename()).toBe("export.xlsx");

    const path = await download.path();
    expect(path).toBeTruthy();
    const bytes = fs.readFileSync(path!);
    expect(bytes.subarray(0, 4).toString("binary")).toBe("PK\u0003\u0004");
  });
});
