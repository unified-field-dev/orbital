import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("tag-group primitive preview", () => {
  test("TGR-01: default group renders multiple tags", async ({ page }) => {
    await openComponentPreview(page, "tag-group");
    const group = page.getByTestId("tag-group-preview").locator(".orbital-tag-group");
    await expect(group).toBeVisible();
    await expect(group.locator(".orbital-tag")).toHaveCount(2);
    await expect(group.locator(".orbital-tag").nth(0)).toHaveText("Alpha");
    await expect(group.locator(".orbital-tag").nth(1)).toHaveText("Beta");
  });

  test("TGR-02: dismissible group shows dismiss buttons and removes tag on click", async ({ page }) => {
    await openComponentPreview(page, "tag-group");
    await expectPreviewVariants(page, ["tag-group-dismissible"]);
    const group = page.getByTestId("tag-group-dismissible").locator(".orbital-tag-group");
    const tags = group.locator(".orbital-tag");
    await expect(tags).toHaveCount(3);
    await expect(tags.first()).toHaveClass(/orbital-tag--dismissible/);
    await expect(tags.first().locator(".orbital-tag__dismiss")).toBeVisible();
    await group.locator(".orbital-tag__dismiss").first().click();
    await expect(group.locator(".orbital-tag")).toHaveCount(2);
  });

  test("TGR-03: small size group applies size modifier", async ({ page }) => {
    await openComponentPreview(page, "tag-group");
    await expectPreviewVariants(page, ["tag-group-small"]);
    const tag = page.getByTestId("tag-group-small").locator(".orbital-tag").first();
    await expect(tag).toHaveClass(/orbital-tag--small/);
  });

  test("TGR-04: appearance group applies appearance modifiers", async ({ page }) => {
    await openComponentPreview(page, "tag-group");
    await expectPreviewVariants(page, ["tag-group-appearances"]);
    const root = page.getByTestId("tag-group-appearances");
    await expect(root.locator(".orbital-tag--filled")).toHaveCount(1);
    await expect(root.locator(".orbital-tag--outline")).toHaveCount(1);
    await expect(root.locator(".orbital-tag--brand")).toHaveCount(1);
  });

  test("TGR-05: many tags layout keeps inline flex group", async ({ page }) => {
    await openComponentPreview(page, "tag-group");
    await expectPreviewVariants(page, ["tag-group-many"]);
    const group = page.getByTestId("tag-group-many").locator(".orbital-tag-group");
    await expect(group).toHaveCSS("display", "inline-flex");
    await expect(group.locator(".orbital-tag")).toHaveCount(5);
  });
});
