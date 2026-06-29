import type { Page } from "@playwright/test";
import { previewUrl, waitForPreviewShell } from "./helpers";

export type ReactiveWarningSite = {
  location: string;
  definedAt?: string;
  raw: string;
};

export type ReactiveWarningReport = {
  slug: string;
  route: "bare" | "full";
  warnings: ReactiveWarningSite[];
};

const WARNING_LOCATION_RE = /At ([^,]+), you access/;
const DEFINED_AT_RE = /defined at ([^\)]+)\)/;

export function parseReactiveWarning(text: string): ReactiveWarningSite | null {
  if (!text.includes("outside a reactive tracking context")) {
    return null;
  }
  const locationMatch = text.match(WARNING_LOCATION_RE);
  if (!locationMatch) {
    return null;
  }
  const definedMatch = text.match(DEFINED_AT_RE);
  return {
    location: locationMatch[1].trim(),
    definedAt: definedMatch?.[1]?.trim(),
    raw: text,
  };
}

/** Orbital workspace sources we treat as actionable client-side violations. */
export function isOrbitalOwnedLocation(location: string): boolean {
  if (location.includes(".cargo/registry")) {
    return false;
  }
  return (
    location.includes("/orbital/") ||
    location.startsWith("orbital/") ||
    location.includes("/orbital-") ||
    location.startsWith("orbital-")
  );
}

/** Known SSR-only false positives when reactive_graph warnings are enabled (patch removed). */
export const KNOWN_SSR_FALSE_POSITIVES: readonly string[] = [
  "orbital-base-components/src/flex/base.rs:83",
  "orbital-core-components/src/preview/preview_view.rs",
  "orbital-preview-app/src/preview/slug_page.rs",
];

export function isKnownSsrFalsePositive(location: string): boolean {
  return KNOWN_SSR_FALSE_POSITIVES.some((site) => location.includes(site));
}

export async function collectPreviewSlugs(page: Page): Promise<string[]> {
  await page.goto(previewUrl("/"));
  await waitForPreviewShell(page);
  await page.getByTestId("preview-catalog-nav").waitFor({ state: "visible" });

  const hrefs = await page.locator('[data-testid="preview-catalog-nav"] a[href]').evaluateAll(
    (anchors) =>
      anchors
        .map((a) => a.getAttribute("href") ?? "")
        .filter((href) => href.startsWith("/") && href !== "/" && !href.startsWith("/debug")),
  );

  let slugs = [
    ...new Set(
      hrefs.map((href) => href.replace(/^\//, "").replace(/\/$/, "")).filter(Boolean),
    ),
  ];

  if (slugs.length === 0) {
    const itemIds = await page
      .locator('[data-testid="preview-catalog-nav"] [data-item-id]')
      .evaluateAll((nodes) =>
        nodes
          .map((node) => node.getAttribute("data-item-id") ?? "")
          .filter((id) => id.length > 0 && id !== "introduction"),
      );
    slugs = [...new Set(itemIds)];
  }

  slugs.sort();
  return slugs;
}

export async function collectReactiveWarningsForRoute(
  page: Page,
  url: string,
  waitForTestId: string,
): Promise<ReactiveWarningSite[]> {
  const seen = new Set<string>();
  const warnings: ReactiveWarningSite[] = [];

  const onConsole = (msg: { text: () => string }) => {
    const parsed = parseReactiveWarning(msg.text());
    if (!parsed) {
      return;
    }
    const key = `${parsed.location}|${parsed.definedAt ?? ""}`;
    if (seen.has(key)) {
      return;
    }
    seen.add(key);
    warnings.push(parsed);
  };

  page.on("console", onConsole);
  try {
    let lastError: unknown;
    for (let attempt = 0; attempt < 3; attempt += 1) {
      try {
        await page.goto(url, { waitUntil: "domcontentloaded" });
        lastError = undefined;
        break;
      } catch (error) {
        lastError = error;
        await page.waitForTimeout(500 * (attempt + 1));
      }
    }
    if (lastError) {
      throw lastError;
    }
    await page.getByTestId(waitForTestId).first().waitFor({ state: "attached", timeout: 15_000 });
    // Allow hydration/render effects to flush console warnings.
    await page.waitForTimeout(100);
    if (waitForTestId === "debug-bare-root") {
      await page.evaluate(() => window.scrollTo(0, 400));
      await page.waitForTimeout(100);
    }
  } finally {
    page.off("console", onConsole);
  }

  return warnings;
}

export async function openBareDebugPreview(page: Page, slug: string) {
  await page.goto(previewUrl(`/debug/${slug}`));
  await page.waitForLoadState("domcontentloaded");
  await page.getByTestId("debug-bare-root").waitFor({ state: "attached", timeout: 15_000 });
}
