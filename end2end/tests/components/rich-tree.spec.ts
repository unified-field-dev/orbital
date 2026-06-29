import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
test.describe("rich-tree preview", () => {
  test("RT-01: default data-driven tree renders nested nodes", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expect(page.getByText("Documents")).toBeVisible();
    await expect(page.getByText("readme.md")).toBeVisible();
  });

  test("RT-02: controlled multi selection", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-selection"]);
    const root = page.getByTestId("rich-tree-selection");
    await root.getByText("Beta").click();
    await expect(root.locator(".orbital-tree-item--selected")).toHaveCount(1);
  });

  test("RT-03: controlled expansion shows nested child", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-expansion"]);
    await expect(page.getByTestId("rich-tree-expansion").getByText("Child")).toBeVisible();
  });

  test("RT-04: disabled item is marked disabled", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-disabled"]);
    const root = page.getByTestId("rich-tree-disabled");
    await expect(root.locator(".orbital-tree-item--disabled")).toHaveCount(1);
  });

  test("RT-05: lazy branch loads child on expand", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-lazy"]);
    const root = page.getByTestId("rich-tree-lazy");
    await root.getByText("Lazy branch").click();
    await expect(root.getByText("Loaded child")).toBeVisible({ timeout: 10_000 });
  });

  test("RT-06: virtualized subtree renders windowed items", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-virtual"]);
    const root = page.getByTestId("rich-tree-virtual");
    await expect(root.locator(".orbital-tree-virtual-scroll")).toBeVisible();
    await expect(root.getByText("Item 0")).toBeVisible();
    await expect(root.getByText("Item 119")).toBeHidden();
  });

  test("RT-07: multi selection ctrl and shift", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-selection"]);
    const root = page.getByTestId("rich-tree-selection");
    const item = (label: string) =>
      root.locator(".orbital-tree-item").filter({ hasText: label });

    await item("Alpha").click();
    await item("Gamma").click({ modifiers: ["ControlOrMeta"] });
    await expect(root.locator(".orbital-tree-item--selected")).toHaveCount(2);

    await item("Alpha").click();
    await item("Gamma").click({ modifiers: ["Shift"] });
    await expect(root.locator(".orbital-tree-item--selected")).toHaveCount(3);

    await item("Beta").click();
    await expect(root.locator(".orbital-tree-item--selected")).toHaveCount(1);
  });

  test("RT-08: inline editing updates label", async ({ page }) => {
    await openComponentPreview(page, "rich-tree", "rich-tree-preview");
    await expectPreviewVariants(page, ["rich-tree-editing"]);
    const root = page.getByTestId("rich-tree-editing");
    const item = root.locator(".orbital-tree-item").filter({ hasText: "Rename me" });
    await item.dblclick();
    const input = root.locator(".orbital-tree-item-layout__label-input");
    await expect(input).toBeVisible();
    await expect(input).toHaveValue("Rename me");
    await input.fill("Renamed node");
    await input.press("Enter");
    await expect(root.locator(".orbital-tree-item").filter({ hasText: "Renamed node" })).toBeVisible();
    await root.locator(".orbital-tree-item").filter({ hasText: "Renamed node" }).dblclick();
    await expect(input).toHaveValue("Renamed node");
  });
});
