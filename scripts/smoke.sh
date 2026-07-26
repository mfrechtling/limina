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

./scripts/wait-for-http.sh http://localhost:8080/_limina/health 30

health_response=$(curl -s http://localhost:8080/_limina/health)

if [[ "$health_response" != "ok" ]]; then
    echo "Unexpected health response:"
    echo "$health_response"
    exit 1
fi

live_response=$(curl -s http://localhost:8080/_limina/health/live)

if [[ "$live_response" != "ok" ]]; then
    echo "Unexpected live response:"
    echo "$live_response"
    exit 1
fi

ready_response=$(curl -s http://localhost:8080/_limina/health/ready)

if [[ "$ready_response" != '{"ready":true}' ]]; then
    echo "Unexpected ready response:"
    echo "$ready_response"
    exit 1
fi

version_response=$(curl -s http://localhost:8080/_limina/version)

if [[ -z "$version_response" ]]; then
    echo "Unexpected version response:"
    echo "$version_response"
    exit 1
fi

if ! [[ "$version_response" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-.+][0-9A-Za-z.-]+)?$ ]]; then
    echo "Version is not semver-like:"
    echo "$version_response"
    exit 1
fi

metrics_response=$(curl -s http://localhost:8080/_limina/metrics)

if ! grep -q "limina_up 1" <<<"$metrics_response"; then
    echo "Metrics missing limina_up gauge"
    exit 1
fi

unknown_code=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/unknown)

if [[ "$unknown_code" != "404" ]]; then
    echo "Expected 404 for unknown route, got: $unknown_code"
    exit 1
fi

validate_wrong_method=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/_limina/config/validate)

if [[ "$validate_wrong_method" != "405" ]]; then
    echo "Expected 405 for GET /_limina/config/validate, got: $validate_wrong_method"
    exit 1
fi

reload_wrong_method=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/_limina/config/reload)

if [[ "$reload_wrong_method" != "405" ]]; then
    echo "Expected 405 for GET /_limina/config/reload, got: $reload_wrong_method"
    exit 1
fi

echo "Smoke test passed"