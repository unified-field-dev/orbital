import type { Locator } from "@playwright/test";
import { expect } from "@playwright/test";

/** Select an option on an Orbital Select wrapper (`span.orbital-select` + native `<select>`). */
export async function selectPreviewOption(wrapper: Locator, value: string) {
  const select = wrapper.locator("select");
  await expect(async () => {
    await select.selectOption(value);
    await expect(select).toHaveValue(value);
  }).toPass({ timeout: 5_000 });
}

/** Fill an Orbital Input wrapper (`span.orbital-input` + native `<input>`). */
export async function fillPreviewInput(wrapper: Locator, value: string) {
  await wrapper.locator("input").fill(value);
}

/** Blur an Orbital Input wrapper. */
export async function blurPreviewInput(wrapper: Locator) {
  await wrapper.locator("input").blur();
}
