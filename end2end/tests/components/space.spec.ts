import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectComputedStyle } from "../lib/assertions/style";
test.describe("space primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "space");
  });

  test("SP-01 default space-between row", async ({ page }) => {
    const space = page.getByTestId("space-preview").locator(".orbital-flex");
    await expect(space).toHaveCSS("flex-direction", "row");
    await expect(space).toHaveCSS("justify-content", "space-between");
    await expect(page.getByTestId("space-item-1")).toBeVisible();
    await expect(page.getByTestId("space-item-2")).toBeVisible();
  });

  test("SP-02 vertical distribution", async ({ page }) => {
    await expectPreviewVariants(page, ["space-vertical"]);
    const space = page.getByTestId("space-vertical").locator(".orbital-flex");
    await expect(space).toHaveCSS("flex-direction", "column");
    await expect(space).toHaveCSS("justify-content", "space-between");
  });

  test("SP-03 even gap override", async ({ page }) => {
    await expectPreviewVariants(page, ["space-even-gap"]);
    const space = page.getByTestId("space-even-gap").locator(".orbital-flex");
    const justify = await space.evaluate((el) => getComputedStyle(el).justifyContent);
    expect(justify).not.toEqual("space-between");
    await expect(page.getByTestId("space-gap-a")).toBeVisible();
  });

  test("SP-04 align center", async ({ page }) => {
    await expectPreviewVariants(page, ["space-align"]);
    await expectComputedStyle(page, "space-align", { "align-items": "center" }, {
      childSelector: ".orbital-flex",
    });
  });

  test("SP-05 justify center override", async ({ page }) => {
    await expectPreviewVariants(page, ["space-justify"]);
    await expectComputedStyle(page, "space-justify", { "justify-content": "center" }, {
      childSelector: ".orbital-flex",
    });
  });

  test("SP-06 toolbar pattern", async ({ page }) => {
    await expectPreviewVariants(page, ["space-toolbar"]);
    const wrapper = page.getByTestId("space-toolbar");
    await expect(wrapper.getByRole("button", { name: "Upload" })).toBeVisible();
    await expect(wrapper.getByText("Documents", { exact: true })).toBeVisible();
    await expect(wrapper.getByText("3 new", { exact: true })).toBeVisible();
  });
});
