import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("data-table programmatic scroll", () => {
  test("scroll_to_row brings target row into view", async ({ page }) => {
    await openComponentPreview(page, "data-table-rendering", "data-table-scroll-preview");
    const preview = page.getByTestId("data-table-scroll-preview");
    await preview.scrollIntoViewIfNeeded();

    const scroll = preview.getByTestId("data-table-scroll");
    await scroll.evaluate((el) => {
      el.scrollTop = 0;
    });

    await preview.getByTestId("scroll-to-row-25").click();
    await expect(preview.getByTestId("data-table-row-25")).toBeVisible({ timeout: 10_000 });

    await expect
      .poll(async () => scroll.evaluate((el) => el.scrollTop), { timeout: 20_000 })
      .toBeGreaterThan(0);
  });
});
