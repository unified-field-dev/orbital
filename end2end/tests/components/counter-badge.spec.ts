import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("counter-badge preview", () => {
  test("CB-01 default count", async ({ page }) => {
    await openComponentPreview(page, "counter-badge");
    await expect(
      page.getByTestId("counter-badge-preview").locator(".orbital-badge"),
    ).toHaveText("3");
  });

  test("CB-02 on avatar", async ({ page }) => {
    await openComponentPreview(page, "counter-badge");
    await expectPreviewVariants(page, ["counter-badge-avatar"]);
    await expect(
      page.getByTestId("counter-badge-avatar").locator(".orbital-avatar"),
    ).toBeVisible();
    await expect(
      page.getByTestId("counter-badge-avatar").locator(".orbital-badge"),
    ).toHaveText("5");
  });

  test("CB-03 overflow cap", async ({ page }) => {
    await openComponentPreview(page, "counter-badge");
    await expectPreviewVariants(page, ["counter-badge-overflow"]);
    await expect(
      page.getByTestId("counter-badge-overflow").locator(".orbital-badge"),
    ).toHaveText("99+");
  });
});
