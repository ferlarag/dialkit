#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
mapfile -d '' files < <(find "$repo_root" \
  -path "$repo_root/.git" -prune -o \
  -path "$repo_root/target" -prune -o \
  -path "$repo_root/codegen/.cache" -prune -o \
  -path "$repo_root/codegen/spec" -prune -o \
  -path '*/tests/transport_contract/redaction_canaries.rs' -prune -o \
  -type f -print0)

patterns="BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|TWILIO_ACCOUNT_SID[[:space:]]*=[[:space:]]*[\"']?AC[0-9A-Fa-f]{32}|TWILIO_(AUTH_TOKEN|API_KEY_SECRET)[[:space:]]*=[[:space:]]*[\"']?[A-Za-z0-9+/=]{20,}|Authorization:[[:space:]]*Basic[[:space:]]+[A-Za-z0-9+/=]{16,}|X-Twilio-Signature:[[:space:]]*[A-Za-z0-9+/=]{20,}|(SipPassword|sip_password|CardNumber|card_number|SecurityCode|security_code)[[:space:]]*[:=][[:space:]]*[\"'][^\"']{4,}"
if rg -n -I -e "$patterns" "${files[@]}"; then
  echo "possible credential material found" >&2
  exit 1
fi
echo "sensitive-data audit passed"
