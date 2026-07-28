# Hermers Rust SDK — developer docs (mdBook)
#
#   make docs-prepare   # sync docs/ + crate README → book/
#   make docs-build     # prepare + mdbook build → site/
#   make docs-serve     # prepare + mdbook serve (http://localhost:3000)
#   make docs-check     # relative links + forbidden-string scan
#   make docs           # build + check

.PHONY: docs-prepare docs-build docs-serve docs-check docs

docs-prepare:
	node scripts/prepare-book.mjs

docs-build: docs-prepare
	mdbook build

docs-serve: docs-prepare
	mdbook serve --open

docs-check:
	node scripts/check-docs.mjs

docs: docs-build docs-check
