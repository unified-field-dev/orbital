import type { Locator } from "@playwright/test";
import { expect } from "@playwright/test";

/** Assert grid-template-columns resolves to the expected track count. */
export async function expectGridColumnCount(
  grid: Locator,
  count: number,
): Promise<void> {
  const tracks = await grid.evaluate((el) => {
    const value = getComputedStyle(el).gridTemplateColumns;
    return value.split(/\s+/).filter(Boolean);
  });
  expect(tracks.length).toBe(count);
}

/** Assert center element is horizontally between left and right elements. */
export async function expectHorizontallyBetween(
  left: Locator,
  center: Locator,
  right: Locator,
): Promise<void> {
  const [leftBox, centerBox, rightBox] = await Promise.all([
    left.first().boundingBox(),
    center.first().boundingBox(),
    right.first().boundingBox(),
  ]);
  expect(leftBox).not.toBeNull();
  expect(centerBox).not.toBeNull();
  expect(rightBox).not.toBeNull();

  const leftMid = leftBox!.x + leftBox!.width / 2;
  const centerMid = centerBox!.x + centerBox!.width / 2;
  const rightMid = rightBox!.x + rightBox!.width / 2;

  expect(leftMid).toBeLessThan(centerMid);
  expect(centerMid).toBeLessThan(rightMid);
}
