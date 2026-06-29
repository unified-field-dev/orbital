import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion slots preview", () => {
  test("custom toolbar and sample replies render", async ({ page }) => {
    await openComponentPreview(page, "discussion-slots");
    const preview = page.getByTestId("discussion-slots-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const toolbar = preview.getByTestId("discussion-custom-toolbar");
    await expect(toolbar).toBeVisible();
    await expect(toolbar.getByRole("button", { name: "Expand all" })).toBeVisible();
    await expect(toolbar.getByRole("button", { name: "Collapse replies" })).toBeVisible();
    await expect(toolbar).toContainText("Custom slot toolbar");

    await expect(preview.locator("[data-reply-id='slots-root']")).toBeVisible();
    await expect(preview.locator("[data-reply-id='slots-reply']")).toBeVisible();
    await expect(preview.getByTestId("discussion-empty-default")).toHaveCount(0);
  });
});
