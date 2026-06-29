import { expectSurfaceStyled } from "../lib/assertions/style";
import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
import { expectOverlayAnchoredNearTrigger } from "../lib/preview/overlays";
test.describe("date-picker preview behaviors", () => {
  test("DP-01 renders default input", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    await expect(page.getByTestId("date-picker-preview").locator("input")).toBeVisible({ timeout: 30_000 });
  });

  test("DP-02 preselected unix value populates input", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const input = page.getByTestId("DP-02").locator("input");
    await expect(input).not.toHaveValue("");
  });

  test("DP-03 panel selection updates unix readout", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const wrapper = page.getByTestId("DP-03");
    const value = page.getByTestId("DP-03-VALUE");
    await expect(value).toHaveText("none");
    await wrapper.getByRole("button", { name: "Open calendar" }).click();
    await page.locator(".orbital-date-picker__day-button").nth(10).click();
    await expect(value).not.toHaveText("none");
  });

  test("DP-04 ISO format displays hyphen date", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const input = page.getByTestId("DP-04").locator("input");
    await expect(input).toHaveValue(/\d{4}-\d{2}-\d{2}/);
  });

  test("DP-05 overlay is anchored near trigger", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const wrapper = page.getByTestId("DP-05");
    await wrapper.getByRole("button", { name: "Open calendar" }).click();
    await expectOverlayAnchoredNearTrigger(
      page,
      "DP-05",
      ".orbital-date-picker__panel",
      "Open calendar",
    );
  });

  test("DP-06 panel surface has themed styling", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const wrapper = page.getByTestId("DP-06");
    await wrapper.getByRole("button", { name: "Open calendar" }).click();
    await expectSurfaceStyled(page, ".orbital-date-picker__panel");
  });

  test("DP-07 required validation shows on blur", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const input = page.getByTestId("DP-07").locator("input");
    await input.focus();
    await input.blur();
    await expect(page.getByTestId("DP-07").locator(".orbital-field__validation-message")).toBeVisible();
  });

  test("DP-08 disabled input blocks interaction", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const wrapper = page.getByTestId("DP-08");
    const input = wrapper.locator("input");
    await expect(input).toBeDisabled();
    await wrapper.getByRole("button", { name: "Open calendar" }).click({ force: true });
    await expect(page.locator(".orbital-date-picker__panel")).toHaveCount(0);
  });

  test("DP-09 formatted timezone readout is rendered", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const value = page.getByTestId("DP-09-TEXT");
    await expect(value).toHaveText(/\d{2}\/\d{2}\/\d{4}/);
  });

  test("DP-10 min/max validation shows on blur for out-of-range date", async ({ page }) => {
    await openComponentPreview(page, "date-picker");
    const wrapper = page.getByTestId("DP-10");
    const input = wrapper.locator("input");
    await input.fill("01/01/2020");
    await input.blur();
    await expect(wrapper.locator(".orbital-field__validation-message")).toBeVisible();
  });
});
