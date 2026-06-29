import type { Locator } from "@playwright/test";

/** Hover a chart SVG mark; pointer layer hit-testing picks up bar coordinates. */
export async function hoverChartMark(locator: Locator) {
  const box = await locator.boundingBox();
  if (box) {
    const page = locator.page();
    const x = box.x + box.width / 2;
    const y = box.y + box.height / 2;
    await page.mouse.move(x, y);
    await page.waitForTimeout(100);
  }
  await locator.hover({ force: true });
  await locator.dispatchEvent("mouseenter");
  await locator.page().waitForTimeout(150);
}

/** Chart root inside a preview wrapper (`[data-orbital-chart]`). */
export function chartRootLocator(preview: Locator) {
  return preview.locator("[data-orbital-chart]").first();
}
