#!/usr/bin/env bash
set -euo pipefail

generated_root=${1:?generated output directory is required}

# OpenAPI Generator 7.25.0 emits duplicated model namespaces for several
# Twilio parameter-enum references and qualifies serde_json as a model.
find "$generated_root/src" -type f -name '*.rs' -exec sed -i \
  -e 's/models::models::/models::/g' \
  -e 's/models::serde_json::Value/serde_json::Value/g' {} +

# The upstream schema marks this account path parameter as required while the
# generator emits an Option-only branch for one operation.
sed -i \
  -e 's/urlencode(params.account_sid)/urlencode(\&params.account_sid)/g' \
  -e 's/if let Some(param_value) = params.account_sid {/let param_value = params.account_sid; {/' \
  "$generated_root/src/apis/api20100401_incoming_phone_number_api.rs"

# Authentication is owned by dialkit-core. Remove the stock generator's
# plaintext Basic-auth application from every operation; Configuration now
# carries the shared authenticated HttpClient instead.
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec perl -0pi -e '
  s/\n    if let Some\(ref auth_conf\) = configuration\.basic_auth \{\n        req_builder = req_builder\.basic_auth\(auth_conf\.0\.to_owned\(\), auth_conf\.1\.to_owned\(\)\);\n    \};\n/\n/g
' {} +

# Preserve repeated form values and their order. The stock Rust template uses
# a HashMap and comma-joins arrays, which is not Twilio's repeated-key wire
# format.
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec sed -i \
  -e 's/let mut multipart_form_params = std::collections::HashMap::new();/let mut multipart_form_params = super::FormParams::default();/' {} +
find "$generated_root/src/apis" -type f -name '*_api.rs' -exec perl -0pi -e '
  s/if let Some\(param_value\) = params\.(\w+) \{\n        multipart_form_params\.insert\(\n            "([^"]+)",\n            param_value\n                \.into_iter\(\)\n                \.map\(\|p\| p\.to_string\(\)\)\n                \.collect::<Vec<String>>\(\)\n                \.join\(","\)\n                \.to_string\(\),\n        \);\n    \}/if let Some(param_values) = params.$1 {\n        for param_value in param_values {\n            multipart_form_params.insert("$2", param_value.to_string());\n        }\n    }/g
' {} +
