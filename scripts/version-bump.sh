#!/usr/bin/env bash
set -euo pipefail
[ $# -ne 1 ] && echo "Usage: $0 <version>" && exit 1
NEW_VERSION="$1"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "$NEW_VERSION" > "$REPO_ROOT/VERSION"
# cyrius.cyml derives its version from VERSION via `${file:VERSION}` — no manifest edit needed.
echo "Bumped to ${NEW_VERSION}. Add a CHANGELOG entry, then tag and push."
