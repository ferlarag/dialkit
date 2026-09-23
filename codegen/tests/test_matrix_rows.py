"""Mutation checks for the pinned REST matrix-row release totals."""

import pathlib
import tempfile
import unittest
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "codegen"))
from audit_coverage import audit_matrix_rows  # noqa: E402


class MatrixRowAuditTest(unittest.TestCase):
    def test_pinned_rows_match_and_domain_mutation_fails(self):
        specification = ROOT / "specs/002-voice-sms-coverage/spec.md"
        api = ROOT / "codegen/coverage/rest-api-2010.toml"
        messaging = ROOT / "codegen/coverage/rest-messaging-v1.toml"
        errors = []
        rows = audit_matrix_rows(specification, (api, messaging), errors)
        self.assertEqual(errors, [])
        self.assertEqual(len(rows), 66)
        self.assertEqual(sum(row["actual"] for row in rows), 197)

        with tempfile.TemporaryDirectory() as directory:
            mutated = pathlib.Path(directory) / "api.toml"
            mutated.write_text(api.read_text().replace('domain = "Calls"', 'domain = "Wrong calls"', 1))
            errors = []
            audit_matrix_rows(specification, (mutated, messaging), errors)
            self.assertTrue(any("matrix-row operation total or identity mismatch" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
