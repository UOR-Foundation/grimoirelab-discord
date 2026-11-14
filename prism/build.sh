#!/usr/bin/env bash
# Regenerate the app from app/prompt.json with the builder that
# prism/builder.lock.json pins, then regenerate CONFORMANCE.md.
#
#   prism/build.sh          write the generated files into this checkout
#   prism/build.sh --check  write nothing; fail if any generated file differs
#
# The builder is fetched at the pinned commit, and git re-derives the commit id
# on fetch, so any other builder is refused. The builder was written against an
# earlier template than the one this repository was created from. It writes
# model/ids.toml whole, so the rows of the template's own suites named in
# keep_suites (the frame) are carried over from the committed register instead
# of being dropped.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
lock="$root/prism/builder.lock.json"
field() { node -e 'const l=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));const v=l[process.argv[2]];process.stdout.write(Array.isArray(v)?v.join(" "):v)' "$lock" "$1"; }
repo="$(field repository)"
commit="$(field commit)"
keep="$(field keep_suites)"
cache="${PRISM_BUILDER_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/prism-builder/$commit}"

if [ "$(git -C "$cache" rev-parse HEAD 2>/dev/null || true)" != "$commit" ]; then
  rm -rf "$cache" && git init -q "$cache"
  git -C "$cache" fetch -q --depth 1 "https://github.com/$repo" "$commit"
  git -C "$cache" checkout -q --detach FETCH_HEAD
  if [ "$(git -C "$cache" rev-parse HEAD)" != "$commit" ]; then
    echo "the builder is not at the pinned $commit" >&2
    exit 3
  fi
fi

check=false
[ "${1:-}" = "--check" ] && check=true
target="$root"
if $check; then
  target="$(mktemp -d)"
  trap 'rm -rf "$target"' EXIT
  git -C "$root" ls-files -z | (cd "$root" && xargs -0 cp --parents -t "$target")
fi

kept="$(mktemp)"
node - "$root/model/ids.toml" "$keep" >"$kept" <<'JS'
const [file, keep] = process.argv.slice(2);
const suites = new Set(keep.split(' ').filter(Boolean));
const blocks = require('fs').readFileSync(file, 'utf8').split(/\n(?=\[\[id\]\])/).filter((b) => b.startsWith('[[id]]'));
for (const b of blocks) {
  const suite = (b.match(/^suite = "([^"]+)"/m) || [])[1];
  if (suites.has(suite)) process.stdout.write(`\n${b.trim()}\n`);
}
JS

prompt="$(mktemp)"
cp "$root/app/prompt.json" "$prompt"
# The page's last-modified instant is the prompt's last commit, so a rebuild of
# the same commit writes the same bytes.
seconds="$(git -C "$root" log -1 --format=%ct -- app/prompt.json)"
instant="$(node -e 'process.stdout.write(new Date(Number(process.argv[1] || Date.now() / 1000) * 1000).toISOString().replace(/\.\d{3}Z$/, "Z"))' "$seconds")"
(cd "$cache" && node builder/cli.mjs build "$prompt" --out "$target" --last-modified "$instant") >/dev/null
cat "$kept" >>"$target/model/ids.toml"
rm -f "$kept" "$prompt"
(cd "$target" && cargo run -q -p xtask -- check-model --write >/dev/null)

if $check; then
  differ="$(diff -r -q -x .git -x target -x .prism -x node_modules "$root" "$target" | grep -v "^Only in $root" || true)"
  if [ -n "$differ" ]; then
    printf '%s\n' "$differ" >&2
    echo "generated files differ from the build of app/prompt.json; run prism/build.sh" >&2
    exit 1
  fi
  echo "every generated file equals the build of app/prompt.json"
fi
