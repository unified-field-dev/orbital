import { test, expect } from "@playwright/test";
import { documentScrollHeight, openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("scroll area preview", () => {
  test("renders default preview", async ({ page }) => {
    await openComponentPreview(page, "scroll-area", "scroll-area-default");
    await expect(page.getByTestId("scroll-area-default")).toBeVisible({ timeout: 30_000 });
    await expect(
      page.getByTestId("scroll-area-default").locator(".orbital-scroll-area").first(),
    ).toBeVisible();
  });

  test("shows documented examples", async ({ page }) => {
    await openComponentPreview(page, "scroll-area", "scroll-area-default");
    await expectPreviewVariants(page, [
      "scroll-area-default",
      "scroll-area-visible",
      "scroll-area-hidden",
      "scroll-area-horizontal",
    ]);
  });

  test("SA-01: default vertical scrollport overflows", async ({ page }) => {
    await openComponentPreview(page, "scroll-area", "scroll-area-default");
    const scrollport = page.getByTestId("scroll-area-default").locator(".orbital-scroll-area").first();
    await expect(scrollport).toBeVisible();
    const scrollHeight = await scrollport.evaluate((el) => el.scrollHeight);
    const clientHeight = await scrollport.evaluate((el) => el.clientHeight);
    expect(scrollHeight).toBeGreaterThan(clientHeight);
  });

  test("SA-02: hidden chrome suppresses scrollbar", async ({ page }) => {
    await openComponentPreview(page, "scroll-area", "scroll-area-default");
    await page.getByTestId("scroll-area-hidden").waitFor({ state: "attached" });
    await scrollIntoPreviewView(page.getByTestId("scroll-area-hidden"));
    const hidden = page.getByTestId("scroll-area-hidden").locator(".orbital-scroll-area").first();
    await expect(hidden).toBeVisible();
    await expect(hidden).toHaveClass(/orbital-scroll-area--hide-chrome/);
    const scrollbarWidth = await hidden.evaluate((el) => getComputedStyle(el).scrollbarWidth);
    expect(scrollbarWidth).toBe("none");
  });

  test("SA-03: page scrollport uses head-injected themed scrollbar", async ({ page }) => {
    await openComponentPreview(page, "scroll-area", "scroll-area-default");
    const pageScroll = page.locator(".orbital-layout__page-scroll").first();
    await expect(pageScroll).toBeVisible();

    const scrollHeight = await pageScroll.evaluate((el) => el.scrollHeight);
    const clientHeight = await pageScroll.evaluate((el) => el.clientHeight);
    expect(scrollHeight).toBeGreaterThan(clientHeight);

    const scrollbarWidth = await pageScroll.evaluate((el) => getComputedStyle(el).scrollbarWidth);
    expect(["thin", "auto"]).toContain(scrollbarWidth);

    const headStyle = page.locator("head style#orbital-style-orbital-scroll-area");
    await expect(headStyle).toHaveCount(1);
    const styleText = await headStyle.evaluate((el) => el.textContent ?? "");
    expect(styleText).toContain(".orbital-scroll-area::-webkit-scrollbar-thumb");
    expect(styleText).toContain("@supports not selector(::-webkit-scrollbar)");

    const docHeight = await documentScrollHeight(page);
    const viewportHeight = page.viewportSize()?.height ?? 0;
    expect(docHeight).toBeLessThanOrEqual(viewportHeight + 1);
  });
});
