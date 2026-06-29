#!/usr/bin/env bash
# Kill any process listening on TCP port 3010 (Orbital preview / leptos watch).
set -euo pipefail

PORT=3010

find_pids() {
  if command -v lsof >/dev/null 2>&1; then
    lsof -ti "tcp:${PORT}" -sTCP:LISTEN 2>/dev/null || true
    return
  fi

  if command -v ss >/dev/null 2>&1; then
    ss -lptn "sport = :${PORT}" 2>/dev/null \
      | sed -n 's/.*pid=\([0-9]*\).*/\1/p' \
      | sort -u
    return
  fi

  if command -v fuser >/dev/null 2>&1; then
    fuser "${PORT}/tcp" 2>/dev/null | tr ' ' '\n' | grep -E '^[0-9]+$' || true
    return
  fi

  echo "error: need lsof, ss, or fuser to find processes on port ${PORT}" >&2
  exit 1
}

PIDS=$(find_pids | tr '\n' ' ' | xargs echo -n 2>/dev/null || true)
PIDS=${PIDS:-}

if [ -z "$PIDS" ]; then
  echo "No process listening on port ${PORT}."
  exit 0
fi

echo "Sending SIGTERM to port ${PORT} listener(s): ${PIDS}"
# shellcheck disable=SC2086
kill ${PIDS} 2>/dev/null || true

sleep 0.5

REMAINING=$(find_pids | tr '\n' ' ' | xargs echo -n 2>/dev/null || true)
REMAINING=${REMAINING:-}

if [ -n "$REMAINING" ]; then
  echo "Sending SIGKILL to: ${REMAINING}"
  # shellcheck disable=SC2086
  kill -9 ${REMAINING} 2>/dev/null || true
fi

echo "Port ${PORT} is free."
