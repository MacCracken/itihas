#!/bin/sh
CC="${1:-./build/cc3}"
echo "=== itihas tests ==="
cat src/main.cyr | "$CC" > /tmp/itihas_test && chmod +x /tmp/itihas_test && /tmp/itihas_test
echo "exit: $?"
rm -f /tmp/itihas_test
