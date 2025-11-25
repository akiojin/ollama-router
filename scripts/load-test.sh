#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:-http://127.0.0.1:11435/health}"
DURATION="${DURATION:-10s}"
CONNECTIONS="${CONNECTIONS:-64}"
THREADS="${THREADS:-4}"

if ! command -v wrk >/dev/null 2>&1; then
  echo "wrk is required. Install: brew install wrk (mac) / apt-get install wrk (linux)" >&2
  exit 1
fi

echo "Running wrk against ${TARGET}"
wrk -t"${THREADS}" -c"${CONNECTIONS}" -d"${DURATION}" "${TARGET}"
