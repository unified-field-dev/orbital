import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("avatar primitive preview", () => {
  test("AV-01 name-derived initials", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    const avatar = page.getByTestId("avatar-preview").locator(".orbital-avatar");
    await expect(avatar).toHaveAttribute("role", "img");
    await expect(avatar).toHaveAttribute("aria-label", "Jane Doe");
    await expect(avatar.locator(".orbital-avatar__initials")).toHaveText("JD");
    await expect(avatar).not.toHaveClass(/orbital-avatar--color-neutral/);
    await expect(avatar).toHaveClass(/orbital-avatar--color-tangerine/);
    const initialsBg = await avatar
      .locator(".orbital-avatar__initials")
      .evaluate((el) => getComputedStyle(el).backgroundColor);
    const cardBg = await page
      .getByTestId("avatar-preview")
      .evaluate((el) => {
        let node = el.parentElement;
        while (node) {
          const bg = getComputedStyle(node).backgroundColor;
          if (bg !== "rgba(0, 0, 0, 0)" && bg !== "transparent") {
            return bg;
          }
          node = node.parentElement;
        }
        return "";
      });
    expect(initialsBg).not.toBe(cardBg);
    const box = await avatar.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
    expect(box!.height).toBeGreaterThan(0);
  });

  test("AV-02 profile image", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-image"]);
    const img = page.getByTestId("avatar-image").locator(".orbital-avatar__image");
    await expect(img).toHaveAttribute("src", /picsum\.photos/);
  });

  test("AV-03 custom initials", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-initials"]);
    await expect(
      page.getByTestId("avatar-initials").locator(".orbital-avatar__initials"),
    ).toHaveText("AB");
  });

  test("AV-04 size matrix dimensions", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-sizes"]);
    const small = page.getByTestId("avatar-size-24").locator(".orbital-avatar");
    const large = page.getByTestId("avatar-size-56").locator(".orbital-avatar");
    const smallBox = await small.boundingBox();
    const largeBox = await large.boundingBox();
    expect(smallBox).not.toBeNull();
    expect(largeBox).not.toBeNull();
    expect(largeBox!.width).toBeGreaterThan(smallBox!.width);
    expect(largeBox!.height).toBeGreaterThan(smallBox!.height);
  });

  test("AV-05 shape modifier classes", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-shapes"]);
    await expect(
      page.getByTestId("avatar-shape-circular").locator(".orbital-avatar"),
    ).toHaveClass(/orbital-avatar--circular/);
    await expect(
      page.getByTestId("avatar-shape-square").locator(".orbital-avatar"),
    ).toHaveClass(/orbital-avatar--square/);
  });

  test("AV-06 neutral surface token", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-theme"]);
    await expect(
      page.getByTestId("avatar-theme").locator(".orbital-avatar"),
    ).toHaveClass(/orbital-avatar--color-neutral/);
    await expectNonEmptyResolvedStyle(page, "avatar-theme", "background-color", {
      childSelector: ".orbital-avatar__initials",
    });
  });

  test("AV-07 colorful initials differ by name", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-colorful"]);
    const a = page.getByTestId("avatar-colorful-a").locator(".orbital-avatar__initials");
    const b = page.getByTestId("avatar-colorful-b").locator(".orbital-avatar__initials");
    const aColor = await a.evaluate((el) => getComputedStyle(el).backgroundColor);
    const bColor = await b.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(aColor).not.toBe(bColor);
  });

  test("AV-08 named color modifier classes", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-named-colors"]);
    await expect(
      page.getByTestId("avatar-color-crimson").locator(".orbital-avatar"),
    ).toHaveClass(/orbital-avatar--color-crimson/);
    await expect(
      page.getByTestId("avatar-color-tangerine").locator(".orbital-avatar"),
    ).toHaveClass(/orbital-avatar--color-tangerine/);
  });

  test("AV-09 brand color modifier class", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-brand-color"]);
    await expect(
      page.getByTestId("avatar-brand-color").locator(".orbital-avatar"),
    ).toHaveClass(/orbital-avatar--color-brand/);
  });

  test("AV-10 with presence badge", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-with-presence"]);
    await expect(
      page.getByTestId("avatar-with-presence").locator(".orbital-presence-badge__indicator"),
    ).toBeVisible();
  });

  test("AV-11 with counter badge", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-with-counter"]);
    await expect(
      page.getByTestId("avatar-with-counter").locator(".orbital-badge"),
    ).toHaveText("7");
  });

  test("AV-12 avatar group composition", async ({ page }) => {
    await openComponentPreview(page, "avatar");
    await expectPreviewVariants(page, ["avatar-group-preview"]);
    await expect(
      page.getByTestId("avatar-group-preview").locator(".orbital-avatar-group__overflow"),
    ).toHaveText("+2");
  });
});
