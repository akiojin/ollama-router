#!/usr/bin/env bash
set -euo pipefail

TARGET="${WRK_TARGET:-http://localhost:8080}"
ENDPOINT="${WRK_ENDPOINT:-/v1/chat/completions}"

if ! command -v wrk >/dev/null 2>&1; then
  echo "wrk not found. Install wrk and retry." >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PAYLOAD_SCRIPT="${WRK_SCRIPT:-${SCRIPT_DIR}/chat.lua}"

echo "Running wrk against ${TARGET}${ENDPOINT}"
echo "Using script: ${PAYLOAD_SCRIPT}"

wrk "$@" -s "${PAYLOAD_SCRIPT}" "${TARGET}${ENDPOINT}"
