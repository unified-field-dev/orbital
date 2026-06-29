import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("floating-button component preview", () => {
  test("FB-01 default is visible and accessible", async ({ page }) => {
    await openComponentPreview(page, "floating-button");
    const button = page.getByTestId("floating-button-preview").getByTestId("floating-button");
    await expect(button).toBeVisible({ timeout: 10_000 });
    await expect(button).toHaveAccessibleName("Add");
    const box = await button.boundingBox();
    expect(box?.width ?? 0).toBeGreaterThan(0);
    expect(box?.height ?? 0).toBeGreaterThan(0);
  });

  test("FB-02 documented variants render", async ({ page }) => {
    await openComponentPreview(page, "floating-button");
    await expectPreviewVariants(page, [
      "floating-button-extended",
      "floating-button-size",
      "floating-button-secondary",
    ]);
    await expect(
      page.getByTestId("floating-button-extended").getByRole("button", { name: "Navigate" }),
    ).toBeVisible();
    await expect(
      page.getByTestId("floating-button-secondary").getByRole("button", { name: "Edit" }),
    ).toBeVisible();
  });
});
