import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("presence-badge preview", () => {
  test("PB-01 default presence dot", async ({ page }) => {
    await openComponentPreview(page, "presence-badge");
    const indicator = page.getByTestId("presence-badge-preview").locator(".orbital-presence-badge__indicator");
    await expect(indicator).toHaveAttribute("aria-label", "Available");
    await expect(indicator).toHaveClass(/orbital-presence-badge__indicator--available/);
  });

  test("PB-02 on avatar", async ({ page }) => {
    await openComponentPreview(page, "presence-badge");
    await expectPreviewVariants(page, ["presence-badge-avatar"]);
    await expect(
      page.getByTestId("presence-badge-avatar").locator(".orbital-avatar"),
    ).toBeVisible();
    await expect(
      page.getByTestId("presence-badge-avatar").locator(".orbital-presence-badge__indicator"),
    ).toBeVisible();
  });

  test("PB-03 status matrix", async ({ page }) => {
    await openComponentPreview(page, "presence-badge");
    await expectPreviewVariants(page, ["presence-badge-statuses"]);
    await expect(
      page.getByTestId("presence-available").locator(".orbital-presence-badge__indicator--available"),
    ).toBeVisible();
    await expect(
      page.getByTestId("presence-away").locator(".orbital-presence-badge__indicator--away"),
    ).toBeVisible();
    await expect(
      page.getByTestId("presence-busy").locator(".orbital-presence-badge__indicator--busy"),
    ).toBeVisible();
    await expect(
      page.getByTestId("presence-offline").locator(".orbital-presence-badge__indicator--offline"),
    ).toBeVisible();
    await expect(
      page.getByTestId("presence-out-of-office").locator(".orbital-presence-badge__indicator--out-of-office"),
    ).toBeVisible();
    await expect(
      page.getByTestId("presence-unknown").locator(".orbital-presence-badge__indicator--unknown"),
    ).toBeVisible();

    const availableColor = await page
      .getByTestId("presence-available")
      .locator(".orbital-presence-badge__indicator")
      .evaluate((el) => getComputedStyle(el).backgroundColor);
    const awayColor = await page
      .getByTestId("presence-away")
      .locator(".orbital-presence-badge__indicator")
      .evaluate((el) => getComputedStyle(el).backgroundColor);
    const busyColor = await page
      .getByTestId("presence-busy")
      .locator(".orbital-presence-badge__indicator")
      .evaluate((el) => getComputedStyle(el).backgroundColor);

    expect(availableColor).not.toBe(awayColor);
    expect(availableColor).not.toBe(busyColor);
    expect(awayColor).not.toBe(busyColor);

    const awayIndicator = page
      .getByTestId("presence-away")
      .locator(".orbital-presence-badge__indicator");
    const awayBox = await awayIndicator.boundingBox();
    expect(awayBox).not.toBeNull();
    const topElementClass = await page.evaluate(
      ({ x, y }) => document.elementFromPoint(x, y)?.className ?? "",
      {
        x: awayBox!.x + awayBox!.width - 1,
        y: awayBox!.y + awayBox!.height / 2,
      },
    );
    expect(String(topElementClass)).toMatch(/orbital-presence-badge__indicator/);
  });
});
