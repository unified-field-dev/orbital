import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("date-pickers-shortcuts preview", () => {
  test("clicking Today preset updates bind readout", async ({ page }) => {
    await openComponentPreview(page, "date-pickers-shortcuts", "date-pickers-shortcuts-preview");
    const value = page.getByTestId("date-pickers-shortcuts-value");
    await expect(value).toHaveText("none");
    await page.getByTestId("picker-shortcuts-bar").getByRole("button", { name: "Today" }).click();
    await expect(value).not.toHaveText("none");
  });
});
