#!/usr/bin/env node
/**
 * Assemble mdBook source under book/ from docs/ and the crate README.
 *
 * Committed book-only pages (SUMMARY.md, whoami.md, getting-started.md) are kept.
 * Canonical developer markdown lives in docs/ (rest/, grpc/, types/); this script
 * copies it into book/ so mdbook has a single src tree.
 *
 * Run from sdks/rs: `make docs-prepare` (also invoked by docs-build / docs-serve).
 */
import {
  cpSync,
  mkdirSync,
  rmSync,
  writeFileSync,
  readFileSync,
  existsSync,
  readdirSync,
  statSync,
} from 'node:fs';
import { dirname, join, resolve, extname, relative, basename } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const book = join(root, 'book');

/** Paths under book/ that are authored for mdBook and must not be wiped. */
const PRESERVE = new Set(['SUMMARY.md', 'whoami.md', 'getting-started.md']);

function walk(dir, out = []) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    if (statSync(p).isDirectory()) walk(p, out);
    else out.push(p);
  }
  return out;
}

function clearGenerated() {
  if (!existsSync(book)) {
    mkdirSync(book, { recursive: true });
    return;
  }
  for (const name of readdirSync(book)) {
    if (PRESERVE.has(name)) continue;
    rmSync(join(book, name), { recursive: true, force: true });
  }
}

clearGenerated();

// docs/ → book/ (README, rest/, grpc/, types/, cheatsheets, …)
for (const name of readdirSync(join(root, 'docs'))) {
  if (name === 'SUMMARY.md') continue;
  cpSync(join(root, 'docs', name), join(book, name), { recursive: true });
}

mkdirSync(join(book, 'crate'), { recursive: true });
cpSync(join(root, 'README.md'), join(book, 'crate', 'README.md'));

/** Relative href from a book chapter file to a target under book/. */
function hrefTo(fromFile, targetUnderBook) {
  const fromDir = dirname(fromFile);
  let rel = relative(fromDir, join(book, targetUnderBook)).replace(/\\/g, '/');
  if (!rel.startsWith('.')) rel = `./${rel}`;
  return rel;
}

/**
 * Rewrite in-repo relative parents so chapters resolve inside book/.
 * Source docs may link to ../README.md; those become paths under book/.
 */
function rewriteLinks(text, filePath) {
  const map = (target) => hrefTo(filePath, target);

  return text
    .replace(/\]\(\.\.\/docs\/README\.md\)/g, `](${map('README.md')})`)
    .replace(/\]\(\.\.\/docs\/?\)/g, `](${map('README.md')})`)
    .replace(/\]\(\.\/docs\/README\.md\)/g, `](${map('README.md')})`)
    .replace(/\]\(\.\/docs\/?\)/g, `](${map('README.md')})`)
    .replace(/\]\(\.\/docs\)/g, `](${map('README.md')})`)
    .replace(/\]\(docs\/README\.md\)/g, `](${map('README.md')})`)
    .replace(/\]\(docs\/?\)/g, `](${map('README.md')})`)
    .replace(/\]\(\.\.\/README\.md\)/g, `](${map('crate/README.md')})`)
    .replace(/\]\(\.\/README\.md\)/g, `](${map('crate/README.md')})`);
}

for (const file of walk(book)) {
  if (extname(file) !== '.md') continue;
  // Keep hand-authored book pages as-is (already use book-relative links).
  if (PRESERVE.has(basename(file)) && dirname(file) === book) continue;
  const before = readFileSync(file, 'utf8');
  const after = rewriteLinks(before, file);
  if (after !== before) writeFileSync(file, after);
}

console.log(`mdBook source prepared at ${relative(root, book) || 'book'}`);
