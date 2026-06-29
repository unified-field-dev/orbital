import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("avatar-group preview", () => {
  test("AG-01 stacked group", async ({ page }) => {
    await openComponentPreview(page, "avatar-group");
    await expectPreviewVariants(page, ["avatar-group-stack"]);
    const avatars = page.getByTestId("avatar-group-stack").locator(".orbital-avatar");
    await expect(avatars).toHaveCount(3);
    await expect(page.getByTestId("avatar-group-stack").locator(".orbital-avatar-group--stack")).toBeVisible();
  });

  test("AG-02 spread layout", async ({ page }) => {
    await openComponentPreview(page, "avatar-group");
    await expectPreviewVariants(page, ["avatar-group-spread"]);
    await expect(
      page.getByTestId("avatar-group-spread").locator(".orbital-avatar-group--spread"),
    ).toBeVisible();
  });

  test("AG-03 overflow chip", async ({ page }) => {
    await openComponentPreview(page, "avatar-group");
    await expectPreviewVariants(page, ["avatar-group-overflow"]);
    await expect(
      page.getByTestId("avatar-group-overflow").locator(".orbital-avatar-group__overflow"),
    ).toHaveText("+2");
  });
});
