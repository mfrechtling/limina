#!/usr/bin/env bash
set -euo pipefail

docker run -d \
    --rm \
    --name limina-smoke \
    -p 8080:8080 \
    limina:ci

cleanup() {
    docker stop limina-smoke >/dev/null 2>&1 || true
}

trap cleanup EXIT

./scripts/wait-for-http.sh http://localhost:8080 30

response=$(curl -s http://localhost:8080)

test "$response" = "Hello Limina"