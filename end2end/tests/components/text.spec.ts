import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("text typography preview", () => {

  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "text");
    await expect(page.getByTestId("text-preview")).toBeVisible();
    await expect(page.getByTestId("text-type-ramp")).toContainText("Display");
  });

  test("shows Orbital form helpers", async ({ page }) => {
    await openComponentPreview(page, "text");
    await expect(page.getByTestId("text-form-helpers")).toContainText("Email address");
    await expect(page.getByTestId("text-form-helpers")).toContainText("Account settings");
  });

  test("truncate prop applies ellipsis styling", async ({ page }) => {
    await openComponentPreview(page, "text");
    await page.getByTestId("text-truncate").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("text-truncate")).toBeVisible();
    await expect(page.getByTestId("text-truncate")).toHaveCSS("text-overflow", "ellipsis");
  });

  test("align and wrap props affect layout", async ({ page }) => {
    await openComponentPreview(page, "text");
    await page.getByTestId("text-align-start").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("text-align-start")).toHaveCSS("text-align", "start");
    await page.getByTestId("text-nowrap").scrollIntoViewIfNeeded();
    await expect(page.getByTestId("text-nowrap")).toHaveCSS("white-space", "nowrap");
  });
});
