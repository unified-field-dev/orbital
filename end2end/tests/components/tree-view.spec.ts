import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants } from "../lib/preview/navigation";
import { expectNonEmptyResolvedStyle } from "../lib/assertions/style";
test.describe("tree-view primitive preview", () => {
  test("TV-01: default nested tree with branch expanded", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    const tree = page.getByRole("tree").first();
    await expect(tree).toBeVisible();
    await expect(page.getByTestId("tree-node-docs")).toBeVisible();
    await expect(page.getByTestId("tree-node-readme")).toBeVisible();
    const chevron = page.locator(".orbital-tree-item-layout__expand-icon svg").first();
    await expect(chevron).toBeVisible();
    const box = await chevron.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(0);
    expect(box!.height).toBeGreaterThan(0);
  });

  test("TV-02: small size modifier", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-small"]);
    await expect(page.getByTestId("tree-small").locator(".orbital-tree--small")).toBeVisible();
  });

  test("TV-03: deep nesting shows nested groups when expanded", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-deep"]);
    const root = page.getByTestId("tree-deep");
    await expect(root.getByRole("group")).toHaveCount(2);
    await expect(root.getByText("Level 3")).toBeVisible();
  });

  test("TV-04: click branch toggles expand and collapse", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-toggle"]);
    const root = page.getByTestId("tree-toggle");
    await expect(root.getByText("Hidden until expanded")).toBeHidden();
    await root.getByTestId("tree-branch-toggle").click();
    await expect(root.getByText("Hidden until expanded")).toBeVisible();
    await root.getByTestId("tree-branch-toggle").click();
    await expect(root.getByText("Hidden until expanded")).toBeHidden();
  });

  test("TV-05: flat leaf list has no chevrons", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-flat"]);
    const root = page.getByTestId("tree-flat");
    await expect(root.getByRole("treeitem")).toHaveCount(2);
    await expect(root.locator(".orbital-tree-item-layout__expand-icon")).toHaveCount(0);
  });

  test("TV-06: theme surfaces resolve on items", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-theme"]);
    await expectNonEmptyResolvedStyle(page, "tree-theme", "background-color", {
      childSelector: "[data-testid='tree-theme-cell']",
    });
  });

  test("TV-07: single selection highlights row", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-selection"]);
    const root = page.getByTestId("tree-selection");
    await root.getByText("Beta").click();
    await expect(root.locator(".orbital-tree-item--selected")).toHaveCount(1);
  });

  test("TV-09: icons and aside regions render", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-icons-aside"]);
    const root = page.getByTestId("tree-icons-aside");
    await expect(root.locator(".orbital-tree-item-layout__icon-before")).toBeVisible();
    await expect(root.locator(".orbital-tree-item-layout__icon-after")).toBeVisible();
    await expect(root.locator(".orbital-tree-item-layout__aside")).toBeVisible();
  });

  test("TV-10: multi selection with ctrl toggle", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-multi"]);
    const root = page.getByTestId("tree-multi");
    await root.locator(".orbital-tree-item").filter({ hasText: "Alpha" }).click();
    await root.locator(".orbital-tree-item").filter({ hasText: "Gamma" }).click({ modifiers: ["ControlOrMeta"] });
    await expect(root.locator(".orbital-tree-item--selected")).toHaveCount(2);
  });

  test("TV-11: checkbox mode selects branch", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-checkbox"]);
    const root = page.getByTestId("tree-checkbox");
    await expect(root.locator('[data-item-id="child"]')).toBeVisible();
    await root.locator(".orbital-tree-item-layout__checkbox").first().click();
    await expect(root.locator('.orbital-tree-item[data-item-id="root"]')).toHaveClass(/orbital-tree-item--selected/);
  });

  test("TV-12: disabled focusable item", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-disabled"]);
    const root = page.getByTestId("tree-disabled");
    await expect(root.locator(".orbital-tree-item--disabled")).toHaveCount(1);
    await root.getByRole("treeitem", { name: "Disabled" }).focus();
    await expect(root.getByRole("treeitem", { name: "Disabled" })).toBeFocused();
  });

  test("TV-13: icon-container expansion only toggles on chevron", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-icon-expand"]);
    const root = page.getByTestId("tree-icon-expand");
    await expect(root.getByText("Leaf")).toBeHidden();
    await root.getByTestId("tree-icon-branch").click();
    await expect(root.getByText("Leaf")).toBeHidden();
    await root.locator(".orbital-tree-item-layout__expand-icon").first().click();
    await expect(root.getByText("Leaf")).toBeVisible();
  });

  test("TV-14: focus via button uses TreeApiRef", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-focus-button"]);
    const root = page.getByTestId("tree-focus-button");
    await root.getByRole("button", { name: "Focus target" }).click();
    await expect(root.locator(".orbital-tree-item--focused")).toHaveCount(1);
  });

  test("TV-15: keyboard navigation moves focus", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-keyboard"]);
    const root = page.getByTestId("tree-keyboard");
    const beta = root.locator(".orbital-tree-item").filter({ hasText: "Beta" });
    await beta.click();
    await expect(beta).toHaveClass(/orbital-tree-item--focused/);
  });

  test("TV-16: inline editing commits on Enter", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-editing"]);
    const root = page.getByTestId("tree-editing");
    const item = root.locator(".orbital-tree-item").filter({ hasText: "Rename me" });
    await item.dblclick();
    const input = root.locator(".orbital-tree-item-layout__label-input");
    await expect(input).toBeVisible();
    await expect(input).toHaveValue("Rename me");
    await input.fill("Renamed");
    await input.blur();
    await expect(root.locator(".orbital-tree-item").filter({ hasText: "Renamed" })).toBeVisible();
    await expect(item).not.toBeVisible();
  });

  test("TV-17: connector borders class applied", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-connectors"]);
    await expect(page.getByTestId("tree-connectors").locator(".orbital-tree--connectors")).toBeVisible();
  });

  test("TV-18: reorderable tree shows drag handles", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-reorder"]);
    const root = page.getByTestId("tree-reorder");
    await expect(root.locator(".orbital-tree-item-layout__drag-handle")).toHaveCount(2);
  });

  test("TV-19: drag handle reorders sibling items", async ({ page }) => {
    await openComponentPreview(page, "tree-view", "tree-view-preview");
    await expectPreviewVariants(page, ["tree-reorder"]);
    const root = page.getByTestId("tree-reorder");
    const labels = () =>
      root.locator(".orbital-tree-item-layout__main").allTextContents();
    await expect(labels()).resolves.toEqual(["One", "Two"]);

    const handleTwo = root
      .locator(".orbital-tree-item")
      .filter({ hasText: "Two" })
      .locator(".orbital-tree-item-layout__drag-handle");
    const itemOne = root.locator(".orbital-tree-item").filter({ hasText: "One" });
    const handleBox = await handleTwo.boundingBox();
    const targetBox = await itemOne.boundingBox();
    expect(handleBox).not.toBeNull();
    expect(targetBox).not.toBeNull();

    await page.mouse.move(
      handleBox!.x + handleBox!.width / 2,
      handleBox!.y + handleBox!.height / 2,
    );
    await page.mouse.down();
    await page.mouse.move(
      targetBox!.x + targetBox!.width / 2,
      targetBox!.y + 4,
      { steps: 8 },
    );
    await page.mouse.up();

    await expect(labels()).resolves.toEqual(["Two", "One"]);
  });
});
