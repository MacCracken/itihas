#!/bin/sh
# itihas — Cyrius test runner
# Equivalent to: cargo test --all-features
CC="${1:-./build/cc3}"
set -e

echo "=== Building itihas ==="
cat src/main.cyr | "$CC" > build/itihas && chmod +x build/itihas
echo "  build: OK ($(wc -c < build/itihas) bytes)"

echo ""
echo "=== Running test suite ==="
./build/itihas
echo ""
echo "=== Done ==="
