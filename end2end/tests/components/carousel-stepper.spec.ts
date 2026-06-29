import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("carousel-stepper preview", () => {
  test("CS-01: default stepper", async ({ page }) => {
    await openComponentPreview(page, "carousel-stepper");
    await expect(page.getByTestId("carousel-stepper-preview")).toBeVisible();
    await expect(
      page.getByTestId("carousel-stepper-preview").locator(".orbital-carousel__stepper"),
    ).toBeVisible();
    await expect(
      page.getByTestId("carousel-stepper-preview").getByRole("button", { name: "Next slide" }),
    ).toBeVisible();
  });

  test("CS-02: inline stepper overlay", async ({ page }) => {
    await openComponentPreview(page, "carousel-stepper");
    await expect(page.getByTestId("carousel-stepper-inline")).toBeVisible();
    await expect(
      page.getByTestId("carousel-stepper-inline").locator(".orbital-carousel__stepper--inline"),
    ).toBeVisible();
  });
});
