#!/usr/bin/env bash
# IP release gate — fail if tracked release surface contains upstream fingerprints.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

fail=0

echo "== IP fingerprint grep (tracked release surface) =="
if rg -i 'fluent|mui|lemmy|@mui|@fluentui|mui\.com|fluentui\.com|canonical_fluent|DataGrid|Mui[A-Z]' \
  --glob '!docs/internal/**' \
  --glob '!vendor/**' \
  --glob '!target/**' \
  --glob '!target-site-preview/**' \
  --glob '!scripts/maintainer/ip_release_gate.sh' \
  --glob '!end2end/node_modules/**' \
  --glob '!end2end/playwright-report/**' \
  --glob '!end2end/test-results/**' \
  --glob '!end2end/blob-report/**' \
  README.md docs/ orbital-*/src/ orbital/ end2end/tests/ scripts/ 2>/dev/null; then
  echo "FAIL: upstream fingerprint matches above"
  fail=1
else
  echo "PASS: no upstream fingerprint matches"
fi

echo ""
echo "== Legacy Fluent CSS vars in shipped source =="
if rg 'var\(--color[A-Z]|--colorBrand|--strokeWidth' orbital-*/src/ 2>/dev/null; then
  echo "FAIL: legacy CSS token vars above"
  fail=1
else
  echo "PASS: no legacy CSS token vars"
fi

echo ""
echo "== Tracked research / design artifacts =="
if git ls-files '**/docs/research/**' '**/docs/design.md' 'orbital/COMPONENT_REGISTRY.md' 2>/dev/null | grep -q .; then
  echo "FAIL: tracked paths that should be internal-only:"
  git ls-files '**/docs/research/**' '**/docs/design.md' 'orbital/COMPONENT_REGISTRY.md'
  fail=1
else
  echo "PASS: no tracked research/design/registry artifacts"
fi

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi

echo ""
echo "IP release gate passed."
