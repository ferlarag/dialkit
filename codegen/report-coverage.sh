#!/usr/bin/env bash
set -euo pipefail
repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
"$repo_root/codegen/audit-coverage.sh"
python3 - "$repo_root" <<'PY'
import json, pathlib, sys
root=pathlib.Path(sys.argv[1]); report=json.loads((root/"target/coverage/report.json").read_text())
text=f'''# Coverage report

Status: **{report["status"].upper()}**

| Inventory | Total |
|---|---:|
| API-2010 selected | {report["api_2010"]} |
| MSG-V1 selected | {report["messaging_v1"]} |
| REST union | {report["rest_total"]} |
| TwiML relationship entries | {report["twiml_entries"]} |
| Webhook families | {report["webhook_families"]} |
| Missing/invalid evidence | {report["missing"]} |

Audit duration: {report["duration_seconds"]} seconds.
'''
(root/"target/coverage/README.md").write_text(text)
PY
echo "wrote target/coverage/README.md and report.json"
