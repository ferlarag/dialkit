#!/usr/bin/env bats
@test "upstream workflow is report only for both pins and documentation manifests" {
  PROJECT_ROOT=$(cd "$BATS_TEST_DIRNAME/../.." && pwd)
  run grep -E 'twilio_(api_v2010|messaging_v1)\.json' "$PROJECT_ROOT/.github/workflows/upstream-spec-check.yml"
  [ "$status" -eq 0 ]
  run grep -q 'git diff --exit-code' "$PROJECT_ROOT/.github/workflows/upstream-spec-check.yml"
  [ "$status" -eq 0 ]
}
