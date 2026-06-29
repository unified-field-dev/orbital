import { test, expect } from "@playwright/test";
import { clickPreviewDocTab, expectPreviewDocContent, expectPreviewDocTabs } from "../lib/preview/docs";
import { expectPreviewPageTitle, openComponentPreview } from "../lib/preview/navigation";
import { previewUrl, waitForPreviewShell } from "../helpers";

test.describe("preview doc panel shell", () => {
  test("divider: minimal description without code fences", async ({ page }) => {
    await openComponentPreview(page, "divider");
    await expectPreviewPageTitle(page, "Divider");
    await expectPreviewDocTabs(page);
    await expectPreviewDocContent(page, {
      contains: ["Visual separator"],
      noPreBlocks: true,
    });
  });

  test("flex: description excludes usage code fences", async ({ page }) => {
    await openComponentPreview(page, "flex");
    await expectPreviewPageTitle(page, "Flex");
    await expectPreviewDocContent(page, {
      contains: ["one-dimensional", "Choose direction"],
      notContains: ["view!", "ButtonAppearance::Secondary"],
      noPreBlocks: true,
    });

    const title = page.getByTestId("preview-page-title");
    await expect(title).toHaveCSS("display", "block");
    const fontSize = await title.evaluate((el) => getComputedStyle(el).fontSize);
    expect(parseFloat(fontSize)).toBeGreaterThanOrEqual(26);

    await expect(page.getByTestId("preview-examples")).toBeVisible();
    await expect(page.getByTestId("preview-examples").getByText("Default", { exact: true })).toBeVisible();
  });

  test("button: best practices and properties tabs render separately", async ({ page }) => {
    await openComponentPreview(page, "button");
    await expectPreviewPageTitle(page, "Button");
    await clickPreviewDocTab(page, "Best Practices");
    await expectPreviewDocContent(page, {
      contains: ["Do's", "Don'ts"],
    });

    await clickPreviewDocTab(page, "Properties");
    await expectPreviewDocContent(page, {
      contains: ["appearance"],
      noPreBlocks: true,
    });
    await clickPreviewDocTab(page, "Description");
    await expectPreviewDocContent(page, { noPreBlocks: true });
  });

  test("layout: usage steps without fenced code", async ({ page }) => {
    await openComponentPreview(page, "layout");
    await expectPreviewDocContent(page, {
      contains: ["overlay_header"],
      noPreBlocks: true,
    });
  });

  test("fixture-doc-panel: macro fixture content in doc tabs", async ({ page }) => {
    await openComponentPreview(page, "fixture-doc-panel", "fixture-doc-panel-preview");
    await expectPreviewPageTitle(page, "Fixture Doc Panel");
    await expectPreviewDocContent(page, {
      contains: ["Register this fixture", "Validating"],
      notContains: ["FixtureDocPanel label=\"hidden\""],
      noPreBlocks: true,
    });

    await clickPreviewDocTab(page, "Best Practices");
    await expectPreviewDocContent(page, {
      contains: ["Keep fixture doc strings stable"],
    });
  });

  test("doc tabs switch panel content", async ({ page }) => {
    await openComponentPreview(page, "button");
    await expectPreviewDocContent(page, { contains: ["main action"] });
    await clickPreviewDocTab(page, "Best Practices");
    await expectPreviewDocContent(page, { contains: ["Do's"] });
    await expectPreviewDocContent(page, { notContains: ["Toolbar and card actions"] });
  });

  test("M4 components: radio, auto-complete, date-picker doc tabs are non-empty", async ({ page }) => {
    test.setTimeout(120_000);
    for (const slug of ["radio", "auto-complete", "date-picker"] as const) {
      await openComponentPreview(page, slug);
      await expectPreviewDocContent(page, { noPreBlocks: true });
      await clickPreviewDocTab(page, "Best Practices");
      await expectPreviewDocContent(page, { noPreBlocks: true });
      await clickPreviewDocTab(page, "Properties");
      await expectPreviewDocContent(page, { noPreBlocks: true });
      await clickPreviewDocTab(page, "Description");
    }
  });

  test("fixture-doc-panel: example aside navigates to secondary example", async ({ page }) => {
    await openComponentPreview(page, "fixture-doc-panel", "fixture-doc-panel-preview");
    await expect(page.getByText("On this page", { exact: true })).toBeVisible();
    await expect(page.getByTestId("preview-example-nav")).toBeVisible();
    await expect(
      page.getByTestId("preview-example-nav").getByRole("link", { name: "Default example" }),
    ).toBeVisible();
    await expect(
      page.getByTestId("preview-example-nav").getByRole("link", { name: "Secondary example" }),
    ).toBeVisible();

    await page.getByTestId("preview-example-nav").getByRole("link", { name: "Secondary example" }).click();
    await expect(page.locator("#example-secondary-example")).toBeInViewport();
  });

  test("show code toggle reveals example source", async ({ page }) => {
    await openComponentPreview(page, "button");
    const examples = page.getByTestId("preview-examples");
    const firstExample = examples.locator("[id^='example-']").first();
    await expect(firstExample.locator(".orbital-material--outlined").first()).toBeVisible();
    await expect(firstExample.locator(".orbital-card-section-border").first()).toBeVisible();

    const showCode = firstExample.getByRole("button", { name: "Show code" });
    await expect(showCode).toHaveClass(/orbital-button--secondary/);
    await showCode.click();
    await expect(firstExample.getByRole("button", { name: "Hide code" })).toBeVisible();
    const codeBlock = firstExample.locator(".orbital-card-preview .orbital-code pre").first();
    await expect(codeBlock).toBeVisible();
    await expect(codeBlock).not.toBeEmpty();
    await expect(firstExample.locator(".orbital-card-section-border")).toHaveCount(2);
  });
});

test.describe("orbital shape defaults", () => {
  test("card surface uses 8px medium border radius", async ({ page }) => {
    await openComponentPreview(page, "card");
    const surface = page.getByTestId("card-preview").locator(".orbital-material").first();
    await expect(surface).toHaveCSS("border-radius", "8px");
  });

  test("button uses 8px medium border radius", async ({ page }) => {
    await openComponentPreview(page, "button");
    const button = page.getByTestId("button-preview").locator(".orbital-button").first();
    await expect(button).toHaveCSS("border-radius", "8px");
  });
});
