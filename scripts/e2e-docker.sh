#!/usr/bin/env bash
set -e

# docker engine
ENGINE=${CONTAINER_ENGINE:-docker}

# take docker image which fits to package.json!
IMAGE="mcr.microsoft.com/playwright:v1.44.1-jammy"

# "cargo leptos end2end" is changing current dir to end2end. with pwd we are always save
TEST_DIR="$(pwd)"

# run docker to test
$ENGINE run --rm -it \
  --network=host \
  -v "$TEST_DIR":/app \
  -w /app \
  "$IMAGE" \
  bash -c "npx playwright test"
