import type { Page } from "@playwright/test";
import { expect } from "@playwright/test";

export type PreviewDocTab = "Description" | "Best Practices" | "Properties";

/** Assert Description, Best Practices, and Properties tabs are visible. */
export async function expectPreviewDocTabs(page: Page) {
  await expect(page.getByRole("tab", { name: "Description" })).toBeVisible();
  await expect(page.getByRole("tab", { name: "Best Practices" })).toBeVisible();
  await expect(page.getByRole("tab", { name: "Properties" })).toBeVisible();
}

/** Activate a doc panel tab by label. */
export async function clickPreviewDocTab(page: Page, name: PreviewDocTab) {
  await page.getByRole("tab", { name }).click();
  await expect(page.getByTestId("preview-doc-content")).toBeVisible();
}

/** Assert content inside the active doc tab panel. */
export async function expectPreviewDocContent(
  page: Page,
  opts: {
    contains?: string[];
    notContains?: string[];
    noPreBlocks?: boolean;
  } = {},
) {
  const panel = page.getByTestId("preview-doc-content");
  await expect(panel).toBeVisible();

  const text = await panel.innerText();
  for (const snippet of opts.contains ?? []) {
    expect(text).toContain(snippet);
  }
  for (const snippet of opts.notContains ?? []) {
    expect(text).not.toContain(snippet);
  }
  if (opts.noPreBlocks) {
    await expect(panel.locator("pre")).toHaveCount(0);
  }
}
