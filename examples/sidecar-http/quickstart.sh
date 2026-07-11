#!/usr/bin/env bash
# Quick smoke test for the sidecar-http example (Linux / macOS).
set -euo pipefail

BUILD=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --build|-b) BUILD=1; shift ;;
    -h|--help)
      echo "Usage: $0 [--build]"
      echo "  --build   cargo build --release --bin weft-core before testing"
      exit 0
      ;;
    *) echo "Unknown option: $1" >&2; exit 1 ;;
  esac
done

EXAMPLE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$EXAMPLE_DIR/../.." && pwd)"
BINARY="$REPO_ROOT/target/release/weft-core"
CONFIG_DIR="$EXAMPLE_DIR"
DATA_DIR="$EXAMPLE_DIR/data"
CONFIG_FILE="$EXAMPLE_DIR/config.toml"
CONFIG_EXAMPLE="$EXAMPLE_DIR/config.example.toml"
PORT=17830
BASE_URL="http://127.0.0.1:${PORT}"
TOKEN_PATH="$DATA_DIR/runtime-token"
STARTED_PID=""

cleanup() {
  if [[ -n "$STARTED_PID" ]] && kill -0 "$STARTED_PID" 2>/dev/null; then
    kill "$STARTED_PID" 2>/dev/null || true
    wait "$STARTED_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

if [[ ! -f "$CONFIG_FILE" ]]; then
  cp "$CONFIG_EXAMPLE" "$CONFIG_FILE"
  echo "Created $CONFIG_FILE — edit [[providers.keys]] before chat completions work."
fi

if [[ "$BUILD" -eq 1 ]]; then
  echo "Building weft-core..."
  (cd "$REPO_ROOT" && cargo build --release -p weft-core --bin weft-core)
fi

if [[ ! -x "$BINARY" ]]; then
  echo "weft-core binary not found at: $BINARY" >&2
  echo "Run: $0 --build   or: cargo build --release -p weft-core --bin weft-core" >&2
  exit 1
fi

health_ok() {
  curl -sf "$BASE_URL/api/health" >/dev/null 2>&1
}

if ! health_ok; then
  echo "Starting weft-core (background)..."
  mkdir -p "$DATA_DIR"
  (
    cd "$REPO_ROOT"
    exec "$BINARY" \
      --config-dir "$CONFIG_DIR" \
      --data-dir "$DATA_DIR"
  ) &
  STARTED_PID=$!

  for _ in $(seq 1 30); do
    if health_ok; then
      break
    fi
    sleep 1
  done
fi

echo
echo "=== Health ==="
curl -s "$BASE_URL/api/health"
echo
echo

echo "=== Runtime token ==="
echo "Path: $TOKEN_PATH"
if [[ ! -f "$TOKEN_PATH" ]]; then
  echo "Token file not found yet. Wait for weft-core to finish starting." >&2
  exit 1
fi
TOKEN="$(tr -d '\r\n' < "$TOKEN_PATH")"
echo "Token: ${TOKEN:0:8}... (truncated)"
echo

echo "=== Chat completion (requires a valid provider key in config.toml) ==="
cat <<EOF
curl -s "$BASE_URL/v1/chat/completions" \\
  -H "Authorization: Bearer $TOKEN" \\
  -H "Content-Type: application/json" \\
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "Say hello in one sentence."}]
  }'
EOF
echo
