import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectTeleportedRole } from "../lib/preview/overlays";
test.describe("drawer primitive preview", () => {
  test("DR-01: inline drawer visible by default", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expect(page.getByTestId("drawer-preview")).toBeVisible();
    await expect(page.getByTestId("drawer-preview")).toContainText("Drawer body");
  });

  test("DR-02: right overlay opens from trigger", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expectPreviewVariants(page, ["drawer-right"]);
    await page.getByTestId("drawer-right").getByRole("button", { name: "Open right" }).click();
    await expectTeleportedRole(page, "dialog");
    await expect(page.getByTestId("drawer-right-content")).toBeVisible();
  });

  test("DR-03: left overlay opens from trigger", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expectPreviewVariants(page, ["drawer-left"]);
    await page.getByTestId("drawer-left").getByRole("button", { name: "Open left" }).click();
    await expect(page.getByTestId("drawer-left-content")).toBeVisible();
  });

  test("DR-04: top overlay opens from trigger", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expectPreviewVariants(page, ["drawer-top"]);
    await page.getByTestId("drawer-top").getByRole("button", { name: "Open top" }).click();
    await expect(page.getByTestId("drawer-top-content")).toBeVisible();
  });

  test("DR-05: bottom overlay opens from trigger", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expectPreviewVariants(page, ["drawer-bottom"]);
    await page.getByTestId("drawer-bottom").getByRole("button", { name: "Open bottom" }).click();
    await expect(page.getByTestId("drawer-bottom-content")).toBeVisible();
  });

  test("DR-06: escape closes esc-enabled drawer", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expectPreviewVariants(page, ["drawer-esc"]);
    await page.getByTestId("drawer-esc").getByRole("button", { name: "Open drawer" }).click();
    const dialog = page.getByRole("dialog").filter({ hasText: "Press Escape to close" });
    await expect(dialog).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
  });

  test("DR-07: large size drawer opens from trigger", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await expectPreviewVariants(page, ["drawer-size"]);
    await page.getByTestId("drawer-size").getByRole("button", { name: "Open large" }).click();
    const dialog = page.getByRole("dialog").filter({ hasText: "Large drawer panel" });
    await expect(dialog).toBeVisible();
    const box = await dialog.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(500);
  });

  test("DR-08: overlay drawer scrim uses shared orbital-backdrop", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    await page.getByTestId("drawer-right").getByRole("button", { name: "Open right" }).click();
    const scrim = page.locator(".orbital-overlay-drawer-container .orbital-backdrop").first();
    await expect(scrim).toBeVisible();
    const bg = await scrim.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(bg).not.toBe("rgba(0, 0, 0, 0)");
    expect(bg).not.toBe("transparent");
  });
});
