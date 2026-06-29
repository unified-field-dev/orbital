import type { Locator, Page } from "@playwright/test";
import { expect } from "@playwright/test";

/** Read a CSS custom property on an element matching selector. */
export async function getCssVariable(
  page: Page,
  selector: string,
  varName: string,
): Promise<string> {
  return page.locator(selector).first().evaluate(
    (el, name) => getComputedStyle(el).getPropertyValue(name).trim(),
    varName,
  );
}

/** Assert computed styles on element found by testid. */
export async function expectComputedStyle(
  page: Page,
  testId: string,
  styles: Record<string, string | RegExp>,
  options?: { childSelector?: string },
): Promise<void> {
  const root = page.getByTestId(testId);
  const target = options?.childSelector
    ? root.locator(options.childSelector).first()
    : root;

  for (const [property, expected] of Object.entries(styles)) {
    if (expected instanceof RegExp) {
      await expect(target).toHaveCSS(property, expected);
    } else {
      await expect(target).toHaveCSS(property, expected);
    }
  }
}

/** Toggle a control and assert a CSS var changed on a scope element. */
export async function expectThemeVarChange(
  page: Page,
  opts: {
    toggleTestId: string;
    scopeSelector: string;
    cssVar: string;
    before?: string | RegExp;
    after?: string | RegExp;
  },
): Promise<void> {
  const scope = page.locator(opts.scopeSelector).first();
  const readVar = () =>
    scope.evaluate(
      (el, name) => getComputedStyle(el).getPropertyValue(name).trim(),
      opts.cssVar,
    );

  const before = await readVar();
  if (opts.before !== undefined) {
    if (opts.before instanceof RegExp) {
      expect(before).toMatch(opts.before);
    } else {
      expect(before).toBe(opts.before);
    }
  }

  await page.getByTestId(opts.toggleTestId).click();

  const after = await readVar();
  if (opts.after !== undefined) {
    if (opts.after instanceof RegExp) {
      expect(after).toMatch(opts.after);
    } else {
      expect(after).toBe(opts.after);
    }
  }
  expect(after).not.toEqual(before);
}

const TRANSPARENT = /^(transparent|rgba\(0,\s*0,\s*0,\s*0\)|initial|none)$/i;

/** Assert a ::before/::after pseudo-element has a non-empty resolved style. */
export async function expectNonEmptyPseudoStyle(
  locator: Locator,
  pseudo: "::before" | "::after",
  property: string,
): Promise<void> {
  const value = await locator.first().evaluate(
    (el, args) => getComputedStyle(el, args.pseudo).getPropertyValue(args.property).trim(),
    { pseudo, property },
  );
  expect(value.length).toBeGreaterThan(0);
  expect(value).not.toMatch(TRANSPARENT);
}

/** Assert a computed style is non-empty and not transparent/initial. */
export async function expectNonEmptyResolvedStyle(
  page: Page,
  testId: string,
  property: string,
  options?: { childSelector?: string },
): Promise<void> {
  const root = page.getByTestId(testId);
  const target = options?.childSelector
    ? root.locator(options.childSelector).first()
    : root;

  const value = await target.evaluate(
    (el, prop) => getComputedStyle(el).getPropertyValue(prop).trim(),
    property,
  );
  expect(value.length).toBeGreaterThan(0);
  expect(value).not.toMatch(TRANSPARENT);
}

/** Assert element has key visible surface styles resolved. */
export async function expectSurfaceStyled(
  page: Page,
  selector: string,
): Promise<void> {
  const target = page.locator(selector).first();
  await expect(target).toBeVisible();
  const [background, borderColor, boxShadow] = await Promise.all([
    target.evaluate((el) => getComputedStyle(el).backgroundColor.trim()),
    target.evaluate((el) => getComputedStyle(el).borderColor.trim()),
    target.evaluate((el) => getComputedStyle(el).boxShadow.trim()),
  ]);

  const hasSurface =
    !TRANSPARENT.test(background) ||
    !TRANSPARENT.test(borderColor) ||
    (boxShadow.length > 0 && boxShadow.toLowerCase() !== "none");
  expect(hasSurface).toBeTruthy();
}

/** Assert two locators resolve different computed values for a property. */
export async function expectDistinctStyle(
  a: Locator,
  b: Locator,
  property: string,
): Promise<void> {
  const [aValue, bValue] = await Promise.all([
    a.first().evaluate(
      (el, prop) => getComputedStyle(el).getPropertyValue(prop).trim(),
      property,
    ),
    b.first().evaluate(
      (el, prop) => getComputedStyle(el).getPropertyValue(prop).trim(),
      property,
    ),
  ]);

  expect(aValue.length).toBeGreaterThan(0);
  expect(bValue.length).toBeGreaterThan(0);
  expect(aValue).not.toBe(bValue);
}
