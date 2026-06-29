import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyPseudoStyle } from "../lib/assertions/style";
test.describe("divider primitive preview", () => {
  test.beforeEach(async ({ page }) => {
    await openComponentPreview(page, "divider");
  });

  test("DV-01 horizontal separator", async ({ page }) => {
    const separator = page.getByTestId("divider-preview").locator("[role=separator]");
    await expect(separator).toHaveAttribute("aria-orientation", "horizontal");
    const height = await separator.evaluate((el) => el.getBoundingClientRect().height);
    expect(height).toBeGreaterThan(0);
  });

  test("DV-02 vertical in flex row", async ({ page }) => {
    await expectPreviewVariants(page, ["divider-vertical"]);
    const wrapper = page.getByTestId("divider-vertical");
    const separator = wrapper.locator("[role=separator]");
    await expect(separator).toHaveAttribute("aria-orientation", "vertical");
    await expect(wrapper.getByText("Left")).toBeVisible();
    await expect(wrapper.getByText("Right")).toBeVisible();
  });

  test("DV-03 theme stroke token", async ({ page }) => {
    await expectPreviewVariants(page, ["divider-theme"]);
    const separator = page.getByTestId("divider-theme").locator("[role=separator]").first();
    await expectNonEmptyPseudoStyle(separator, "::before", "border-top-color");
  });

  test("DV-04 labeled divider", async ({ page }) => {
    await expectPreviewVariants(page, ["divider-labeled"]);
    await expect(page.getByTestId("divider-labeled")).toContainText("OR");
  });

  test("DV-05 toolbar cluster", async ({ page }) => {
    await expectPreviewVariants(page, ["divider-toolbar"]);
    await expect(page.getByText("Save")).toBeVisible();
    await expect(page.getByText("Cancel")).toBeVisible();
    await expect(page.getByTestId("divider-toolbar").locator("[role=separator]")).toBeVisible();
  });
});
