import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("progress-bar primitive preview", () => {
  test("PB-01: default progress bar is visible with resolved fill", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    const wrapper = page.getByTestId("progress-bar-preview");
    const track = wrapper.locator(".orbital-progress-bar");
    const fill = wrapper.locator(".orbital-progress-bar__bar");
    await expect(track).toBeVisible();
    await expect(track).toHaveAttribute("aria-valuenow", "0.5");
    const trackBox = await track.boundingBox();
    expect(trackBox).not.toBeNull();
    expect(trackBox!.height).toBeGreaterThanOrEqual(4);
    expect(trackBox!.width).toBeGreaterThan(100);
    const fillWidth = await fill.evaluate((el) => el.getBoundingClientRect().width);
    expect(fillWidth).toBeGreaterThan(trackBox!.width * 0.4);
    await expectNonEmptyResolvedStyle(page, "progress-bar-preview", "background-color", {
      childSelector: ".orbital-progress-bar__bar",
    });
  });

  test("PB-02: semantic color modifiers apply visible fills", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    await expectPreviewVariants(page, ["progress-bar-colors"]);
    const wrapper = page.getByTestId("progress-bar-colors");
    await expect(wrapper.locator(".orbital-progress-bar--success")).toBeVisible();
    await expect(wrapper.locator(".orbital-progress-bar--warning")).toBeVisible();
    await expect(wrapper.locator(".orbital-progress-bar--error")).toBeVisible();
    for (const color of ["success", "warning", "error"] as const) {
      const fill = wrapper.locator(`.orbital-progress-bar--${color} .orbital-progress-bar__bar`);
      const fillBox = await fill.boundingBox();
      expect(fillBox).not.toBeNull();
      expect(fillBox!.width).toBeGreaterThan(0);
      expect(fillBox!.height).toBeGreaterThanOrEqual(4);
      const bg = await fill.evaluate((el) => getComputedStyle(el).backgroundColor.trim());
      expect(bg.length).toBeGreaterThan(0);
      expect(bg).not.toBe("rgba(0, 0, 0, 0)");
    }
  });

  test("PB-03: custom max sets aria-valuemax and visible fill", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    await expectPreviewVariants(page, ["progress-bar-max"]);
    const wrapper = page.getByTestId("progress-bar-max");
    const track = wrapper.getByRole("progressbar");
    await expect(track).toHaveAttribute("aria-valuemax", "5");
    await expect(track).toHaveAttribute("aria-valuenow", "3");
    const fill = wrapper.locator(".orbital-progress-bar__bar");
    const trackBox = await track.boundingBox();
    const fillWidth = await fill.evaluate((el) => el.getBoundingClientRect().width);
    expect(trackBox).not.toBeNull();
    expect(fillWidth).toBeGreaterThan(trackBox!.width * 0.5);
  });

  test("PB-04: low and high values clamp within track", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    await expectPreviewVariants(page, ["progress-bar-extremes"]);
    const bars = page.getByTestId("progress-bar-extremes").locator(".orbital-progress-bar__bar");
    const lowWidth = await bars.nth(0).evaluate((el) => el.getBoundingClientRect().width);
    const highWidth = await bars.nth(1).evaluate((el) => el.getBoundingClientRect().width);
    expect(highWidth).toBeGreaterThan(lowWidth);
    const lowHeight = await bars.nth(0).evaluate((el) => el.getBoundingClientRect().height);
    expect(lowHeight).toBeGreaterThanOrEqual(4);
  });

  test("PC-01: progress circle exposes progressbar role", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    await expectPreviewVariants(page, ["progress-circle-preview"]);
    const wrapper = page.getByTestId("progress-circle-preview");
    const circle = wrapper.getByRole("progressbar");
    await expect(circle).toBeVisible();
    await expect(circle).toHaveAttribute("aria-valuenow", "65");
    await expect(wrapper.getByText("65 %")).toBeVisible();

    const centers = await wrapper.evaluate((el) => {
      const svg = el.querySelector("svg");
      const label = el.querySelector(".orbital-progress-circle__content");
      if (!svg || !label) {
        return null;
      }
      const svgBox = svg.getBoundingClientRect();
      const labelBox = label.getBoundingClientRect();
      return {
        svgCenterX: svgBox.x + svgBox.width / 2,
        labelCenterX: labelBox.x + labelBox.width / 2,
      };
    });
    expect(centers).not.toBeNull();
    expect(Math.abs(centers!.svgCenterX - centers!.labelCenterX)).toBeLessThan(4);
  });

  test("PC-02: progress circle color variants render", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    await expectPreviewVariants(page, ["progress-circle-colors"]);
    await expect(page.getByTestId("progress-circle-colors").locator(".orbital-progress-circle")).toHaveCount(2);
  });

  test("PC-03: custom size applies css variable", async ({ page }) => {
    await openComponentPreview(page, "progress-bar");
    await expectPreviewVariants(page, ["progress-circle-size"]);
    const circle = page.getByTestId("progress-circle-size").locator(".orbital-progress-circle");
    const size = await circle.evaluate((el) => getComputedStyle(el).getPropertyValue("--orbital-progress-circle-size").trim());
    expect(size).toBe("80px");
    const box = await circle.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeCloseTo(80, 0);
  });
});
