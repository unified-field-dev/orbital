import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { expectOverlayInViewport } from "../lib/preview/overlays";
import {
  clickToastPreviewButton,
  expectToastAtCorner,
  expectToastInsideContainer,
  expectToastNearViewportCorner,
  expectToastStackPosition,
  expectVisibleToastCount,
  firstDispatchedToast,
} from "../lib/preview/toast";

test.describe("toast primitive preview", () => {
  test.use({ viewport: { width: 1280, height: 720 } });

  test("TO-01: static toast composition renders title and body", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-static");
    await expect(page.getByTestId("toast-static").locator(".orbital-toast-title")).toBeVisible();
    await expect(page.getByTestId("toast-static").locator(".orbital-toast-body")).toContainText(
      "Your changes were saved.",
    );
  });

  test("TO-02: provider dispatch auto-dismisses toast", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-provider");
    await clickToastPreviewButton(page, "toast-provider", "Show toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await expect(toast).toBeHidden({ timeout: 3_000 });
  });

  test("TO-03: success intent toast uses success modifier class", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-provider");
    await clickToastPreviewButton(page, "toast-provider", "Show success");
    const toast = page.locator(".orbital-toast-stack .orbital-toast--success").first();
    await expect(toast).toBeVisible();
    const successBg = await toast.evaluate((el) => getComputedStyle(el).backgroundColor);
    await clickToastPreviewButton(page, "toast-provider", "Show toast");
    const infoToast = page.locator(".orbital-toast-stack .orbital-toast--info").first();
    await expect(infoToast).toBeVisible();
    const infoBg = await infoToast.evaluate((el) => getComputedStyle(el).backgroundColor);
    expect(successBg).not.toBe(infoBg);
  });

  test("TO-04: dispatched error toast footer dismisses on retry", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-error-action");
    await clickToastPreviewButton(page, "toast-error-action", "Show error toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    const action = page.locator(".orbital-toast-stack .orbital-toast-footer").getByRole("button", {
      name: "Retry",
    });
    await expect(action).toBeVisible();
    await action.click();
    await expect(toast).toBeHidden({ timeout: 2_000 });
  });

  test("TO-04b: retry footer action runs callback before dismiss", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-error-action");
    await expect(page.getByTestId("toast-retry-count")).toHaveText("Retries: 0");
    await clickToastPreviewButton(page, "toast-error-action", "Show error toast");
    await page
      .locator(".orbital-toast-stack .orbital-toast-footer")
      .getByRole("button", { name: "Retry" })
      .click();
    await expect(page.getByTestId("toast-retry-count")).toHaveText("Retries: 1");
  });

  test("TO-28: warning and error intents use distinct semantic backgrounds", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-provider");
    await clickToastPreviewButton(page, "toast-provider", "Show warning");
    const warningToast = page.locator(".orbital-toast-stack .orbital-toast--warning").first();
    await expect(warningToast).toBeVisible();
    const warningBg = await warningToast.evaluate((el) => getComputedStyle(el).backgroundColor);

    await clickToastPreviewButton(page, "toast-provider", "Show error");
    const errorToast = page.locator(".orbital-toast-stack .orbital-toast--error").first();
    await expect(errorToast).toBeVisible();
    const errorBg = await errorToast.evaluate((el) => getComputedStyle(el).backgroundColor);

    await clickToastPreviewButton(page, "toast-provider", "Show toast");
    const infoToast = page.locator(".orbital-toast-stack .orbital-toast--info").first();
    await expect(infoToast).toBeVisible();
    const infoBg = await infoToast.evaluate((el) => getComputedStyle(el).backgroundColor);

    expect(warningBg).not.toBe(infoBg);
    expect(errorBg).not.toBe(infoBg);
    expect(warningBg).not.toBe(errorBg);
  });

  test("TO-05: bottom-end position applies stack placement class and data attribute", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-position");
    await clickToastPreviewButton(page, "toast-position", "Show toast");
    await expectToastStackPosition(page, "bottom-end");
  });

  test("TO-06: custom timeout toast disappears", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-timeout");
    await clickToastPreviewButton(page, "toast-timeout", "Show timed toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await expect(toast).toBeHidden({ timeout: 2_000 });
  });

  test("TO-07: persistent toast stays visible", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-persistent");
    await clickToastPreviewButton(page, "toast-persistent", "Show persistent toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await page.waitForTimeout(2_000);
    await expect(toast).toBeVisible();
  });

  test("TO-08: dismiss toast removes toast immediately", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-dismiss");
    await clickToastPreviewButton(page, "toast-dismiss", "Make toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await clickToastPreviewButton(page, "toast-dismiss", "Dismiss toast");
    await expect(toast).toBeHidden({ timeout: 2_000 });
  });

  test("TO-09: dismiss all clears every toast", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-dismiss-all");
    const wrapper = page.getByTestId("toast-dismiss-all");
    for (let i = 0; i < 3; i += 1) {
      await wrapper.getByRole("button", { name: "Make toast" }).click();
    }
    await expectVisibleToastCount(page, 3);
    await wrapper.getByRole("button", { name: "Dismiss all" }).click();
    await expectVisibleToastCount(page, 0);
  });

  test("TO-10: dismiss action in footer removes toast", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-dismiss-action");
    await clickToastPreviewButton(page, "toast-dismiss-action", "Make toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await page
      .locator(".orbital-toast-stack .orbital-toast-footer")
      .getByRole("button", { name: "Dismiss" })
      .click();
    await expect(toast).toBeHidden({ timeout: 2_000 });
  });

  test("TO-11: toast auto-dismisses without hover", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-pause-hover");
    await clickToastPreviewButton(page, "toast-pause-hover", "Make toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await expect(toast).toBeHidden({ timeout: 2_000 });
  });

  test("TO-12: hover pauses toast dismissal", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-pause-hover");
    await clickToastPreviewButton(page, "toast-pause-hover", "Make toast");
    const toast = firstDispatchedToast(page);
    await expect(toast).toBeVisible();
    await toast.hover();
    await page.waitForTimeout(1_500);
    await expect(toast).toBeVisible();
    await page.mouse.move(0, 0);
    await expect(toast).toBeHidden({ timeout: 2_000 });
  });

  test("TO-13: toast limit caps visible toasts", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-limit");
    const wrapper = page.getByTestId("toast-limit");
    for (let i = 0; i < 5; i += 1) {
      await wrapper.getByRole("button", { name: "Make toast" }).click();
    }
    await expectVisibleToastCount(page, 3);
  });

  test("TO-14: dismissing visible toast dequeues next toast", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-limit");
    const wrapper = page.getByTestId("toast-limit");
    for (let i = 0; i < 4; i += 1) {
      await wrapper.getByRole("button", { name: "Make toast" }).click();
    }
    await expectVisibleToastCount(page, 3);
    await page
      .locator(".orbital-toast-stack .orbital-toast-footer")
      .getByRole("button", { name: "Dismiss" })
      .first()
      .click();
    await expectVisibleToastCount(page, 3);
  });

  test("TO-15: top-start position is near viewport corner", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await expectToastAtCorner(page, "toast-positions", "Top start", "top-start");
  });

  test("TO-16: top-end position is near viewport corner", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await expectToastAtCorner(page, "toast-positions", "Top end", "top-end");
  });

  test("TO-17: top position is centered near top edge", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await expectToastAtCorner(page, "toast-positions", "Top", "top");
  });

  test("TO-18: bottom-start position is near viewport corner", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await expectToastAtCorner(page, "toast-positions", "Bottom start", "bottom-start");
  });

  test("TO-19: bottom-end position is near viewport corner", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await expectToastAtCorner(page, "toast-positions", "Bottom end", "bottom-end");
  });

  test("TO-20: bottom position is centered near bottom edge", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await expectToastAtCorner(page, "toast-positions", "Bottom", "bottom");
  });

  test("TO-21: per-toast position override wins over provider default", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-positions");
    await clickToastPreviewButton(page, "toast-positions", "Override top start", true);
    await expectToastStackPosition(page, "top-start");
    await expectToastNearViewportCorner(page, "top-start");
  });

  test("TO-22: offset css variables match provider values", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-offset");
    await clickToastPreviewButton(page, "toast-offset", "Make toast");
    const stack = page.locator(".orbital-toast-stack").first();
    await expect(stack).toBeVisible();
    const vars = await stack.evaluate((el) => {
      const computed = getComputedStyle(el);
      return {
        x: computed.getPropertyValue("--orbital-toast-offset-x").trim(),
        y: computed.getPropertyValue("--orbital-toast-offset-y").trim(),
      };
    });
    expect(vars.x).toBe("40px");
    expect(vars.y).toBe("60px");
  });

  test("TO-23: offset shifts stack inset from viewport edge", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-offset");
    await clickToastPreviewButton(page, "toast-offset", "Make toast");
    const stack = page.locator(".orbital-toast-stack").first();
    await expect(stack).toBeVisible();
    const box = await stack.boundingBox();
    expect(box).not.toBeNull();
    const viewport = page.viewportSize();
    expect(viewport).not.toBeNull();
    expect(Math.abs(viewport!.width - (box!.x + box!.width) - 40)).toBeLessThanOrEqual(24);
    expect(Math.abs(viewport!.height - (box!.y + box!.height) - 60)).toBeLessThanOrEqual(24);
  });

  test("TO-24: inline toaster renders inside container", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-inline");
    await clickToastPreviewButton(page, "toast-inline", "Make toast");
    await expectToastInsideContainer(page, "toast-inline-container");
  });

  test("TO-25: portaled toaster renders in viewport", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-provider");
    await clickToastPreviewButton(page, "toast-provider", "Show toast");
    await expectOverlayInViewport(page, ".orbital-toast-stack");
  });

  test("TO-26: composed dispatch renders title body and footer slots", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-default-composition");
    await clickToastPreviewButton(page, "toast-default-composition", "Make toast");
    const toast = firstDispatchedToast(page);
    await expect(toast.locator(".orbital-toast-title")).toContainText("Email sent");
    await expect(toast.locator(".orbital-toast-body")).toContainText("Your message was delivered.");
    await expect(toast.locator(".orbital-toast-footer")).toContainText("Undo");
  });

  test("TO-27: provider default intent applies to plain show()", async ({ page }) => {
    await openComponentPreview(page, "toast", "toast-provider");
    await clickToastPreviewButton(page, "toast-provider", "Show toast");
    await expect(page.locator(".orbital-toast-stack .orbital-toast--info").first()).toBeVisible();
  });
});
