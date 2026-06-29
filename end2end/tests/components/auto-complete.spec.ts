import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("auto-complete primitive preview", () => {

  test("AC-01: autocomplete preview renders", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await expect(page.getByTestId("auto-complete-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("AC-02: select callback example is present", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await page.getByTestId("auto-complete-select").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("auto-complete-select").locator("input")).toBeVisible();
  });

  test("AC-03: disabled autocomplete input is non-editable", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await page.getByTestId("auto-complete-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("auto-complete-disabled").locator("input")).toBeDisabled();
  });

  test("AC-04: field label is associated with autocomplete input", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await page.getByTestId("auto-complete-field").scrollIntoViewIfNeeded();
    const labelFor = await page.getByTestId("auto-complete-field").locator("label").getAttribute("for");
    const inputId = await page.getByTestId("auto-complete-field").locator("input").getAttribute("id");
    expect(labelFor).toEqual(inputId);
  });

  test("AC-05: size matrix large input is taller than small", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await page.getByTestId("auto-complete-size-matrix").scrollIntoViewIfNeeded();
    const small = page.getByTestId("auto-complete-size-small").locator(".orbital-input");
    const large = page.getByTestId("auto-complete-size-large").locator(".orbital-input");
    const smallH = await small.evaluate((el) => el.getBoundingClientRect().height);
    const largeH = await large.evaluate((el) => el.getBoundingClientRect().height);
    expect(largeH).toBeGreaterThan(smallH);
  });

  test("AC-06: arrow and enter pick suggestion", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    const input = page.getByTestId("auto-complete-preview").locator("input");
    await input.click();
    await input.press("ArrowDown");
    await expect(page.locator(".orbital-auto-complete-option[data-activedescendant-focusvisible]").first())
      .toBeVisible();
    await input.press("Enter");
    await expect(input).toHaveValue("Alpha");
  });

  test("AC-07: typing filters options by value", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    const input = page.getByTestId("auto-complete-preview").locator("input");
    await input.click();
    await input.fill("Beta");
    await expect(
      page.locator(".orbital-auto-complete-option:not(.orbital-auto-complete-option--hidden)", { hasText: "Alpha" }),
    ).toHaveCount(0);
    await expect(
      page.locator(".orbital-auto-complete-option:not(.orbital-auto-complete-option--hidden)", { hasText: "Beta" }),
    ).toBeVisible();
    await input.fill("Be");
    await expect(
      page.locator(".orbital-auto-complete-option:not(.orbital-auto-complete-option--hidden)", { hasText: "Beta" }),
    ).toBeVisible();
  });

  test("AC-08: clear after select resets input value", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await page.getByTestId("auto-complete-clear").scrollIntoViewIfNeeded();
    const input = page.getByTestId("auto-complete-clear").locator("input");
    await input.click();
    await input.press("ArrowDown");
    await input.press("Enter");
    await expect(input).toHaveValue("");
  });

  test("AC-09: blur after select removes focus from input", async ({ page }) => {
    await openComponentPreview(page, "auto-complete");
    await page.getByTestId("auto-complete-blur").scrollIntoViewIfNeeded();
    const input = page.getByTestId("auto-complete-blur").locator("input");
    await input.click();
    await input.press("ArrowDown");
    await input.press("Enter");
    await expect(input).not.toBeFocused();
  });
});
