#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifest="$repo_root/codegen/generation-manifest.toml"
spec="$repo_root/codegen/spec/twilio_api_v2010.json"
config="$repo_root/codegen/generator-config.yaml"
templates="$repo_root/codegen/templates/rust"
jar="$repo_root/codegen/.cache/openapi-generator-cli-7.25.0.jar"
generated="$repo_root/crates/api-generated"
generated_inventory="$repo_root/codegen/generated-files.txt"
mode=${1:---check}

case "$mode" in
  --check|--update) ;;
  *) echo "usage: $0 [--check|--update]" >&2; exit 2 ;;
esac

value() {
  local section=$1 key=$2
  awk -v section="[$section]" -v key="$key" '
    $0 == section { active=1; next }
    /^\[/ { active=0 }
    active && $1 == key { gsub(/["[:space:]]/, "", $3); print $3; exit }
  ' "$manifest"
}

verify_hash() {
  local path=$1 expected=$2 actual
  actual=$(sha256sum "$path" | cut -d' ' -f1)
  if [[ "$actual" != "$expected" ]]; then
    echo "checksum mismatch: ${path#"$repo_root/"}" >&2
    echo "expected $expected" >&2
    echo "actual   $actual" >&2
    exit 1
  fi
}

verify_hash "$spec" "$(value specification sha256)"
verify_hash "$config" "$(value configuration sha256)"
verify_hash "$generated_inventory" "$(value outputs inventory_sha256)"
mapfile -t generated_paths < <(sed '/^[[:space:]]*$/d' "$generated_inventory")
for path in "${generated_paths[@]}"; do
  if [[ "$path" = /* || "$path" == *..* ]]; then
    echo "unsafe generated inventory path: $path" >&2
    exit 1
  fi
done
template_hash=$(
  cd "$templates"
  find . -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | cut -d' ' -f1
)
if [[ "$template_hash" != "$(value templates tree_sha256)" ]]; then
  echo "checksum mismatch: codegen/templates/rust" >&2
  exit 1
fi

mkdir -p "$(dirname "$jar")"
if [[ ! -f "$jar" ]]; then
  curl -fsSL "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/7.25.0/openapi-generator-cli-7.25.0.jar" -o "$jar.download"
  mv "$jar.download" "$jar"
fi
verify_hash "$jar" "$(value generator artifact_sha256)"

tmp=$(mktemp -d "${TMPDIR:-/tmp}/dialkit-generate.XXXXXX")
trap 'rm -rf "$tmp"' EXIT
export TZ=UTC LC_ALL=C LANG=C

java -jar "$jar" validate -i "$spec"
java -jar "$jar" generate \
  -g rust \
  -i "$spec" \
  -o "$tmp" \
  -c "$config" \
  -t "$templates"

"$repo_root/codegen/normalize-generated.sh" "$tmp"
rustfmt --edition 2024 "$tmp"/src/lib.rs "$tmp"/src/models/*.rs "$tmp"/src/apis/*.rs
# Some generator constructs (notably array-valued form parameters) only have a
# stable multiline shape after rustfmt. Normalize that shape, then format once
# more so the checked-in output remains canonical.
"$repo_root/codegen/normalize-generated.sh" "$tmp"
rustfmt --edition 2024 "$tmp"/src/lib.rs "$tmp"/src/models/*.rs "$tmp"/src/apis/*.rs

if [[ "$mode" == "--update" ]]; then
  mkdir -p "$tmp/previous"
  for path in "${generated_paths[@]}"; do
    mkdir -p "$tmp/previous/$(dirname "$path")" "$generated/$(dirname "$path")"
    cp -R "$generated/$path" "$tmp/previous/$path"
    rm -rf "$generated/$path"
    cp -R "$tmp/$path" "$generated/$path"
  done
  if ! cargo test --manifest-path "$repo_root/Cargo.toml" -p dialkit-api-generated --tests; then
    for path in "${generated_paths[@]}"; do
      rm -rf "$generated/$path"
      cp -R "$tmp/previous/$path" "$generated/$path"
    done
    echo "candidate failed generated-layer tests; restored previous output" >&2
    exit 1
  fi
  printf '%s\n' "updated all declared generator-owned artifacts from verified inputs"
  exit 0
fi

status=0
for path in "${generated_paths[@]}"; do
  if ! diff -qr "$tmp/$path" "$generated/$path"; then
    status=1
  fi
done
if [[ $status -ne 0 ]]; then
  echo "generated output differs; review inputs then run codegen/regenerate.sh --update" >&2
  exit 1
fi
cargo test --manifest-path "$repo_root/Cargo.toml" -p dialkit-api-generated --tests
echo "generated output is clean"
