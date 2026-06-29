import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("input primitive preview", () => {

  test("IN-01: default input accepts typing", async ({ page }) => {
    await openComponentPreview(page, "input");
    const input = page.getByTestId("input-preview").locator("input");
    await input.fill("Jane");
    await expect(input).toHaveValue("Jane");
  });

  test("IN-02: prefix and suffix slots render", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-affix").scrollIntoViewIfNeeded();
    const affix = page.getByTestId("input-affix");
    await expect(affix.locator(".orbital-input__prefix svg")).toBeVisible();
    await expect(affix.locator(".orbital-input__suffix")).toHaveText(".00");
  });

  test("IN-03: size matrix heights differ", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-sizes").scrollIntoViewIfNeeded();
    const small = page.getByTestId("input-sizes").locator(".orbital-input--small").first();
    const large = page.getByTestId("input-sizes").locator(".orbital-input--large").first();
    const smallH = await small.evaluate((el) => el.getBoundingClientRect().height);
    const largeH = await large.evaluate((el) => el.getBoundingClientRect().height);
    expect(largeH).toBeGreaterThan(smallH);
  });

  test("IN-04: disabled input has disabled attribute", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-disabled").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("input-disabled").locator("input")).toBeDisabled();
  });

  test("IN-05: placeholder attribute is set", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-placeholder").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("input-placeholder").locator("input"))
      .toHaveAttribute("placeholder", "This is a placeholder");
  });

  test("IN-06: autofocus receives focus on mount", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-autofocus").scrollIntoViewIfNeeded();
    const input = page.getByTestId("input-autofocus").locator("input");
    await expect(input).toBeFocused({ timeout: 5_000 });
  });

  test("IN-07: email input uses type email", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-email").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("input-email").locator("input")).toHaveAttribute("type", "email");
  });

  test("IN-08: required validation shows error on blur", async ({ page }) => {
    await openComponentPreview(page, "input");
    await page.getByTestId("input-validation").scrollIntoViewIfNeeded();
    const input = page.getByTestId("input-validation").locator("input");
    await input.focus();
    await input.blur();
    await expect(page.getByTestId("input-validation").locator(".orbital-field__validation-message"))
      .toBeVisible();
  });
});
