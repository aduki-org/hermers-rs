#!/usr/bin/env bash
# Prepare book/ from docs/, then mdbook serve (http://localhost:3000)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
node scripts/prepare-book.mjs
mdbook serve --open
