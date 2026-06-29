import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("checkbox primitive preview", () => {

  test("CB-01: checked preview starts checked", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await expect(page.getByTestId("checkbox-preview").locator("input[type=checkbox]")).toBeChecked();
    await expect(page.getByTestId("checkbox-preview").locator(".orbital-checkbox--checked")).toBeVisible();
  });

  test("CB-02: unchecked preview starts unchecked", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await page.getByTestId("checkbox-unchecked").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("checkbox-unchecked").locator("input[type=checkbox]")).not.toBeChecked();
  });

  test("CB-03: click toggles unchecked to checked", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await page.getByTestId("checkbox-unchecked").scrollIntoViewIfNeeded();
    const input = page.getByTestId("checkbox-unchecked").locator("input[type=checkbox]");
    await input.click({ force: true });
    await expect(input).toBeChecked();
  });

  test("CB-04: disabled fieldset prevents toggle", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await page.getByTestId("checkbox-disabled").scrollIntoViewIfNeeded();
    const input = page.getByTestId("checkbox-disabled").locator("input[type=checkbox]");
    await expect(input).toBeChecked();
    await input.click({ force: true });
    await expect(input).toBeChecked();
  });

  test("CB-05: size matrix large indicator is bigger than medium", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await page.getByTestId("checkbox-size-matrix").scrollIntoViewIfNeeded();
    const medium = page.getByTestId("checkbox-size-medium").locator(".orbital-checkbox__indicator");
    const large = page.getByTestId("checkbox-size-large").locator(".orbital-checkbox__indicator");
    const mediumW = await medium.evaluate((el) => el.getBoundingClientRect().width);
    const largeW = await large.evaluate((el) => el.getBoundingClientRect().width);
    expect(largeW).toBeGreaterThan(mediumW);
  });

  test("CB-06: checked indicator uses themed brand compound background", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await expectNonEmptyResolvedStyle(page, "checkbox-preview", "background-color", {
      childSelector: ".orbital-checkbox--checked .orbital-checkbox__indicator",
    });
  });

  test("CB-07: field wrapper associates form label with checkbox", async ({ page }) => {
    await openComponentPreview(page, "checkbox");
    await page.getByTestId("checkbox-field").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("checkbox-field").locator(".orbital-field__label")).toHaveText("Terms");
    const inlineFor = await page.getByTestId("checkbox-field").locator(".orbital-checkbox__label").getAttribute("for");
    const inputId = await page.getByTestId("checkbox-field").locator("input[type=checkbox]").getAttribute("id");
    expect(inlineFor).toEqual(inputId);
  });
});
