import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("interaction-tag primitive preview", () => {
  test("IT-01: default interaction tag renders primary button", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    const root = page.getByTestId("interaction-tag-preview").locator(".orbital-interaction-tag");
    await expect(root).toBeVisible();
    await expect(root.locator("button.orbital-tag--primary-action")).toHaveText("Filter");
  });

  test("IT-02: size matrix applies height modifiers", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-sizes"]);
    const medium = page.getByTestId("interaction-tag-size-medium").locator(".orbital-interaction-tag");
    const small = page.getByTestId("interaction-tag-size-small").locator(".orbital-interaction-tag");
    const extraSmall = page.getByTestId("interaction-tag-size-extra-small").locator(".orbital-interaction-tag");
    await expect(medium).toHaveClass(/orbital-interaction-tag--medium/);
    await expect(small).toHaveClass(/orbital-interaction-tag--small/);
    await expect(extraSmall).toHaveClass(/orbital-interaction-tag--extra-small/);
    const mediumH = await medium.evaluate((el) => el.getBoundingClientRect().height);
    const smallH = await small.evaluate((el) => el.getBoundingClientRect().height);
    const xsH = await extraSmall.evaluate((el) => el.getBoundingClientRect().height);
    expect(mediumH).toBeGreaterThan(smallH);
    expect(smallH).toBeGreaterThan(xsH);
  });

  test("IT-03: primary button is clickable", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-click"]);
    const button = page.getByTestId("interaction-tag-click").getByRole("button", { name: "Click me" });
    await expect(button).toBeVisible();
    await button.click();
    await expect(button).toBeVisible();
  });

  test("IT-04: inherits small size from tag group", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-in-group"]);
    await expect(
      page.getByTestId("interaction-tag-in-group").locator(".orbital-interaction-tag"),
    ).toHaveClass(/orbital-interaction-tag--small/);
  });

  test("IT-05: secondary action button is visible", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-secondary"]);
    const root = page.getByTestId("interaction-tag-secondary").locator(".orbital-interaction-tag");
    await expect(root.locator(".orbital-tag--with-secondary")).toBeVisible();
    await expect(root.locator(".orbital-tag--secondary-action")).toBeVisible();
  });

  test("IT-06: secondary click removes tag", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-secondary-dismiss"]);
    const root = page.getByTestId("interaction-tag-secondary-dismiss");
    await expect(root.locator(".orbital-interaction-tag")).toHaveCount(2);
    await root.locator(".orbital-tag--secondary-action").first().click();
    await expect(root.locator(".orbital-interaction-tag")).toHaveCount(1);
  });

  test("IT-07: appearance matrix applies appearance modifiers", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-appearances"]);
    const root = page.getByTestId("interaction-tag-appearances");
    await expect(root.locator("button.orbital-tag--filled.orbital-tag--primary-action")).toHaveCount(1);
    await expect(root.locator("button.orbital-tag--outline.orbital-tag--primary-action")).toHaveCount(1);
    await expect(root.locator("button.orbital-tag--brand.orbital-tag--primary-action")).toHaveCount(1);
  });

  test("IT-08: icon and secondary action render together", async ({ page }) => {
    await openComponentPreview(page, "interaction-tag");
    await expectPreviewVariants(page, ["interaction-tag-icon-secondary"]);
    const root = page.getByTestId("interaction-tag-icon-secondary").locator(".orbital-interaction-tag");
    await expect(root.locator(".orbital-tag--with-media")).toBeVisible();
    await expect(root.locator(".orbital-tag__media")).toBeVisible();
    await expect(root.locator(".orbital-tag--secondary-action")).toBeVisible();
  });
});
