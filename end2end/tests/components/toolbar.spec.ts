import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("toolbar preview", () => {
  test("T-01 default toolbar", async ({ page }) => {
    await openComponentPreview(page, "toolbar");
    const toolbar = page.getByTestId("toolbar-preview").locator(".orbital-toolbar");
    await expect(toolbar).toBeVisible();
    await expect(toolbar).toHaveAttribute("role", "toolbar");
    await expect(toolbar).toHaveAttribute("aria-orientation", "horizontal");
  });

  test("T-02 vertical layout", async ({ page }) => {
    await openComponentPreview(page, "toolbar");
    await expectPreviewVariants(page, ["toolbar-vertical"]);
    await expect(page.getByTestId("toolbar-vertical").locator(".orbital-toolbar")).toHaveAttribute(
      "aria-orientation",
      "vertical",
    );
  });

  test("T-03 with overflow", async ({ page }) => {
    await openComponentPreview(page, "toolbar");
    await expectPreviewVariants(page, ["toolbar-overflow"]);
    await expect(
      page.getByTestId("toolbar-overflow").locator(".orbital-overflow"),
    ).toBeVisible();
  });
});
