import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("carousel preview", () => {
  test("C-01 default carousel", async ({ page }) => {
    await openComponentPreview(page, "carousel");
    await expect(page.getByTestId("carousel-preview")).toBeVisible();
    await expect(
      page.getByTestId("carousel-preview").locator(".orbital-carousel__slide"),
    ).toHaveCount(3);
  });

  test("C-02 next and previous navigation", async ({ page }) => {
    await openComponentPreview(page, "carousel");
    await expectPreviewVariants(page, ["carousel-next-prev"]);

    const section = page.getByTestId("carousel-next-prev");
    await expect(section.getByTestId("carousel-active-index")).toContainText("1");

    await section.getByRole("button", { name: "Next slide" }).click();
    await expect(section.getByTestId("carousel-active-index")).toContainText("2");

    await section.getByRole("button", { name: "Previous slide" }).click();
    await expect(section.getByTestId("carousel-active-index")).toContainText("1");
  });

  test("C-03 wrap navigation", async ({ page }) => {
    await openComponentPreview(page, "carousel");
    await expectPreviewVariants(page, ["carousel-wrap"]);

    const section = page.getByTestId("carousel-wrap");
    await expect(section.getByTestId("carousel-wrap-index")).toContainText("0");
    await section.getByRole("button", { name: "Next slide" }).click();
    await expect(section.getByTestId("carousel-wrap-index")).not.toContainText("Index: 0");
    await section.getByRole("button", { name: "Next slide" }).click();
    await section.getByRole("button", { name: "Next slide" }).click();
    await expect(section.getByTestId("carousel-wrap-index")).toContainText("0");
  });
});
