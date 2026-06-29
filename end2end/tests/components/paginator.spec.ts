import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("paginator auto preview", () => {
  test("PN-01: default paginator shows ten pages", async ({ page }) => {
    await openComponentPreview(page, "paginator");
    const root = page.getByTestId("paginator-preview").getByTestId("orbital-paginator").first();
    await expect(root.getByRole("button", { name: "1", exact: true })).toBeVisible();
    await expect(root.getByRole("button", { name: "10", exact: true })).toBeVisible();
  });

  test("PN-02: small dataset shows three pages", async ({ page }) => {
    await openComponentPreview(page, "paginator");
    await expectPreviewVariants(page, ["paginator-small"]);
    const root = page.getByTestId("paginator-small").getByTestId("orbital-paginator");
    await expect(root.getByRole("button", { name: "3" })).toBeVisible();
  });

  test("PN-03: unknown total shows single page with next disabled", async ({ page }) => {
    await openComponentPreview(page, "paginator");
    await expectPreviewVariants(page, ["paginator-unknown"]);
    const root = page.getByTestId("paginator-unknown").getByTestId("orbital-paginator");
    await expect(root.getByRole("button", { name: "1" })).toBeVisible();
    const next = root.locator(".orbital-pagination-item").last();
    await expect(next).toHaveClass(/orbital-button--disabled/);
  });

  test("PN-04: clicking page 2 updates offset", async ({ page }) => {
    await openComponentPreview(page, "paginator");
    await expectPreviewVariants(page, ["paginator-offset"]);
    const root = page.getByTestId("paginator-offset").getByTestId("orbital-paginator");
    await root.getByRole("button", { name: "2" }).click();
    await expect(page.getByTestId("paginator-offset-value")).toHaveText("25");
  });

  test("PN-05: theme surfaces on active page button", async ({ page }) => {
    await openComponentPreview(page, "paginator");
    await expectPreviewVariants(page, ["paginator-theme"]);
    await expectNonEmptyResolvedStyle(page, "paginator-theme", "background-color", {
      childSelector: ".orbital-button--primary",
    });
  });
});
