#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
mapfile -d '' files < <(find "$repo_root" \
  -path "$repo_root/.git" -prune -o \
  -path "$repo_root/target" -prune -o \
  -path "$repo_root/codegen/.cache" -prune -o \
  -path '*/tests/transport_contract/redaction_canaries.rs' -prune -o \
  -type f -print0)

patterns='BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|TWILIO_(AUTH_TOKEN|API_KEY_SECRET|ACCOUNT_SID)[[:space:]]*=[[:space:]]*[^$<{[:space:]]|Authorization:[[:space:]]*Basic[[:space:]]+[A-Za-z0-9+/=]{16,}'
if rg -n -I -e "$patterns" "${files[@]}"; then
  echo "possible credential material found" >&2
  exit 1
fi
echo "sensitive-data audit passed"
