import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion citations preview", () => {
  test("citation affordance, menu extras, and default actions", async ({ page }) => {
    await openComponentPreview(page, "discussion-citations");
    const preview = page.getByTestId("discussion-citations-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });
    await expect(preview.getByTestId("discussion-citation-list")).toBeVisible();

    const affordance = preview.getByTestId("discussion-citation-affordance-cit-1");
    await expect(affordance).toBeVisible();
    await expect(affordance.getByRole("button", { name: "Agree" })).toBeVisible();

    const menu = preview.getByTestId("discussion-citation-menu-cit-1");
    await menu.getByRole("button", { name: "Citation actions" }).click();
    await page.getByTestId("discussion-citation-action-mark_agree").click();
    await expect(page.getByTestId("discussion-citations-action-log")).toHaveText("cit-1:mark_agree");

    await menu.getByRole("button", { name: "Citation actions" }).click();
    await page.getByRole("menuitem", { name: "Open link" }).click();
    await expect(page.getByTestId("discussion-citations-action-log")).toHaveText("cit-1:open_link");
  });

  test("citation ref superscript links to citation row", async ({ page }) => {
    await openComponentPreview(page, "discussion-citations");
    const preview = page.getByTestId("discussion-citations-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const citationRef = preview.locator("#discussion-citation-ref-cit-1");
    await expect(citationRef).toBeVisible();
    await expect(citationRef).toHaveAttribute("href", "#discussion-citation-row-cit-1");
  });
});
