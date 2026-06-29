import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("search-box primitive preview", () => {

  test("SB-01: search type and icon prefix", async ({ page }) => {
    await openComponentPreview(page, "search-box");
    const box = page.getByTestId("search-box-preview");
    await expect(box.locator("input")).toHaveAttribute("type", "search");
    await expect(box.locator(".orbital-input__prefix svg")).toBeVisible();
  });

  test("SB-02: disabled search box", async ({ page }) => {
    await openComponentPreview(page, "search-box");
    await page.getByTestId("search-box-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("search-box-disabled").locator("input")).toBeDisabled();
  });

  test("SB-03: field wrapper associates label", async ({ page }) => {
    await openComponentPreview(page, "search-box");
    await page.getByTestId("search-box-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("search-box-field").locator("label").getAttribute("for");
    const inputId = await page.getByTestId("search-box-field").locator("input").getAttribute("id");
    expect(labelFor).toEqual(inputId);
  });

  test("SB-04: custom placeholder text", async ({ page }) => {
    await openComponentPreview(page, "search-box");
    await page.getByTestId("search-box-placeholder").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("search-box-placeholder").locator("input"))
      .toHaveAttribute("placeholder", "Find people…");
  });
});
