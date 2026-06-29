import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test("CS-03: inline stepper dots sit below arrows", async ({ page }) => {
  await openComponentPreview(page, "carousel-stepper");
  const inline = page.getByTestId("carousel-stepper-inline");
  const stepper = inline.locator(".orbital-carousel__stepper--inline");
  const carousel = inline.locator(".orbital-carousel");
  await expect(stepper).toBeVisible();

  const layout = await stepper.evaluate((el) => getComputedStyle(el).display);
  expect(layout).toBe("grid");

  const boxes = await stepper.evaluate((nav) => {
    const navBox = nav.getBoundingClientRect();
    const prev = nav.querySelector(".orbital-carousel__stepper-button:first-of-type");
    const next = nav.querySelector(".orbital-carousel__stepper-button:last-of-type");
    const dots = nav.querySelector(".orbital-carousel__indicators");
    const prevBox = prev?.getBoundingClientRect();
    const nextBox = next?.getBoundingClientRect();
    const dotsBox = dots?.getBoundingClientRect();
    return { navBox, prevBox, nextBox, dotsBox };
  });

  expect(boxes.prevBox).toBeTruthy();
  expect(boxes.nextBox).toBeTruthy();
  expect(boxes.dotsBox).toBeTruthy();

  const prevCenterY = boxes.prevBox!.top + boxes.prevBox!.height / 2;
  const nextCenterY = boxes.nextBox!.top + boxes.nextBox!.height / 2;
  const dotsCenterY = boxes.dotsBox!.top + boxes.dotsBox!.height / 2;
  const navCenterY = boxes.navBox.top + boxes.navBox.height / 2;

  // Arrows stay near vertical center of the overlay.
  expect(Math.abs(prevCenterY - navCenterY)).toBeLessThan(boxes.navBox.height * 0.2);
  expect(Math.abs(nextCenterY - navCenterY)).toBeLessThan(boxes.navBox.height * 0.2);

  // Dots sit along the bottom edge, not centered with the arrows.
  expect(dotsCenterY).toBeGreaterThan(navCenterY);
  expect(Math.abs(boxes.dotsBox!.bottom - boxes.navBox.bottom)).toBeLessThan(4);

  await expect(carousel).toBeVisible();
});
