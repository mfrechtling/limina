#!/usr/bin/env bash
set -euo pipefail

url=$1
timeout=$2
start_time=$(date +%s)
while ! curl -s "$url" >/dev/null; do
    sleep 1
    current_time=$(date +%s)
    if (( current_time - start_time >= timeout )); then
        echo "Timed out waiting for $url"
        exit 1
    fi
done