import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("persona primitive preview", () => {

  test("PE-01: default persona renders name and avatar", async ({ page }) => {
    await openComponentPreview(page, "persona");
    const preview = page.getByTestId("persona-preview");
    await expect(preview.locator(".orbital-persona")).toBeVisible({ timeout: 30_000 });
    await expect(preview.locator(".orbital-persona__primary-text")).toHaveText("Jane Doe");
    await expect(preview.locator(".orbital-avatar__initials")).toHaveText("JD");
  });

  test("PE-02: secondary text slot renders beneath primary", async ({ page }) => {
    await openComponentPreview(page, "persona");
    await page.getByTestId("persona-secondary").scrollIntoViewIfNeeded();
    const persona = page.getByTestId("persona-secondary").locator(".orbital-persona");
    await expect(persona.locator(".orbital-persona__primary-text")).toHaveText("Jane Doe");
    await expect(persona.locator(".orbital-persona__secondary-text")).toHaveText("Engineer");
  });

  test("PE-03: full text stack renders four lines", async ({ page }) => {
    await openComponentPreview(page, "persona");
    await page.getByTestId("persona-stack").scrollIntoViewIfNeeded();
    const persona = page.getByTestId("persona-stack").locator(".orbital-persona");
    await expect(persona.locator(".orbital-persona__primary-text")).toHaveText("Jane Doe");
    await expect(persona.locator(".orbital-persona__secondary-text")).toHaveText("Engineer");
    await expect(persona.locator(".orbital-persona__tertiary-text")).toHaveText("Product");
    await expect(persona.locator(".orbital-persona__quaternary-text")).toHaveText("Seattle");
  });

  test("PE-04: size presets scale avatar dimensions", async ({ page }) => {
    await openComponentPreview(page, "persona");
    await page.getByTestId("persona-sizes").scrollIntoViewIfNeeded();
    const avatars = page.getByTestId("persona-sizes").locator(".orbital-avatar");
    await expect(avatars).toHaveCount(3);
    const widths = await avatars.evaluateAll(
      (els) => els.map((el) => el.getBoundingClientRect().width),
    );
    expect(widths[0]).toBeLessThan(widths[1]);
    expect(widths[1]).toBeLessThan(widths[2]);
  });

  test("PE-05: center alignment applies center modifier class", async ({ page }) => {
    await openComponentPreview(page, "persona");
    await page.getByTestId("persona-align-center").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("persona-align-center").locator(".orbital-persona")).toHaveClass(
      /orbital-persona--center/,
    );
  });

  test("PE-06: text position variants apply before and below classes", async ({ page }) => {
    await openComponentPreview(page, "persona");
    await page.getByTestId("persona-position").scrollIntoViewIfNeeded();
    const personas = page.getByTestId("persona-position").locator(".orbital-persona");
    await expect(personas.nth(0)).toHaveClass(/orbital-persona--before/);
    await expect(personas.nth(1)).toHaveClass(/orbital-persona--below/);
  });

  test("PE-07: avatar image renders when avatar_src is provided", async ({ page }) => {
    await openComponentPreview(page, "persona");
    await page.getByTestId("persona-avatar").scrollIntoViewIfNeeded();
    const image = page.getByTestId("persona-avatar").locator(".orbital-avatar__image");
    await expect(image).toBeVisible();
    await expect(image).toHaveAttribute("src", /pravatar\.cc/);
  });

  test("PE-08: default persona has horizontal gap between avatar and primary text", async ({ page }) => {
    await openComponentPreview(page, "persona");
    const preview = page.getByTestId("persona-preview");
    await expect(preview.locator(".orbital-persona")).toBeVisible({ timeout: 30_000 });

    const gap = await preview.evaluate(() => {
      const avatar = document.querySelector(
        '[data-testid="persona-preview"] .orbital-avatar',
      );
      const primary = document.querySelector(
        '[data-testid="persona-preview"] .orbital-persona__primary-text',
      );
      if (!avatar || !primary) return 0;
      const avatarRect = avatar.getBoundingClientRect();
      const primaryRect = primary.getBoundingClientRect();
      return primaryRect.left - avatarRect.right;
    });

    expect(gap).toBeGreaterThan(0);
  });
});
