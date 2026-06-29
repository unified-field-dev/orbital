import { test, expect } from "@playwright/test";
import { openComponentPreview, expectPreviewVariants, scrollIntoPreviewView } from "../lib/preview/navigation";
test.describe("date-calendar preview behaviors", () => {
  test("renders preview page", async ({ page }) => {
    await openComponentPreview(page, "date-calendar");
    await expect(page.getByTestId("date-calendar-preview")).toBeVisible({ timeout: 30_000 });
  });

  test("shows documented example", async ({ page }) => {
    await openComponentPreview(page, "date-calendar");
    await expectPreviewVariants(page, ["date-calendar-preview"]);
  });

  test("clicking a day updates bound value text", async ({ page }) => {
    await openComponentPreview(page, "date-calendar");
    const wrapper = page.getByTestId("date-calendar-preview");
    await expect(wrapper).toBeVisible();

    const value = page.getByTestId("date-calendar-preview-VALUE");
    await expect(value).toHaveText("none");

    await wrapper.locator(".orbital-calendar-item:not(.orbital-calendar-item--disabled)").nth(10).click();
    await expect(value).not.toHaveText("none");
  });

  test("navigation changes visible month", async ({ page }) => {
    await openComponentPreview(page, "date-calendar");
    const wrapper = page.getByTestId("date-calendar-preview");
    await expect(wrapper).toBeVisible();

    const title = wrapper.getByTestId("calendar-header-title");
    const initial = await title.innerText();
    await wrapper.getByRole("button", { name: "Next" }).click();
    await expect(title).not.toHaveText(initial);
  });

  test("DC-04 min/max Field validation surfaces on value change", async ({ page }) => {
    await openComponentPreview(page, "date-calendar", "date-calendar-preview");
    const wrapper = page.getByTestId("DC-04");
    await scrollIntoPreviewView(wrapper);
    await expect(wrapper).toBeVisible();
    await wrapper.getByRole("button", { name: "Today" }).click();
    await expect(wrapper.locator(".orbital-field__validation-message")).toBeVisible({ timeout: 5_000 });
  });
});
