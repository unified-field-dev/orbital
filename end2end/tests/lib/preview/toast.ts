import type { Page } from "@playwright/test";
import { expect } from "@playwright/test";

const TOAST_POSITION_TOLERANCE_PX = 24;

export type ToastCorner =
  | "top-start"
  | "top-end"
  | "top"
  | "bottom-start"
  | "bottom-end"
  | "bottom";

function visibleToastStacks(page: Page) {
  return page.locator(".orbital-toast-stack").filter({ has: page.locator(".orbital-toast") });
}

/** First dispatched toast in the default stack. */
export function firstDispatchedToast(page: Page) {
  return page.locator(".orbital-toast-stack .orbital-toast").first();
}

/** Click a button inside a toast preview wrapper. */
export async function clickToastPreviewButton(
  page: Page,
  wrapperTestId: string,
  name: string,
  exact?: boolean,
) {
  await page.getByTestId(wrapperTestId).getByRole("button", { name, exact }).click();
}

/** Dispatch from the positions fixture and assert viewport corner placement. */
export async function expectToastAtCorner(
  page: Page,
  wrapperTestId: string,
  buttonName: string,
  corner: ToastCorner,
) {
  await clickToastPreviewButton(page, wrapperTestId, buttonName, true);
  await expectToastNearViewportCorner(page, corner);
}

/** Assert toast stack has expected position modifier and data attribute. */
export async function expectToastStackPosition(page: Page, position: string) {
  const stack = visibleToastStacks(page).first();
  await expect(stack).toBeVisible({ timeout: 10_000 });
  await expect(stack).toHaveClass(new RegExp(`orbital-toast-stack--${position.replace("-", "\\-")}`));
  await expect(stack).toHaveAttribute("data-orbital-toast-position", position);
}

/** Wait for a specific number of visible dispatched toasts. */
export async function expectVisibleToastCount(page: Page, count: number) {
  await expect(page.locator(".orbital-toast-stack .orbital-toast")).toHaveCount(count, {
    timeout: 10_000,
  });
}

/** Assert stack is pinned near a viewport corner (within tolerance px). */
export async function expectToastNearViewportCorner(page: Page, corner: ToastCorner) {
  const stack = visibleToastStacks(page).first();
  await expect(stack).toBeVisible({ timeout: 10_000 });
  const box = await stack.boundingBox();
  expect(box).not.toBeNull();
  const viewport = page.viewportSize();
  expect(viewport).not.toBeNull();

  const style = await stack.evaluate((el) => {
    const computed = getComputedStyle(el);
    return {
      offsetX: Number.parseFloat(computed.getPropertyValue("--orbital-toast-offset-x")) || 20,
      offsetY: Number.parseFloat(computed.getPropertyValue("--orbital-toast-offset-y")) || 16,
    };
  });

  const { offsetX, offsetY } = style;
  const tol = TOAST_POSITION_TOLERANCE_PX;

  switch (corner) {
    case "top-start":
      expect(Math.abs(box!.x - offsetX)).toBeLessThanOrEqual(tol);
      expect(Math.abs(box!.y - offsetY)).toBeLessThanOrEqual(tol);
      break;
    case "top-end":
      expect(Math.abs(viewport!.width - (box!.x + box!.width) - offsetX)).toBeLessThanOrEqual(tol);
      expect(Math.abs(box!.y - offsetY)).toBeLessThanOrEqual(tol);
      break;
    case "top": {
      const centerX = box!.x + box!.width / 2;
      expect(Math.abs(centerX - viewport!.width / 2)).toBeLessThanOrEqual(tol + offsetX);
      expect(Math.abs(box!.y - offsetY)).toBeLessThanOrEqual(tol);
      break;
    }
    case "bottom-start":
      expect(Math.abs(box!.x - offsetX)).toBeLessThanOrEqual(tol);
      expect(Math.abs(viewport!.height - (box!.y + box!.height) - offsetY)).toBeLessThanOrEqual(tol);
      break;
    case "bottom-end":
      expect(Math.abs(viewport!.width - (box!.x + box!.width) - offsetX)).toBeLessThanOrEqual(tol);
      expect(Math.abs(viewport!.height - (box!.y + box!.height) - offsetY)).toBeLessThanOrEqual(tol);
      break;
    case "bottom": {
      const centerX = box!.x + box!.width / 2;
      expect(Math.abs(centerX - viewport!.width / 2)).toBeLessThanOrEqual(tol + offsetX);
      expect(Math.abs(viewport!.height - (box!.y + box!.height) - offsetY)).toBeLessThanOrEqual(tol);
      break;
    }
  }
}

/** Assert toast stack bounding box is inside a container testid (inline mode). */
export async function expectToastInsideContainer(page: Page, containerTestId: string) {
  const container = page.getByTestId(containerTestId);
  const stack = visibleToastStacks(page).first();
  await expect(stack).toBeVisible({ timeout: 10_000 });
  const containerBox = await container.boundingBox();
  const stackBox = await stack.boundingBox();
  expect(containerBox).not.toBeNull();
  expect(stackBox).not.toBeNull();
  expect(stackBox!.x).toBeGreaterThanOrEqual(containerBox!.x - 1);
  expect(stackBox!.y).toBeGreaterThanOrEqual(containerBox!.y - 1);
  expect(stackBox!.x + stackBox!.width).toBeLessThanOrEqual(containerBox!.x + containerBox!.width + 1);
  expect(stackBox!.y + stackBox!.height).toBeLessThanOrEqual(containerBox!.y + containerBox!.height + 1);
}

/** Assert toast stack is outside a container bounding box (portaled fixed mode). */
export async function expectToastOutsideContainer(page: Page, containerTestId: string) {
  const container = page.getByTestId(containerTestId);
  const stack = visibleToastStacks(page).first();
  await expect(stack).toBeVisible({ timeout: 10_000 });
  const containerBox = await container.boundingBox();
  const stackBox = await stack.boundingBox();
  expect(containerBox).not.toBeNull();
  expect(stackBox).not.toBeNull();
  const inside =
    stackBox!.x >= containerBox!.x &&
    stackBox!.y >= containerBox!.y &&
    stackBox!.x + stackBox!.width <= containerBox!.x + containerBox!.width &&
    stackBox!.y + stackBox!.height <= containerBox!.y + containerBox!.height;
  expect(inside).toBe(false);
}
