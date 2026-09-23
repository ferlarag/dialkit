#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
manifest="$repo_root/codegen/generation-manifest.toml"
templates="$repo_root/codegen/templates/rust"
jar="$repo_root/codegen/.cache/openapi-generator-cli-7.25.0.jar"
mode=${1:---check}

case "$mode" in
  --check|--update) ;;
  *) echo "usage: $0 [--check|--update]" >&2; exit 2 ;;
esac

manifest_value() {
  local section=$1 key=$2
  awk -v section="[$section]" -v key="$key" '
    $0 == section { active=1; next }
    /^\[/ { active=0 }
    active && $1 == key { gsub(/["[:space:]]/, "", $3); print $3; exit }
  ' "$manifest"
}

source_value() {
  local source_id=$1 key=$2
  awk -v source_id="$source_id" -v key="$key" '
    /^\[\[sources\]\]$/ { active=1; matched=0; next }
    /^\[/ { active=0 }
    active && $1 == "id" {
      value=$3; gsub(/["[:space:]]/, "", value); matched=(value == source_id)
    }
    active && matched && $1 == key {
      value=$3; gsub(/["[:space:]]/, "", value); print value; exit
    }
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

source_ids=(API-2010 MSG-V1)
for source_id in "${source_ids[@]}"; do
  spec="$repo_root/$(source_value "$source_id" spec)"
  config="$repo_root/$(source_value "$source_id" config)"
  inventory="$repo_root/$(source_value "$source_id" inventory)"
  verify_hash "$spec" "$(source_value "$source_id" sha256)"
  verify_hash "$config" "$(source_value "$source_id" config_sha256)"
  verify_hash "$inventory" "$(source_value "$source_id" inventory_sha256)"
  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if [[ "$path" = /* || "$path" == *..* ]]; then
      echo "unsafe generated inventory path: $path" >&2
      exit 1
    fi
  done < "$inventory"
done

template_hash=$(
  cd "$templates"
  find . -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | cut -d' ' -f1
)
if [[ "$template_hash" != "$(manifest_value templates tree_sha256)" ]]; then
  echo "checksum mismatch: codegen/templates/rust" >&2
  exit 1
fi

mkdir -p "$(dirname "$jar")"
if [[ ! -f "$jar" ]]; then
  curl -fsSL "https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/7.25.0/openapi-generator-cli-7.25.0.jar" -o "$jar.download"
  mv "$jar.download" "$jar"
fi
verify_hash "$jar" "$(manifest_value generator artifact_sha256)"

tmp=$(mktemp -d "${TMPDIR:-/tmp}/dialkit-generate.XXXXXX")
trap 'rm -rf "$tmp"' EXIT
export TZ=UTC LC_ALL=C LANG=C

for source_id in "${source_ids[@]}"; do
  spec="$repo_root/$(source_value "$source_id" spec)"
  config="$repo_root/$(source_value "$source_id" config)"
  candidate="$tmp/$source_id"
  java -jar "$jar" validate -i "$spec" >/dev/null
  java -jar "$jar" generate \
    -g rust \
    -i "$spec" \
    -o "$candidate" \
    -c "$config" \
    -t "$templates" >/dev/null
  "$repo_root/codegen/normalize-generated.sh" "$candidate"
  rustfmt --edition 2024 "$candidate"/src/lib.rs "$candidate"/src/models/*.rs "$candidate"/src/apis/*.rs
  "$repo_root/codegen/normalize-generated.sh" "$candidate"
  rustfmt --edition 2024 "$candidate"/src/lib.rs "$candidate"/src/models/*.rs "$candidate"/src/apis/*.rs
done

compare_source() {
  local source_id=$1 status=0 output inventory candidate path
  output="$repo_root/$(source_value "$source_id" output)"
  inventory="$repo_root/$(source_value "$source_id" inventory)"
  candidate="$tmp/$source_id"
  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if ! diff -qr "$candidate/$path" "$output/$path"; then
      status=1
    fi
  done < "$inventory"
  return "$status"
}

if [[ "$mode" == "--check" ]]; then
  status=0
  for source_id in "${source_ids[@]}"; do
    compare_source "$source_id" || status=1
  done
  if [[ $status -ne 0 ]]; then
    echo "generated output differs; review inputs then run codegen/regenerate.sh --update" >&2
    exit 1
  fi
  python3 "$repo_root/codegen/contracts/generate_runtime_contracts.py" --root "$repo_root" --check
  python3 "$repo_root/codegen/contracts/generate_pagination.py" --root "$repo_root" \
    --crate "$repo_root/crates/api-generated" --source API-2010 \
    --test-output "$repo_root/crates/api-generated/tests/pagination_contracts.rs" --check
  python3 "$repo_root/codegen/contracts/generate_pagination.py" --root "$repo_root" \
    --crate "$repo_root/crates/messaging-generated" --source MSG-V1 \
    --test-output "$repo_root/crates/messaging-generated/tests/pagination_contracts.rs" --check
  cargo test --manifest-path "$repo_root/Cargo.toml" -p dialkit-api-generated --tests
  cargo test --manifest-path "$repo_root/Cargo.toml" -p dialkit-messaging-generated --tests
  echo "generated output is clean"
  exit 0
fi

restore_all() {
  local source_id output inventory backup path
  for source_id in "${source_ids[@]}"; do
    output="$repo_root/$(source_value "$source_id" output)"
    inventory="$repo_root/$(source_value "$source_id" inventory)"
    backup="$tmp/previous/$source_id"
    while IFS= read -r path; do
      [[ -z "$path" ]] && continue
      rm -rf "$output/$path"
      cp -R "$backup/$path" "$output/$path"
    done < "$inventory"
  done
}

for source_id in "${source_ids[@]}"; do
  output="$repo_root/$(source_value "$source_id" output)"
  inventory="$repo_root/$(source_value "$source_id" inventory)"
  candidate="$tmp/$source_id"
  backup="$tmp/previous/$source_id"
  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    mkdir -p "$backup/$(dirname "$path")" "$output/$(dirname "$path")"
    cp -R "$output/$path" "$backup/$path"
    rm -rf "$output/$path"
    cp -R "$candidate/$path" "$output/$path"
  done < "$inventory"
done

python3 "$repo_root/codegen/contracts/generate_runtime_contracts.py" --root "$repo_root"
python3 "$repo_root/codegen/contracts/generate_pagination.py" --root "$repo_root" \
  --crate "$repo_root/crates/api-generated" --source API-2010 \
  --test-output "$repo_root/crates/api-generated/tests/pagination_contracts.rs"
python3 "$repo_root/codegen/contracts/generate_pagination.py" --root "$repo_root" \
  --crate "$repo_root/crates/messaging-generated" --source MSG-V1 \
  --test-output "$repo_root/crates/messaging-generated/tests/pagination_contracts.rs"

if ! cargo test --manifest-path "$repo_root/Cargo.toml" -p dialkit-api-generated --tests \
  || ! cargo test --manifest-path "$repo_root/Cargo.toml" -p dialkit-messaging-generated --tests; then
  restore_all
  echo "candidate failed generated-layer tests; restored both previous outputs" >&2
  exit 1
fi

echo "updated all declared generator-owned artifacts from verified inputs"
