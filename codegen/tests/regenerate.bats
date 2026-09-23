#!/usr/bin/env bats

setup() {
  PROJECT_ROOT=$(cd "$BATS_TEST_DIRNAME/../.." && pwd)
  WORK_ROOT=$(mktemp -d "${TMPDIR:-/tmp}/dialkit-regen-test.XXXXXX")
  tar -C "$PROJECT_ROOT" --exclude=.git --exclude=target -cf - . | tar -C "$WORK_ROOT" -xf -
  # Never share artifacts with the source checkout. Rust embeds
  # CARGO_MANIFEST_DIR in tests that inspect repository fixtures, so sharing
  # the target directory would leave binaries pointing at this temporary
  # checkout after teardown.
  export CARGO_TARGET_DIR="$WORK_ROOT/target"
}

teardown() {
  rm -rf "$WORK_ROOT"
}

@test "unchanged pins reproduce a clean generated tree" {
  cd "$WORK_ROOT"
  run ./codegen/regenerate.sh --check
  [ "$status" -eq 0 ]
  [[ "$output" == *"generated output is clean"* ]]
}

@test "checksum mismatch stops before generated output changes" {
  cd "$WORK_ROOT"
  api_before=$(sha256sum crates/api-generated/src/lib.rs)
  messaging_before=$(sha256sum crates/messaging-generated/src/lib.rs)
  printf '\n' >> codegen/spec/twilio_messaging_v1.json
  run ./codegen/regenerate.sh --update
  [ "$status" -ne 0 ]
  [[ "$output" == *"checksum mismatch"* ]]
  [ "$api_before" = "$(sha256sum crates/api-generated/src/lib.rs)" ]
  [ "$messaging_before" = "$(sha256sum crates/messaging-generated/src/lib.rs)" ]
}

@test "check mode detects changed added and deleted generated paths" {
  cd "$WORK_ROOT"
  printf '\n// drift\n' >> crates/api-generated/src/lib.rs
  touch crates/api-generated/src/added_drift.rs
  rm crates/api-generated/src/models/call_enum_status.rs
  printf '\n# doc drift\n' >> crates/api-generated/docs/Api20100401CallApi.md
  printf '\n// messaging drift\n' >> crates/messaging-generated/src/lib.rs
  run ./codegen/regenerate.sh --check
  [ "$status" -ne 0 ]
  [[ "$output" == *"generated output differs"* ]]
}

@test "update mode replaces only generated source and docs" {
  cd "$WORK_ROOT"
  printf 'sentinel\n' > unrelated.txt
  printf '\n// drift\n' >> crates/api-generated/src/lib.rs
  printf '\n# drift\n' >> crates/api-generated/docs/Api20100401CallApi.md
  printf '\n// drift\n' >> crates/messaging-generated/src/lib.rs
  run ./codegen/regenerate.sh --update
  [ "$status" -eq 0 ]
  [ "$(cat unrelated.txt)" = "sentinel" ]
  ! grep -q 'drift' crates/api-generated/src/lib.rs
  ! grep -q 'drift' crates/api-generated/docs/Api20100401CallApi.md
  ! grep -q 'drift' crates/messaging-generated/src/lib.rs
}

@test "failed candidate tests restore both generated trees" {
  cd "$WORK_ROOT"
  api_before=$(sha256sum crates/api-generated/src/lib.rs)
  messaging_before=$(sha256sum crates/messaging-generated/src/lib.rs)
  printf '\n// drift\n' >> crates/api-generated/src/lib.rs
  printf '\n// drift\n' >> crates/messaging-generated/src/lib.rs
  printf 'this is not valid Rust\n' > crates/messaging-generated/tests/forced_failure.rs
  run ./codegen/regenerate.sh --update
  [ "$status" -ne 0 ]
  [[ "$output" == *"restored both previous outputs"* ]]
  ! [ "$api_before" = "$(sha256sum crates/api-generated/src/lib.rs)" ]
  ! [ "$messaging_before" = "$(sha256sum crates/messaging-generated/src/lib.rs)" ]
  grep -q 'drift' crates/api-generated/src/lib.rs
  grep -q 'drift' crates/messaging-generated/src/lib.rs
}
