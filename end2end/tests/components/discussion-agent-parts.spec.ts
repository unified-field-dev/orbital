import { test, expect } from "@playwright/test";
import { openComponentPreview } from "../lib/preview/navigation";
test.describe("discussion agent parts preview", () => {
  test("approve button fires on_tool_approval callback", async ({ page }) => {
    await openComponentPreview(page, "discussion-agent-parts");
    const preview = page.getByTestId("discussion-agent-parts-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await preview.getByTestId("discussion-tool-approve").getByRole("button", { name: "Approve" }).click();

    await expect(page.getByTestId("discussion-agent-parts-log")).toHaveText(
      /approval:tc-search-docs:approved=true/,
    );
  });

  test("streaming cursor visible on streaming text part", async ({ page }) => {
    await openComponentPreview(page, "discussion-agent-parts");
    const preview = page.getByTestId("discussion-agent-parts-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    await expect(preview.getByTestId("discussion-streaming-cursor")).toBeVisible();
  });

  test("composer disabled while thread has Streaming reply", async ({ page }) => {
    await openComponentPreview(page, "discussion-agent-parts");
    const preview = page.getByTestId("discussion-agent-parts-preview");
    await expect(preview).toBeVisible({ timeout: 30_000 });

    const textarea = preview.locator(".orbital-discussion__composer-textarea textarea");
    await expect(textarea).toBeDisabled();

    const sendButton = preview.locator('[data-testid="discussion-composer-send"] button');
    await expect(sendButton).toBeDisabled();
  });
});
