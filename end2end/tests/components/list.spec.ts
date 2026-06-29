import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("list preview", () => {
  test("L-01 default list", async ({ page }) => {
    await openComponentPreview(page, "list");
    await expect(page.getByTestId("list-preview").locator(".orbital-list__item")).toHaveCount(3);
  });

  test("L-02 nav mode", async ({ page }) => {
    await openComponentPreview(page, "list");
    await expectPreviewVariants(page, ["list-nav"]);
    await expect(page.getByTestId("list-nav").locator(".orbital-list")).toHaveClass(/orbital-list--nav-nav/);
  });

  test("L-03 single select", async ({ page }) => {
    await openComponentPreview(page, "list");
    await expectPreviewVariants(page, ["list-single-select"]);
    await expect(
      page.getByTestId("list-single-select").locator(".orbital-list__item--selected"),
    ).toHaveCount(1);
  });
});
