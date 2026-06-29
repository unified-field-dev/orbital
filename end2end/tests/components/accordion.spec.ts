import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("accordion primitive preview", () => {
  test("AC-01: default panel open", async ({ page }) => {
    await openComponentPreview(page, "accordion");
    await expect(page.getByTestId("accordion-panel-one")).toBeVisible();
    await expect(page.getByTestId("accordion-panel-two")).toBeHidden();
  });

  test("AC-02: multiple panels can open", async ({ page }) => {
    await openComponentPreview(page, "accordion");
    await expectPreviewVariants(page, ["accordion-multiple"]);
    const root = page.getByTestId("accordion-multiple");
    await root.getByRole("button", { name: "A" }).click();
    await root.getByRole("button", { name: "B" }).click();
    await expect(root.getByTestId("accordion-panel-a")).toBeVisible();
    await expect(root.getByTestId("accordion-panel-b")).toBeVisible();
  });

  test("AC-03: collapsible closes last open panel", async ({ page }) => {
    await openComponentPreview(page, "accordion");
    await expectPreviewVariants(page, ["accordion-collapsible"]);
    const root = page.getByTestId("accordion-collapsible");
    await expect(root.getByTestId("accordion-panel-only")).toBeVisible();
    await root.getByRole("button", { name: "Only section" }).click();
    await expect(root.getByTestId("accordion-panel-only")).toBeHidden();
  });

  test("AC-04: keyboard Enter toggles panel", async ({ page }) => {
    await openComponentPreview(page, "accordion");
    await expectPreviewVariants(page, ["accordion-keyboard"]);
    const root = page.getByTestId("accordion-keyboard");
    const header = root.getByRole("button", { name: "Keyboard section" });
    await header.focus();
    await page.keyboard.press("Enter");
    await expect(root.getByTestId("accordion-panel-kb")).toBeVisible();
    await page.keyboard.press("Enter");
    await expect(root.getByTestId("accordion-panel-kb")).toBeHidden();
  });

  test("AC-05: theme header color token", async ({ page }) => {
    await openComponentPreview(page, "accordion");
    await expectPreviewVariants(page, ["accordion-theme"]);
    await expectNonEmptyResolvedStyle(page, "accordion-theme", "color", {
      childSelector: ".orbital-accordion-header",
    });
  });
});
