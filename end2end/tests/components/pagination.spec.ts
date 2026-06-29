import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("pagination primitive preview", () => {
  test("PG-01: ten pages with page 1 primary and prev disabled", async ({ page }) => {
    await openComponentPreview(page, "pagination");
    const root = page.getByTestId("pagination-preview");
    await expect(root.getByRole("button", { name: "1", exact: true })).toHaveClass(/orbital-button--primary/);
    const prev = root.locator(".orbital-pagination-item").first();
    await expect(prev).toHaveClass(/orbital-button--disabled/);
  });

  test("PG-02: three numbered page buttons", async ({ page }) => {
    await openComponentPreview(page, "pagination");
    await expectPreviewVariants(page, ["pagination-three"]);
    const root = page.getByTestId("pagination-three");
    await expect(root.getByRole("button", { name: "1", exact: true })).toBeVisible();
    await expect(root.getByRole("button", { name: "2" })).toBeVisible();
    await expect(root.getByRole("button", { name: "3" })).toBeVisible();
  });

  test("PG-03: clicking page 3 updates page signal", async ({ page }) => {
    await openComponentPreview(page, "pagination");
    await expectPreviewVariants(page, ["pagination-click"]);
    const root = page.getByTestId("pagination-click");
    await root.getByRole("button", { name: "3" }).click();
    await expect(root.getByRole("button", { name: "3" })).toHaveClass(/orbital-button--primary/);
    await expect(page.getByTestId("pagination-click-page")).toHaveText("3");
  });

  test("PG-04: ellipsis with first and last page buttons", async ({ page }) => {
    await openComponentPreview(page, "pagination");
    await expectPreviewVariants(page, ["pagination-ellipsis"]);
    const root = page.getByTestId("pagination-ellipsis");
    await expect(root.getByText("...").first()).toBeVisible();
    await expect(root.getByRole("button", { name: "1", exact: true })).toBeVisible();
    await expect(root.getByRole("button", { name: "20", exact: true })).toBeVisible();
  });

  test("PG-05: theme surfaces on active page button", async ({ page }) => {
    await openComponentPreview(page, "pagination");
    await expectPreviewVariants(page, ["pagination-theme"]);
    await expectNonEmptyResolvedStyle(page, "pagination-theme", "background-color", {
      childSelector: ".orbital-button--primary",
    });
  });
});
