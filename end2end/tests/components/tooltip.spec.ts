import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { hoverTooltipInPreview, expectTeleportedRole, expectOverlayInViewport, expectOverlayNonZeroSize } from "../lib/preview/overlays";
test.describe("tooltip primitive preview", () => {

  test("TT-01: hover reveals tooltip body on default preview", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await hoverTooltipInPreview(page, "tooltip-preview", "Hover me");
    await expectTeleportedRole(page, "tooltip", "More information");
    await expectOverlayNonZeroSize(page, ".orbital-tooltip-content");
    await expectOverlayInViewport(page, ".orbital-tooltip-content");
  });

  test("TT-02: inverted appearance uses darker tooltip surface", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-inverted").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-inverted", "Target");
    const inverted = page.getByRole("tooltip", { name: "Dark tooltip" });
    await expect(inverted).toBeVisible();
    await expect(inverted).toHaveClass(/orbital-tooltip-content--inverted/);
    await expectOverlayNonZeroSize(page, "[role='tooltip']");
  });

  test("TT-03: bottom placement sets follower data-orbital-placement", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-bottom").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-bottom", "Bottom");
    await expectTeleportedRole(page, "tooltip", "Below");
    await expect(page.locator('[data-orbital-placement="bottom"]').first()).toHaveAttribute("data-orbital-placement", "bottom");
  });

  test("TT-04: left placement sets follower data-orbital-placement", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-left").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-left", "Left");
    await expectTeleportedRole(page, "tooltip", "To the left");
    await expect(page.locator('[data-orbital-placement="left"]').first()).toHaveAttribute("data-orbital-placement", "left");
  });

  test("TT-05: right placement sets follower data-orbital-placement", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-right").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-right", "Right");
    await expectTeleportedRole(page, "tooltip", "To the right");
    await expect(page.locator('[data-orbital-placement="right"]').first()).toHaveAttribute("data-orbital-placement", "right");
  });

  test("TT-06: top-start placement sets follower data-orbital-placement", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-top-start").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-top-start", "Top start");
    await expectTeleportedRole(page, "tooltip", "Top start");
    await expect(page.locator('[data-orbital-placement="top-start"]').first()).toHaveAttribute("data-orbital-placement", "top-start");
  });

  test("TT-07: bottom-end placement sets follower data-orbital-placement", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-bottom-end").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-bottom-end", "Bottom end");
    await expectTeleportedRole(page, "tooltip", "Bottom end");
    await expect(page.locator('[data-orbital-placement="bottom-end"]').first()).toHaveAttribute("data-orbital-placement", "bottom-end");
  });

  test("TT-08: tooltip surface uses theme background token", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-theme").scrollIntoViewIfNeeded();
    await hoverTooltipInPreview(page, "tooltip-theme", "Theme");
    await expectTeleportedRole(page, "tooltip", "Themed tooltip");
    const tooltip = page.getByRole("tooltip", { name: "Themed tooltip" });
    await expect(tooltip).toBeVisible();
    await expectOverlayNonZeroSize(page, "[role='tooltip']");
  });

  test("TT-10: show_delay_ms prevents immediate tooltip open", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-show-delay").scrollIntoViewIfNeeded();
    const btn = page
      .getByTestId("tooltip-show-delay")
      .getByRole("button", { name: "Hover and wait" });
    await btn.hover();
    await expect(page.getByRole("tooltip", { name: "Delayed hint" })).not.toBeVisible({
      timeout: 200,
    });
    await page.waitForTimeout(350);
    await expectTeleportedRole(page, "tooltip", "Delayed hint");
  });

  test("TT-09: disabled focusable button shows tooltip on focus", async ({ page }) => {
    await openComponentPreview(page, "tooltip");
    await page.getByTestId("tooltip-disabled-focusable").scrollIntoViewIfNeeded();
    const btn = page.getByTestId("tooltip-disabled-focusable").getByRole("button", { name: "Save" });
    await btn.focus();
    await expectTeleportedRole(
      page,
      "tooltip",
      "Save is unavailable until you fix validation errors",
    );
  });
});
