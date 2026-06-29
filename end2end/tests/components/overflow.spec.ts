import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { clickMenuTriggerInPreview } from "../lib/preview/overlays";
test.describe("overflow preview", () => {
  test("O-01 horizontal overflow", async ({ page }) => {
    await openComponentPreview(page, "overflow");
    await expect(page.getByTestId("overflow-preview").locator(".orbital-overflow")).toBeVisible();
    await expect(page.getByTestId("overflow-preview").locator("button")).toHaveCount(5);
  });

  test("O-02 overflow menu affordance", async ({ page }) => {
    await openComponentPreview(page, "overflow");
    await expectPreviewVariants(page, ["overflow-menu"]);
    await expect(
      page.getByTestId("overflow-menu").getByRole("button", { name: "..." }),
    ).toBeVisible();
    await clickMenuTriggerInPreview(page, "overflow-menu", "...");
    await expect(page.getByRole("menuitem", { name: "Export" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Share" }).first()).toBeVisible();
  });

  test("O-03 horizontal overflow menu items", async ({ page }) => {
    await openComponentPreview(page, "overflow");
    const preview = page.getByTestId("overflow-preview");
    await expect(preview.getByRole("button", { name: "..." })).toBeVisible();
    await clickMenuTriggerInPreview(page, "overflow-preview", "...");
    await expect(page.getByRole("menuitem", { name: "Copy" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Paste" }).first()).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Delete" }).first()).toBeVisible();
  });
});
