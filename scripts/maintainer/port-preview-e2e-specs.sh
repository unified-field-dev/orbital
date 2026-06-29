#!/usr/bin/env bash
# Port component preview Playwright specs from an external source tree into this repo.
# Requires SOURCE — absolute path to a repo root containing end2end/tests/orbital/*.spec.ts
set -euo pipefail

if [[ -z "${SOURCE:-}" ]]; then
  echo "SOURCE is required: repo root with end2end/tests/orbital preview specs" >&2
  exit 1
fi

DEST="${DEST:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../end2end/tests/components" && pwd)}"
SRC="$SOURCE/end2end/tests/orbital"

if [[ ! -d "$SRC" ]]; then
  echo "Expected specs at: $SRC" >&2
  exit 1
fi

mkdir -p "$DEST"

EXCLUDE=(
  auth.spec.ts
  notifications-subscription.spec.ts
  topbar-third-layout.spec.ts
)

should_exclude() {
  local base="$1"
  for ex in "${EXCLUDE[@]}"; do
    [[ "$base" == "$ex" ]] && return 0
  done
  return 1
}

for src in "$SRC"/*.spec.ts; do
  base=$(basename "$src")
  should_exclude "$base" && continue
  dest="$DEST/$base"

  python3 - "$src" "$dest" <<'PY'
import re, sys
src_path, dest_path = sys.argv[1:3]
text = open(src_path, encoding="utf-8").read()

text = re.sub(
    r'import \{ test, expect \} from "\.\./fixtures";',
    'import { test, expect } from "@playwright/test";',
    text,
)
text = re.sub(r'import \{ seedTestData[^"]*"\.\./helpers";?\n', "", text)
text = re.sub(
    r'import \{ openPrimitivePreview, expectPrimitiveVariants \} from "\./_primitive_preview";',
    'import { openComponentPreview, expectPreviewVariants } from "../_preview";',
    text,
)
text = re.sub(r"openPrimitivePreview", "openComponentPreview", text)
text = re.sub(r"expectPrimitiveVariants", "expectPreviewVariants", text)

# Remove entire beforeEach blocks (auth + preview open)
text = re.sub(
    r"\n\s*test\.beforeEach\(async \(\{[^}]*\}\) => \{.*?\n\s*\}\);\n",
    "\n",
    text,
    flags=re.S,
)

# Remove orphaned auth/seed fragments
text = re.sub(
    r"\n\s*await seedTestData\(.*?\);\n",
    "\n",
    text,
    flags=re.S,
)
text = re.sub(
    r"\n\s*await auth\.signIn\(\{.*?\}\);\n",
    "\n",
    text,
    flags=re.S,
)

# Remove stray top-level await openComponentPreview after describe open
text = re.sub(
    r'(test\.describe\([^\)]+\) \{\n)\s*await openComponentPreview\(page, "[^"]+"\);\n\s*\}\);\n',
    r"\1",
    text,
)

slug_match = re.search(r'openComponentPreview\(page, "([^"]+)"\)', text)
slug = slug_match.group(1) if slug_match else None

if slug:
    def inject_preview(match):
        body = match.group(2)
        if "openComponentPreview" in body:
            return match.group(0)
        return f'{match.group(1)}{{\n    await openComponentPreview(page, "{slug}");\n{body}}}'

    text = re.sub(
        r'(test\("[^"]+", async \(\{ page \}\) => )\{',
        lambda m: f'{m.group(1)}{{\n    await openComponentPreview(page, "{slug}");',
        text,
    )

if "COMPONENT_PREVIEW_E2E" not in text:
    text = re.sub(
        r"(test\.describe\([^\)]+\) \{\n)",
        r'\1  test.skip(!process.env.COMPONENT_PREVIEW_E2E, "Set COMPONENT_PREVIEW_E2E=1 with preview server on :3010");\n\n',
        text,
        count=1,
    )

open(dest_path, "w", encoding="utf-8").write(text)
PY
done

echo "Ported specs to $DEST"
