import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectTeleportedRole } from "../lib/preview/overlays";
test.describe("motion migration smoke", () => {
  test("MM-01: dialog opens with OrbitalPresence scrim", async ({ page }) => {
    await openComponentPreview(page, "dialog", "dialog-basic");
    await page.getByTestId("dialog-basic").getByRole("button", { name: "Open dialog" }).click();
    await expectTeleportedRole(page, "dialog");
    await expect(page.getByTestId("dialog-preview")).toContainText("Save your changes?");
  });

  test("MM-02: drawer overlay opens", async ({ page }) => {
    await openComponentPreview(page, "drawer");
    const preview = page.getByTestId("drawer-preview");
    await preview.getByRole("button", { name: "Open drawer" }).click();
    await expect(preview.locator(".orbital-overlay-drawer")).toBeVisible();
  });

  test("MM-03: accordion panel expands", async ({ page }) => {
    await openComponentPreview(page, "accordion");
    await expectPreviewVariants(page, ["accordion-keyboard"]);
    const preview = page.getByTestId("accordion-keyboard");
    const panel = preview.locator(".orbital-accordion-panel").first();
    await expect(panel).toBeHidden();
    await preview.getByRole("button").first().click();
    await expect(panel).toBeVisible();
  });
});
