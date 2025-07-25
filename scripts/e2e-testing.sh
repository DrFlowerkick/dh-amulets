#!/usr/bin/env bash
set -e

# Determine container engine (docker or podman fallback)
ENGINE=${CONTAINER_ENGINE:-docker}

# Leptos environment
ENVIRONMENT=${LEPTOS_ENV:-DEV}

# Use same test directory regardless of CWD
TEST_DIR="$(pwd)"

# Playwright Docker image (used in DEV only)
IMAGE="mcr.microsoft.com/playwright:v1.44.1-jammy"

echo "Detected LEPTOS_ENV=$ENVIRONMENT"

# 🧹 Define cleanup (optional here, since we skip starting the server)
cleanup() {
  echo ""
  echo "🧼 Cleanup complete (no server to stop)."
  exit
}

trap cleanup SIGINT

if [[ "$ENVIRONMENT" == "DEV" ]]; then
  echo "🔧 Running Playwright E2E tests via Docker (local DEV mode)..."
  $ENGINE run --rm -it \
    --network=host \
    -v "$TEST_DIR":/app \
    -w /app \
    "$IMAGE" \
    bash -c "npx playwright test"

elif [[ "$ENVIRONMENT" == "PROD" ]]; then
  echo "⚙️  Running Playwright E2E tests directly (CI mode)..."
  echo "🔍 Reporter: HTML, no interactive mode"

  # Run Playwright tests with HTML report only
  npx playwright test --reporter=html || {
    echo "❌ Playwright tests failed."
    cleanup
    exit 1
  }

  echo "✅ Playwright tests finished (CI)"
else
  echo "❗️Unknown LEPTOS_ENV value: $ENVIRONMENT"
  cleanup
  exit 1
fi
