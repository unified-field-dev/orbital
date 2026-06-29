import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("transfer-list preview", () => {
  test("TL-01: move selected item from left to right", async ({ page }) => {
    await openComponentPreview(page, "transfer-list");
    const preview = page.getByTestId("transfer-list-preview");
    const left = preview.getByTestId("transfer-list-left");
    await left.getByTestId("transfer-item-1").click();
    await preview.getByRole("button", { name: ">", exact: true }).click();
    await expect(preview.getByTestId("transfer-list-right").getByTestId("transfer-item-1")).toBeVisible();
    await expect(left.getByTestId("transfer-item-1")).toHaveCount(0);
  });

  test("TL-02: enhanced header shows selection counter", async ({ page }) => {
    await openComponentPreview(page, "transfer-list", "transfer-list-enhanced");
    const preview = page.getByTestId("transfer-list-enhanced");
    await preview.scrollIntoViewIfNeeded();
    const left = preview.getByTestId("transfer-list-left");
    const leftHeader = left.locator("xpath=preceding-sibling::div[contains(@class, 'orbital-transfer-list__header')]");
    await left.getByTestId("transfer-item-l-1").click();
    await expect(leftHeader).toContainText("1/4 selected");
    await leftHeader.getByRole("checkbox").click();
    await expect(leftHeader).toContainText("4/4 selected");
  });

  test("TL-03: custom labels remain visible after move", async ({ page }) => {
    await openComponentPreview(page, "transfer-list", "transfer-list-labels");
    const preview = page.getByTestId("transfer-list-labels");
    await preview.scrollIntoViewIfNeeded();
    const left = preview.getByTestId("transfer-list-left");
    await left.getByTestId("transfer-item-alpha").click();
    await preview.getByRole("button", { name: ">", exact: true }).click();
    await expect(preview.getByTestId("transfer-list-right")).toContainText("Alpha team");
  });

  test("TL-04: disabled item cannot be moved", async ({ page }) => {
    await openComponentPreview(page, "transfer-list", "transfer-list-disabled");
    const preview = page.getByTestId("transfer-list-disabled");
    await preview.scrollIntoViewIfNeeded();
    const left = preview.getByTestId("transfer-list-left");
    const locked = left.getByRole("checkbox", { name: "Locked item" });
    await expect(locked).toBeDisabled();
    await left.getByTestId("transfer-item-open").click();
    await preview.getByRole("button", { name: ">>", exact: true }).click();
    await expect(left.getByTestId("transfer-item-locked")).toBeVisible();
    await expect(preview.getByTestId("transfer-list-right").getByTestId("transfer-item-open")).toBeVisible();
  });
});
