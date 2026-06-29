import { test, expect } from "@playwright/test";
import { previewUrl } from "../helpers";

test.describe("preview server smoke", () => {
  test("unknown slug shows not-found page", async ({ page }) => {
    await page.goto(previewUrl("/does-not-exist"));
    await expect(page.getByTestId("preview-not-found")).toBeVisible();
  });
});
