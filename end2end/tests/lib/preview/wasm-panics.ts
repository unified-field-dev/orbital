import type { Page } from "@playwright/test";
import { expect } from "@playwright/test";

const PANIC_PATTERNS = ["panicked", "unreachable", "already been disposed"];

function isWasmPanic(text: string): boolean {
  return PANIC_PATTERNS.some((pattern) => text.includes(pattern));
}

/** Collect WASM panic messages from console and page errors during a test. */
export class WasmPanicGuard {
  private readonly panics: string[] = [];

  constructor(page: Page) {
    page.on("console", (msg) => {
      if (msg.type() === "error" && isWasmPanic(msg.text())) {
        this.panics.push(msg.text());
      }
    });
    page.on("pageerror", (err) => {
      if (isWasmPanic(err.message)) {
        this.panics.push(err.message);
      }
    });
  }

  assertClean() {
    expect(this.panics, `WASM panics detected:\n${this.panics.join("\n")}`).toEqual([]);
  }
}
