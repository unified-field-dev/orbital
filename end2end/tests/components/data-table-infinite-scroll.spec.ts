import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table infinite scroll", () => {
  test("scroll loads additional rows and shows end marker", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-infinite-scroll-preview");
    const preview = page.getByTestId("data-table-infinite-scroll-preview");
    await preview.scrollIntoViewIfNeeded();

    const scrollHost = preview.getByTestId("data-table-scroll");
    await expect(preview.locator("tbody tr").first()).toBeVisible();

    const initialCount = await preview.locator("tbody tr").count();
    expect(initialCount).toBeGreaterThan(0);

    await scrollHost.evaluate((el) => {
      el.scrollTop = el.scrollHeight;
    });

    await expect.poll(async () => preview.locator("tbody tr").count()).toBeGreaterThan(initialCount);

    await expect
      .poll(async () => {
        await scrollHost.evaluate((el) => {
          el.scrollTop = el.scrollHeight;
        });
        return preview.getByTestId("data-table-infinite-end").isVisible();
      })
      .toBe(true);
  });

  test("uses footer spinner only (no duplicate loading overlay)", async ({ page }) => {
    await openComponentPreview(page, "data-table-data-source", "data-table-infinite-scroll-preview");
    const preview = page.getByTestId("data-table-infinite-scroll-preview");
    await preview.scrollIntoViewIfNeeded();

    await expect(preview.getByTestId("data-table-loading")).toHaveCount(0);
    await expect(preview.locator("tbody tr").first()).toBeVisible({ timeout: 15000 });
  });
});
