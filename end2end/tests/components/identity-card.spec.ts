import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("identity-card pattern preview", () => {
  test("IC-01 default identity card", async ({ page }) => {
    await openComponentPreview(page, "components/patterns/identity-card", "identity-card-preview");
    const card = page.getByTestId("identity-card-default");
    await expect(card.locator(".orbital-persona")).toBeVisible();
    await expect(card.locator(".orbital-persona__primary-text")).toContainText("Core Observability");
    await expect(card.locator(".orbital-persona__secondary-text")).toContainText("Owns dashboards");
  });

  test("IC-02 with contact row", async ({ page }) => {
    await openComponentPreview(page, "components/patterns/identity-card", "identity-card-preview");
    await expectPreviewVariants(page, ["identity-card-contact"]);
    await expect(
      page.getByTestId("identity-card-contact").locator("a[href^='mailto:']"),
    ).toHaveText("taylor.reid@example.com");
    await expect(
      page.getByTestId("identity-card-contact").locator(".orbital-persona__secondary-text"),
    ).toBeVisible();
  });

  test("IC-03 colorful avatar class", async ({ page }) => {
    await openComponentPreview(page, "components/patterns/identity-card", "identity-card-preview");
    await expectPreviewVariants(page, ["identity-card-colorful"]);
    await expect(
      page.getByTestId("identity-card-colorful").locator(".orbital-avatar"),
    ).not.toHaveClass(/orbital-avatar--color-neutral/);
  });

  test("IC-04 nested in parent card", async ({ page }) => {
    await openComponentPreview(page, "components/patterns/identity-card", "identity-card-preview");
    await expectPreviewVariants(page, ["identity-card-nested"]);
    await expect(page.getByTestId("identity-card-nested").locator(".orbital-card-content").first()).toBeVisible();
    await expect(page.getByTestId("identity-card-nested").locator(".orbital-persona")).toBeVisible();
  });
});
