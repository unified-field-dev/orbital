import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollPreviewMain, previewMainScrollTop } from "../lib/preview/navigation";
async function scrollUntilBackToTopVisible(
  page: import("@playwright/test").Page,
  containerTestId: string,
  scrollTop = 600,
  buttonTestId = "back-to-top-button",
) {
  await scrollPreviewMain(page, scrollTop);
  await expect(page.getByTestId(containerTestId).getByTestId(buttonTestId)).toBeVisible({
    timeout: 10_000,
  });
}

test.describe("back-to-top component preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "back-to-top");
    await scrollPreviewMain(page, 0);
  });

  test("BTT-01 default scrolls preview back to top", async ({ page }) => {
    const container = page.getByTestId("back-to-top-preview");
    await expect(container.getByTestId("back-to-top-button")).toBeHidden();
    await scrollUntilBackToTopVisible(page, "back-to-top-preview");
    const button = container.getByTestId("back-to-top-button");
    const box = await button.boundingBox();
    expect(box?.width ?? 0).toBeGreaterThan(0);
    expect(box?.height ?? 0).toBeGreaterThan(0);
    const position = await button.evaluate((el) => getComputedStyle(el).position);
    expect(position).toBe("fixed");
    await expect(button).toBeInViewport();
    await button.click({ force: true });
    await page.waitForTimeout(500);
    expect(await previewMainScrollTop(page)).toBeLessThan(50);
  });

  test("BTT-02 custom position", async ({ page }) => {
    await expectPreviewVariants(page, ["back-to-top-position"]);
    const container = page.getByTestId("back-to-top-position");
    await scrollUntilBackToTopVisible(page, "back-to-top-position", 600, "back-to-top-position-button");
    const style = await container.getByTestId("back-to-top-position-button").evaluate((el) => ({
      right: getComputedStyle(el).right,
      bottom: getComputedStyle(el).bottom,
    }));
    expect(style.right).toBe("40px");
    expect(style.bottom).toBe("110px");
  });

  test("BTT-03 visibility height", async ({ page }) => {
    await expectPreviewVariants(page, ["back-to-top-visibility"]);
    const container = page.getByTestId("back-to-top-visibility");
    await scrollPreviewMain(page, 250);
    await expect(container.getByTestId("back-to-top-visibility-button")).toBeHidden();
    await scrollPreviewMain(page, 450);
    await expect(container.getByTestId("back-to-top-visibility-button")).toBeVisible({ timeout: 10_000 });
    await expect(container.getByTestId("back-to-top-visibility-button")).toBeInViewport();
  });
});
