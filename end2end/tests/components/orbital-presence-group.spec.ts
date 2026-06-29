import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("orbital presence group preview", () => {
  test("OPG-01: starts empty and add item shows staggered tile", async ({ page }) => {
    await openComponentPreview(page, "orbital-presence-group", "orbital-presence-group-preview");
    const wrapper = page.getByTestId("orbital-presence-group-preview").first();
    await expect(wrapper).toBeVisible({ timeout: 30_000 });
    await expect(wrapper.getByTestId("orbital-presence-group-empty")).toBeVisible();

    await wrapper.getByRole("button", { name: "Add item" }).click();
    await expect(wrapper.getByTestId("orbital-presence-group-empty")).toBeHidden();
    await expect(wrapper.getByTestId("orbital-presence-group-tile-1")).toBeVisible();

    await wrapper.getByRole("button", { name: "Add item" }).click();
    await expect(wrapper.getByTestId("orbital-presence-group-tile-2")).toBeVisible();
  });

  test("OPG-02: remove item hides last tile", async ({ page }) => {
    await openComponentPreview(page, "orbital-presence-group", "orbital-presence-group-preview");
    const wrapper = page.getByTestId("orbital-presence-group-preview").first();
    await wrapper.getByRole("button", { name: "Add item" }).click();
    await wrapper.getByRole("button", { name: "Add item" }).click();
    await expect(wrapper.getByTestId("orbital-presence-group-tile-2")).toBeVisible();

    await wrapper.getByRole("button", { name: "Remove item" }).click();
    await expect(wrapper.getByTestId("orbital-presence-group-tile-2")).toBeHidden();
    await expect(wrapper.getByTestId("orbital-presence-group-tile-1")).toBeVisible();
  });
});
