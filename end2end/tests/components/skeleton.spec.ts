import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("skeleton primitive preview", () => {

  test("SK-01: text block skeleton exposes busy progressbar", async ({ page }) => {
    await openComponentPreview(page, "skeleton");
    const skeleton = page.getByTestId("skeleton-preview").locator(".orbital-skeleton");
    await expect(skeleton).toHaveAttribute("role", "progressbar");
    await expect(skeleton).toHaveAttribute("aria-busy", "true");
    await expect(page.getByTestId("skeleton-preview").locator(".orbital-skeleton-item")).toHaveCount(2);
  });

  test("SK-02: avatar row skeleton preserves row layout and circle size", async ({ page }) => {
    await openComponentPreview(page, "skeleton");
    await page.getByTestId("skeleton-row").scrollIntoViewIfNeeded();
    const row = page.getByTestId("skeleton-row");
    await expect(row.locator(".orbital-skeleton-item")).toHaveCount(3);
    const display = await row.evaluate((el) => getComputedStyle(el).display);
    expect(display).toMatch(/flex|grid/);

    const avatar = row.locator(".orbital-skeleton-item--circle").first();
    const avatarBox = await avatar.boundingBox();
    expect(avatarBox?.width).toBeCloseTo(40, 0);
    expect(avatarBox?.height).toBeCloseTo(40, 0);
  });

  test("SK-03: card skeleton media block is 120px tall", async ({ page }) => {
    await openComponentPreview(page, "skeleton");
    await page.getByTestId("skeleton-card").scrollIntoViewIfNeeded();
    const card = page.getByTestId("skeleton-card");
    await expect(card.locator(".orbital-skeleton-item")).toHaveCount(2);

    const heights = await card.locator(".orbital-skeleton-item").evaluateAll(
      (els) => els.map((el) => el.getBoundingClientRect().height),
    );
    expect(Math.max(...heights)).toBeCloseTo(120, 0);
    expect(Math.min(...heights)).toBeCloseTo(16, 0);
  });

  test("SK-04: skeleton item shimmer animation is active", async ({ page }) => {
    await openComponentPreview(page, "skeleton");
    const animationName = await page.getByTestId("skeleton-preview").locator(".orbital-skeleton-item").first()
      .evaluate((el) => getComputedStyle(el).animationName);
    expect(animationName).not.toBe("none");
  });
});
