#!/usr/bin/env bash
# Prepare book/ from docs/, then mdbook build → site/
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
node scripts/prepare-book.mjs
mdbook build
