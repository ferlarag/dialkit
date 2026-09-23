#!/usr/bin/env bats
setup() {
  PROJECT_ROOT=$(cd "$BATS_TEST_DIRNAME/../.." && pwd)
  WORK_ROOT=$(mktemp -d)
  cp "$PROJECT_ROOT/codegen/coverage/rest-api-2010.toml" "$WORK_ROOT/api.toml"
  cp "$PROJECT_ROOT/codegen/coverage/twiml-nodes.toml" "$WORK_ROOT/twiml.toml"
  cp "$PROJECT_ROOT/codegen/coverage/webhook-families.toml" "$WORK_ROOT/webhooks.toml"
}
teardown() { rm -rf "$WORK_ROOT"; }

@test "complete union TwiML and webhook inventory passes under ten minutes" {
  cd "$PROJECT_ROOT"
  start=$SECONDS
  run ./codegen/audit-coverage.sh
  [ "$status" -eq 0 ]
  [[ "$output" == *"REST total: 197"* ]]
  [ $((SECONDS-start)) -lt 600 ]
}

@test "missing entry fails" {
  sed -i '/\[\[operation\]\]/,$d' "$WORK_ROOT/api.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --api-manifest "$WORK_ROOT/api.toml"
  [ "$status" -ne 0 ]
}

@test "duplicate entry fails" {
  awk 'BEGIN { seen=0 } /^\[\[operation\]\]/{ seen++; if (seen == 2) exit } seen == 1 { print }' "$WORK_ROOT/api.toml" >> "$WORK_ROOT/api.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --api-manifest "$WORK_ROOT/api.toml"
  [ "$status" -ne 0 ]
}

@test "wrong source fails" {
  sed -i 's/source_id = "API-2010"/source_id = "MSG-V1"/' "$WORK_ROOT/api.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --api-manifest "$WORK_ROOT/api.toml"
  [ "$status" -ne 0 ]
  [[ "$output" == *"wrong source_id"* ]]
}

@test "renamed operation fails" {
  sed -i '0,/operation_id = /s//operation_id = "LaterSourceOperation" #/' "$WORK_ROOT/api.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --api-manifest "$WORK_ROOT/api.toml"
  [ "$status" -ne 0 ]
}

@test "operation from a later source fails" {
  sed -i '0,/operation_id = /s//operation_id = "CreateAlphaSender" #/' "$WORK_ROOT/api.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --api-manifest "$WORK_ROOT/api.toml"
  [ "$status" -ne 0 ]
  [[ "$output" == *"unresolved or substituted"* ]]
}

@test "missing evidence fails" {
  sed -i '0,/evidence.success = /s//evidence.success_removed = /' "$WORK_ROOT/api.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --api-manifest "$WORK_ROOT/api.toml"
  [ "$status" -ne 0 ]
}

@test "duplicate TwiML relationship fails" {
  sed -i 's/nodes = \[/nodes = [\n  { family="voice", name="Connect", parent="Response", kind="verb", attributes=["action"], children=[] },/' "$WORK_ROOT/twiml.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --twiml-manifest "$WORK_ROOT/twiml.toml"
  [ "$status" -ne 0 ]
  [[ "$output" == *"duplicate coverage key"* ]]
}

@test "duplicate webhook family fails" {
  sed -i 's/families = \[/families = [\n  { family="voice-instruction", variants=["future"], encodings=["form"], parser="parse_voice" },/' "$WORK_ROOT/webhooks.toml"
  cd "$PROJECT_ROOT"
  run ./codegen/audit-coverage.sh --webhook-manifest "$WORK_ROOT/webhooks.toml"
  [ "$status" -ne 0 ]
  [[ "$output" == *"duplicate family"* ]]
}
