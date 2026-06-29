import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("dropdown primitive preview", () => {

  test("DD-01: basic dropdown changes selection", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    const select = page.getByTestId("dropdown-preview").locator("select");
    await select.selectOption("b");
    await expect(select).toHaveValue("b");
  });

  test("DD-02: disabled dropdown", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    await page.getByTestId("dropdown-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("dropdown-disabled").locator("select")).toBeDisabled();
  });

  test("DD-03: dropdown in field associates label", async ({ page }) => {
    await openComponentPreview(page, "dropdown");
    await page.getByTestId("dropdown-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("dropdown-field").locator("label").getAttribute("for");
    const selectId = await page.getByTestId("dropdown-field").locator("select").getAttribute("id");
    expect(labelFor).toEqual(selectId);
    await expect(page.getByTestId("dropdown-field").locator("label")).toContainText("Status");
  });
});
