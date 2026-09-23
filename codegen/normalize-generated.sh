#!/usr/bin/env bash
set -euo pipefail

generated_root=${1:?generated output directory is required}
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

# OpenAPI Generator 7.25.0 emits duplicated model namespaces for several
# Twilio parameter-enum references and qualifies serde_json as a model.
find "$generated_root/src" -type f -name '*.rs' -exec sed -i \
  -e 's/models::models::/models::/g' \
  -e 's/models::serde_json::Value/serde_json::Value/g' {} +

# The upstream schema marks this account path parameter as required while the
# generator emits an Option-only branch for one operation.
incoming_phone_number_api="$generated_root/src/apis/api20100401_incoming_phone_number_api.rs"
if [[ -f "$incoming_phone_number_api" ]]; then
  sed -i \
    -e 's/urlencode(params.account_sid)/urlencode(\&params.account_sid)/g' \
    -e 's/if let Some(param_value) = params.account_sid {/let param_value = params.account_sid; {/' \
    "$incoming_phone_number_api"
  # The pinned operation has AccountSid both as a required path parameter and
  # as an optional request-body reassignment. Keep them independently
  # representable instead of forcing the path value into the body.
  if ! grep -q 'pub new_account_sid:' "$incoming_phone_number_api"; then
    perl -0pi -e '
      s/(pub struct UpdateIncomingPhoneNumberParams \{.*?pub sid: String,)/$1\n    \/\/ Optional account reassignment from the request body.\n    pub new_account_sid: Option<String>,/s
    ' "$incoming_phone_number_api"
  fi
  perl -0pi -e '
    s/    let param_value = params\.account_sid;\n    \{\n        multipart_form_params\.insert\("AccountSid", param_value\.to_string\(\)\);\n    \}/    if let Some(param_value) = params.new_account_sid {\n        multipart_form_params.insert("AccountSid", param_value.to_string());\n    }/s
  ' "$incoming_phone_number_api"
fi

# Authentication is owned by dialkit-core. Remove the stock generator's
# plaintext Basic-auth application from every operation; Configuration now
# carries the shared authenticated HttpClient instead.
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec perl -0pi -e '
  s/\n    if let Some\(ref auth_conf\) = configuration\.basic_auth \{\n        req_builder = req_builder\.basic_auth\(auth_conf\.0\.to_owned\(\), auth_conf\.1\.to_owned\(\)\);\n    \};\n/\n/g
' {} +

# Install the generated, model-specific adapters behind one pagination API.
if [[ -f "$generated_root/src/apis/api20100401_call_api.rs" ]]; then
  pagination_source=API-2010
else
  pagination_source=MSG-V1
fi
python3 "$repo_root/codegen/contracts/generate_pagination.py" \
  --root "$repo_root" --crate "$generated_root" --source "$pagination_source"

# Required arrays must also be encoded as repeated form keys. The stock Rust
# generator comma-joins them, which changes Twilio's documented cardinality.
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec perl -0pi -e '
  s/multipart_form_params\.insert\(\n        "([^"]+)",\n        params\s*\.\s*(\w+)\n            \.into_iter\(\)\n            \.map\(\|p\| p\.to_string\(\)\)\n            \.collect::<Vec<String>>\(\)\n            \.join\(","\)\n            \.to_string\(\),\n    \);/for param_value in params.$2 {\n        multipart_form_params.insert("$1", param_value.to_string());\n    }/g
' {} +

# OpenAPI Generator 7.25.0 incorrectly makes this required array optional.
tollfree_api="$generated_root/src/apis/messaging_v1_tollfree_verification_api.rs"
if [[ -f "$tollfree_api" ]]; then
  perl -0pi -e '
    s/(pub struct CreateTollfreeVerificationParams \{[^}]*?pub )use_case_categories: Option<Vec<String>>/$1use_case_categories: Vec<String>/s;
    s/    match params\.use_case_categories \{\n        Some\(param_value\) => \{\n            multipart_form_params\.insert\(\n                "UseCaseCategories",\n                param_value\n                    \.into_iter\(\)\n                    \.map\(\|p\| p\.to_string\(\)\)\n                    \.collect::<Vec<String>>\(\)\n                    \.join\(","\)\n                    \.to_string\(\),\n            \);\n        \}\n        None => \{\n            multipart_form_params\.insert\("UseCaseCategories", String::new\(\)\);\n        \}\n    \}/    for param_value in params.use_case_categories {\n        multipart_form_params.insert("UseCaseCategories", param_value.to_string());\n    }/s
  ' "$tollfree_api"
fi

# Preserve repeated form values and their order. The stock Rust template uses
# a HashMap and comma-joins arrays, which is not Twilio's repeated-key wire
# format.
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec sed -i \
  -e 's/let mut multipart_form_params = std::collections::HashMap::new();/let mut multipart_form_params = super::FormParams::default();/' \
  -e 's/multipart_form_params.insert(\("[^"]*"\), "");/multipart_form_params.insert(\1, String::new());/' {} +
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec perl -0pi -e '
  s/if let Some\(param_value\) = params\.(\w+) \{\n        multipart_form_params\.insert\(\n            "([^"]+)",\n            param_value\n                \.into_iter\(\)\n                \.map\(\|p\| p\.to_string\(\)\)\n                \.collect::<Vec<String>>\(\)\n                \.join\(","\)\n                \.to_string\(\),\n        \);\n    \}/if let Some(param_values) = params.$1 {\n        for param_value in param_values {\n            multipart_form_params.insert("$2", param_value.to_string());\n        }\n    }/g
' {} +
