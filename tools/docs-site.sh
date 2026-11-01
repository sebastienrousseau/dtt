#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Build the API documentation site served at https://doc.dttlib.com.
#
# Runs rustdoc for this crate only (--no-deps), then adds what a search
# engine needs to index the current pages and forget old ones:
#
#   index.html   redirect to the crate page
#   robots.txt   keeps crawlers out of /src/ and search-result URLs
#   sitemap.xml  every current item page, nothing else
#   canonical    <link rel="canonical"> on every page the sitemap lists
#
# Then it checks the result and fails if any of that is missing.
#
# An earlier deployment documented the whole dependency tree, so Google
# still requests pages such as /src/syn/op.rs.html. robots.txt and the
# sitemap tell it which URLs exist now.
#
# Usage: tools/docs-site.sh [--check-only]
# Environment:
#   DOCS_BASE_URL  site origin (default https://doc.dttlib.com)
#   DOCS_OUT       rustdoc output directory (default target/doc)

set -euo pipefail

cd "$(dirname "$0")/.."

readonly CRATE="dtt"
readonly BASE_URL="${DOCS_BASE_URL:-https://doc.dttlib.com}"
readonly OUT="${DOCS_OUT:-target/doc}"

fail() {
  echo "docs-site: $*" >&2
  exit 1
}

# rustdoc writes a meta-refresh stub for each re-export or legacy path.
is_redirect_stub() {
  grep -q 'http-equiv="refresh"' "$1"
}

build_docs() {
  local flags="${RUSTDOCFLAGS:-}"
  # `--cfg docsrs` turns on `doc_cfg`, which only nightly accepts.
  if rustc --version | grep -q nightly; then
    flags="$flags --cfg docsrs"
  fi
  rm -rf "$OUT"
  RUSTDOCFLAGS="$flags" cargo doc --no-deps --all-features --locked
}

write_index() {
  cat >"$OUT/index.html" <<EOF
<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>${CRATE}</title><meta http-equiv="refresh" content="0; url=${CRATE}/index.html"><link rel="canonical" href="${BASE_URL}/${CRATE}/index.html"></head><body><a href="${CRATE}/index.html">${CRATE}</a></body></html>
EOF
}

write_robots() {
  cat >"$OUT/robots.txt" <<EOF
User-agent: *
Disallow: /src/
Disallow: /*?search=
Disallow: /help.html
Disallow: /settings.html

Sitemap: ${BASE_URL}/sitemap.xml
EOF
}

# Item pages under the crate directory, minus redirect stubs, as
# "page<TAB>canonical" lines. rustdoc inlines a re-export as a second,
# identical page (dtt/struct.DateTime.html and
# dtt/datetime/struct.DateTime.html); both link to the same source
# lines, so pages are grouped by that link and the shortest path in each
# group is canonical. A page with no source link is its own group.
# Sorted so the output is reproducible.
list_pages() {
  local page key
  (cd "$OUT" && find "$CRATE" -name '*.html' | LC_ALL=C sort) |
    while IFS= read -r page; do
      is_redirect_stub "$OUT/$page" && continue
      key=$(grep -o 'class="src" href="[^"]*"' "$OUT/$page" | head -n 1 |
        sed 's:.*\(src/[^"]*\)":\1:' || true)
      printf '%s\t%s\t%s\n' "${key:-$page}" "${page//[^\/]/}" "$page"
    done |
    LC_ALL=C sort -t "$(printf '\t')" -k1,1 -k2,2 -k3,3 |
    awk -F '\t' '$1 != key { key = $1; canon = $3 } { print $3 "\t" canon }' |
    LC_ALL=C sort
}

add_canonical() {
  local page="$1" url="$2"
  grep -q 'rel="canonical"' "$OUT/$page" && return 0
  URL="$url" perl -0pi -e \
    's{<head>}{<head><link rel="canonical" href="$ENV{URL}">}' "$OUT/$page"
}

# Every page gets a canonical link; only canonical pages go in the sitemap.
write_sitemap() {
  local page canon
  {
    echo '<?xml version="1.0" encoding="UTF-8"?>'
    echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">'
    while IFS=$'\t' read -r page canon; do
      add_canonical "$page" "$BASE_URL/$canon"
      if [ "$page" = "$canon" ]; then
        printf '  <url><loc>%s</loc></url>\n' "$BASE_URL/$page"
      fi
    done < <(list_pages)
    echo '</urlset>'
  } >"$OUT/sitemap.xml"
}

check_robots() {
  grep -qx 'Disallow: /src/' "$OUT/robots.txt" || fail "robots.txt lacks /src/"
  grep -qx 'Disallow: /\*?search=' "$OUT/robots.txt" ||
    fail "robots.txt lacks the search rule"
  grep -qx "Sitemap: $BASE_URL/sitemap.xml" "$OUT/robots.txt" ||
    fail "robots.txt lacks the Sitemap line"
}

# Each sitemap URL is an existing page under /$CRATE/ whose canonical
# link names that same URL.
check_sitemap() {
  local loc page
  while IFS= read -r loc; do
    page="${loc#"$BASE_URL"/}"
    case "$page" in "$CRATE"/*) ;; *) fail "sitemap URL outside /$CRATE/: $loc" ;; esac
    [ -f "$OUT/$page" ] || fail "sitemap URL has no file: $loc"
    grep -qF "<link rel=\"canonical\" href=\"$loc\">" "$OUT/$page" ||
      fail "no matching canonical in $page"
  done < <(sed -n 's:.*<loc>\(.*\)</loc>.*:\1:p' "$OUT/sitemap.xml")
}

# Every item page, in the sitemap or not, names a canonical URL.
check_canonicals() {
  local page canon
  while IFS=$'\t' read -r page canon; do
    grep -qF "<link rel=\"canonical\" href=\"$BASE_URL/$canon\">" \
      "$OUT/$page" || fail "$page lacks canonical $canon"
  done < <(list_pages)
}

check_site() {
  local count
  [ -f "$OUT/$CRATE/index.html" ] || fail "missing $OUT/$CRATE/index.html"
  # --no-deps: the only crate documented, and the only source tree, is ours.
  [ "$(ls "$OUT/src")" = "$CRATE" ] || fail "src/ holds other crates"
  check_robots
  count=$(grep -c '<loc>' "$OUT/sitemap.xml" || true)
  [ "$count" -gt 0 ] || fail "sitemap.xml is empty"
  check_sitemap
  check_canonicals
  echo "docs-site: ok, $count pages in sitemap.xml"
}

main() {
  if [ "${1:-}" != "--check-only" ]; then
    build_docs
    write_index
    write_robots
    write_sitemap
  fi
  check_site
}

main "$@"
