import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectComputedStyle, expectDistinctStyle, expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("image primitive preview", () => {
  test("IM-01 default framed image", async ({ page }) => {
    await openComponentPreview(page, "image");
    const img = page.getByTestId("image-preview").locator("img.orbital-image");
    await expect(img).toHaveAttribute("src", /picsum\.photos/);
    await expect(img).toHaveAttribute("alt", "Sample landscape");
    const box = await img.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
    expect(box!.height).toBeGreaterThan(0);
  });

  test("IM-02 rounded border radius", async ({ page }) => {
    await openComponentPreview(page, "image");
    await expectPreviewVariants(page, ["image-rounded"]);
    await expectComputedStyle(page, "image-rounded", {
      "border-radius": /.+px/,
    }, { childSelector: "img.orbital-image--rounded" });
  });

  test("IM-03 shadow elevation", async ({ page }) => {
    await openComponentPreview(page, "image");
    await expectPreviewVariants(page, ["image-shadow"]);
    const img = page.getByTestId("image-shadow").locator("img.orbital-image--shadow");
    await expect(img).toBeVisible();
    const shadow = await img.evaluate((el) => getComputedStyle(el).boxShadow.trim());
    expect(shadow.length).toBeGreaterThan(0);
    expect(shadow.toLowerCase()).not.toBe("none");
  });

  test("IM-04 fit cover and contain", async ({ page }) => {
    await openComponentPreview(page, "image");
    await expectPreviewVariants(page, ["image-fit"]);
    const cover = page.getByTestId("image-fit-cover").locator("img.orbital-image");
    const contain = page.getByTestId("image-fit-contain").locator("img.orbital-image");
    await expect(cover).toHaveCSS("object-fit", "cover");
    await expect(contain).toHaveCSS("object-fit", "contain");
    await expectDistinctStyle(cover, contain, "object-fit");
  });

  test("IM-05 block width", async ({ page }) => {
    await openComponentPreview(page, "image");
    await expectPreviewVariants(page, ["image-block"]);
    const wrapper = page.getByTestId("image-block");
    const img = wrapper.locator("img.orbital-image--block");
    await expect(img).toHaveClass(/orbital-image--block/);
    const wrapperBox = await wrapper.boundingBox();
    const imgBox = await img.boundingBox();
    expect(wrapperBox).not.toBeNull();
    expect(imgBox).not.toBeNull();
    expect(Math.abs(imgBox!.width - wrapperBox!.width)).toBeLessThan(2);
  });

  test("IM-06 theme border token", async ({ page }) => {
    await openComponentPreview(page, "image");
    await expectPreviewVariants(page, ["image-theme"]);
    await expectNonEmptyResolvedStyle(page, "image-theme", "border-color", {
      childSelector: "img.orbital-image",
    });
  });

  test("IM-07 bounded frame dimensions", async ({ page }) => {
    await openComponentPreview(page, "image");
    await expectPreviewVariants(page, ["image-bounded"]);
    const img = page.getByTestId("image-bounded").locator("img.orbital-image");
    await expect(img).toHaveAttribute("width", "160px");
    await expect(img).toHaveAttribute("height", "100px");
    await expect(img).toHaveCSS("object-fit", "cover");
  });
});
