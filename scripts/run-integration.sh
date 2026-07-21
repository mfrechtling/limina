#!/usr/bin/env bash
set -euo pipefail

response=$(curl -s http://localhost:8080)

if [[ "$response" != "Hello limina" ]]; then
    echo "Unexpected reponse:"
    echo "$response"
    exit 1
fi

echo "Integration test passed"