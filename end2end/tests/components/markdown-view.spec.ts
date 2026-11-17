import AxeBuilder from "@axe-core/playwright";
import { test, expect, type Page } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";

const EXAMPLES = ["markdown-view-preview", "markdown-view-wide"];

async function open(page: Page) {
  await openComponentPreview(page, "markdown-view");
  return page.getByTestId("markdown-view-preview").locator(".orbital-markdown");
}

test.describe("markdown view preview", () => {
  test("MV-01: a GFM table renders with a header row and body rows", async ({ page }) => {
    const view = await open(page);
    const table = view.locator("table.orbital-markdown__table");
    await expect(table).toBeVisible();
    await expect(table.locator("thead th")).toHaveText(["Feature", "Crate", "Status"]);
    await expect(table.locator("tbody tr")).toHaveCount(3);
    await expect(table.getByRole("link", { name: "Tables" })).toHaveAttribute(
      "href",
      "https://example.com",
    );
    await expect(table.locator("tbody td code").first()).toHaveText("orbital-datatable");
    await expect(table.locator("tbody tr").first().locator("td").nth(2)).toHaveCSS(
      "text-align",
      "center",
    );
  });

  test("MV-02: blockquote, heading and rule render", async ({ page }) => {
    const view = await open(page);
    await expect(view.locator("blockquote strong")).toHaveText("Product UI in Rust.");
    await expect(view.getByRole("heading", { name: "Features", level: 2 })).toBeVisible();
    await expect(view.locator("hr.orbital-markdown__rule")).toHaveCount(1);
    await expect(view.locator("em")).toHaveText("Updated weekly.");
  });

  test("MV-03: a wide table scrolls inside its own box", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const example = page.getByTestId("markdown-view-wide");
    const scroller = example.locator(".orbital-markdown__table-scroll");
    const sizes = await scroller.evaluate((el) => ({
      scroll: el.scrollWidth,
      client: el.clientWidth,
    }));
    expect(sizes.scroll).toBeGreaterThan(sizes.client);
    const box = await example.boundingBox();
    expect(box?.width ?? 0).toBeLessThanOrEqual(320);
    await scroller.focus();
    await expect(scroller).toBeFocused();
  });

  test("MV-A1: axe clean in dark and light at 390px and desktop", async ({ page }) => {
    await openComponentPreview(page, "markdown-view");
    const scan = async (label: string) => {
      for (const testId of EXAMPLES) {
        await page.getByTestId(testId).scrollIntoViewIfNeeded();
        const results = await new AxeBuilder({ page })
          .include(`[data-testid='${testId}']`)
          .analyze();
        expect(
          results.violations.filter((v) => v.impact === "critical" || v.impact === "serious"),
          `${label} ${testId}`,
        ).toEqual([]);
      }
    };
    const desktop = page.viewportSize() ?? { width: 1280, height: 720 };

    // The preview shell starts in dark mode.
    await scan("dark desktop");
    await page.setViewportSize({ width: 390, height: 844 });
    await scan("dark 390px");

    await page.setViewportSize(desktop);
    const themeSwitch = page.getByTestId("preview-theme-toggle").locator("input[role=switch]");
    await themeSwitch.click();
    await expect(themeSwitch).not.toBeChecked();
    await scan("light desktop");
    await page.setViewportSize({ width: 390, height: 844 });
    await scan("light 390px");
  });
});
