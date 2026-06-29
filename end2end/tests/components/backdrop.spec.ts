import { test, expect, type Locator } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { previewUrl, waitForPreviewShell } from "../helpers";

async function expectNonTransparentBackground(locator: Locator) {
  const bg = await locator.evaluate((el) => getComputedStyle(el).backgroundColor);
  expect(bg).not.toBe("");
  expect(bg).not.toBe("rgba(0, 0, 0, 0)");
  expect(bg).not.toBe("transparent");
}

test.describe("backdrop preview", () => {
  test("BD-01: dimmed scrim opens and click dismisses", async ({ page }) => {
    await openComponentPreview(page, "backdrop");
    const preview = page.getByTestId("backdrop-preview");
    await preview.getByRole("button", { name: "Show backdrop" }).click();
    const scrim = preview.locator(".orbital-backdrop");
    await expect(scrim).toBeVisible();
    await expectNonTransparentBackground(scrim);
    const box = await scrim.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
    expect(box!.height).toBeGreaterThan(0);
    await scrim.click({ force: true });
    await expect(scrim).toBeHidden();
  });

  test("BD-02: loading overlay shows spinner on scrim", async ({ page }) => {
    await page.goto(previewUrl("/backdrop"));
    await waitForPreviewShell(page);
    const preview = page.getByTestId("backdrop-loading");
    await preview.scrollIntoViewIfNeeded();
    await expect(preview).toBeVisible();
    const scrim = preview.locator(".orbital-backdrop");
    await expect(scrim).toBeVisible();
    await expectNonTransparentBackground(scrim);
    const spinner = preview.getByTestId("backdrop-spinner");
    await expect(spinner).toBeVisible();
    const spinnerBox = await spinner.boundingBox();
    expect(spinnerBox).not.toBeNull();
    expect(spinnerBox!.width).toBeGreaterThan(0);
    expect(spinnerBox!.height).toBeGreaterThan(0);
  });

  test("BD-03: framed scrim uses custom class and covers frame", async ({ page }) => {
    await openComponentPreview(page, "backdrop", "backdrop-framed");
    const frame = page.getByTestId("backdrop-framed");
    await frame.scrollIntoViewIfNeeded();
    const scrim = frame.locator(".orbital-backdrop.backdrop-framed-scrim");
    await expect(scrim).toBeVisible();
    await expectNonTransparentBackground(scrim);
    const frameBox = await frame.boundingBox();
    const scrimBox = await scrim.boundingBox();
    expect(frameBox).not.toBeNull();
    expect(scrimBox).not.toBeNull();
    expect(scrimBox!.width).toBeGreaterThanOrEqual(frameBox!.width * 0.9);
    expect(scrimBox!.height).toBeGreaterThanOrEqual(frameBox!.height * 0.9);
  });

  test("BD-04: spotlight cutout highlights target element", async ({ page }) => {
    await openComponentPreview(page, "backdrop", "backdrop-preview");
    const section = page.getByTestId("backdrop-spotlight");
    await section.scrollIntoViewIfNeeded();
    await section.getByRole("button", { name: "Highlight" }).click();
    await expect(page.getByTestId("backdrop-spotlight-target")).toBeVisible();
    await expect(page.locator(".orbital-backdrop-spotlight").first()).toBeVisible();
  });
});
