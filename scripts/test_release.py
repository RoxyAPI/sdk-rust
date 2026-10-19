import copy
import contextlib
import io
import json
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch
from urllib.error import HTTPError, URLError

from release import archive_files, bump_version, classify, ensure_github_release, latest_tag, main, plan, semver, verify_archive


def spec():
    return {"paths": {"/sample": {"get": {"operationId": "getSample", "parameters": [], "responses": {"200": {"description": "Result", "content": {"application/json": {"schema": {"type": "object", "properties": {"value": {"type": "string"}, "description": {"type": "string"}}}}}}}}}}}


def schema(document):
    return document["paths"]["/sample"]["get"]["responses"]["200"]["content"]["application/json"]["schema"]


def archive(files):
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as output:
        for path, data in files.items():
            member = tarfile.TarInfo(f"sample-1.1.1/{path}")
            member.size = len(data)
            output.addfile(member, io.BytesIO(data))
    return buffer.getvalue()


def http_error(status):
    return HTTPError("https://api.github.com", status, "HTTP error", {}, io.BytesIO())


class ReleaseTests(unittest.TestCase):
    def test_latest_tag_returns_none_only_when_reachable_tags_are_absent(self):
        with patch("release.subprocess.check_output", return_value="") as git:
            self.assertIsNone(latest_tag())
        git.assert_called_once_with(["git", "tag", "--list", "v*", "--no-column", "--merged", "HEAD"], text=True)

    def test_latest_tag_describes_existing_reachable_tags(self):
        with patch("release.subprocess.check_output", side_effect=["v1.1.1\nv1.2.0\n", "v1.2.0\n"]) as git:
            self.assertEqual(latest_tag(), "v1.2.0")
        self.assertEqual(git.call_args_list[-1].args[0], ["git", "describe", "--tags", "--abbrev=0", "--match", "v*", "HEAD"])

    def test_latest_tag_excludes_first_release_without_parent_ref(self):
        with patch("release.subprocess.check_output", return_value="v1.1.1\n") as git:
            self.assertIsNone(latest_tag("v1.1.1", exclude="v1.1.1"))
        git.assert_called_once_with(["git", "tag", "--list", "v*", "--no-column", "--merged", "v1.1.1"], text=True)

    def test_latest_tag_excludes_current_release_when_describing_previous(self):
        with patch("release.subprocess.check_output", side_effect=["v1.1.1\nv1.2.0\n", "v1.1.1\n"]) as git:
            self.assertEqual(latest_tag("v1.2.0", exclude="v1.2.0"), "v1.1.1")
        self.assertEqual(git.call_args_list[-1].args[0], ["git", "describe", "--tags", "--abbrev=0", "--match", "v*", "--exclude", "v1.2.0", "v1.2.0"])

    def test_latest_tag_propagates_listing_and_describe_failures(self):
        for ref, responses in [
            ("invalid-ref", [subprocess.CalledProcessError(128, ["git", "tag"])]),
            ("HEAD", [subprocess.CalledProcessError(1, ["git", "tag"])]),
            ("HEAD", ["v1.1.1\n", subprocess.CalledProcessError(128, ["git", "describe"])]),
        ]:
            with self.subTest(ref=ref, responses=responses), patch("release.subprocess.check_output", side_effect=responses):
                with self.assertRaises(subprocess.CalledProcessError):
                    latest_tag(ref)

    def test_semver_validation(self):
        self.assertEqual(semver("1.12.3"), (1, 12, 3))
        for value in ["v1.2.3", "1.2", "1.2.3-beta", "01.2.3", "1.2.3\n"]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                semver(value)
        self.assertEqual(bump_version("1.12.3", "minor"), "1.13.0")

    def test_copy_only_patch(self):
        before = spec()
        after = copy.deepcopy(before)
        schema(after)["properties"]["value"]["description"] = "Value description"
        after["paths"]["/sample"]["get"]["tags"] = ["Samples"]
        self.assertEqual(classify(before, after), "patch")

    def test_schema_keyword_names_are_valid_property_names(self):
        before = spec()
        schema(before)["properties"]["properties"] = {"type": "string"}
        after = copy.deepcopy(before)
        schema(after)["properties"]["properties"]["description"] = "Documentation"
        self.assertEqual(classify(before, after), "patch")
        schema(after)["properties"]["properties"]["type"] = "integer"
        self.assertEqual(classify(before, after), "major")

    def test_optional_public_request_and_response_field_additions_break(self):
        for section in ["response", "request"]:
            with self.subTest(section=section):
                before = spec()
                if section == "request":
                    before["paths"]["/sample"]["get"]["requestBody"] = {"content": {"application/json": {"schema": copy.deepcopy(schema(before))}}}
                after = copy.deepcopy(before)
                if section == "response":
                    schema(after)["properties"]["new"] = {"type": "string"}
                else:
                    after["paths"]["/sample"]["get"]["requestBody"]["content"]["application/json"]["schema"]["properties"]["new"] = {"type": "string"}
                self.assertEqual(classify(before, after), "major")

    def test_added_endpoint_is_minor(self):
        before = spec()
        after = copy.deepcopy(before)
        after["paths"]["/new"] = after["paths"]["/sample"]
        self.assertEqual(classify(before, after), "minor")
        after = copy.deepcopy(before)
        after["paths"]["/sample"]["post"] = after["paths"]["/sample"]["get"]
        self.assertEqual(classify(before, after), "minor")

    def test_new_component_is_minor(self):
        before = spec()
        after = copy.deepcopy(before)
        after["components"] = {"schemas": {"NewResult": {"type": "object", "properties": {"new": {"type": "string"}}}}}
        self.assertEqual(classify(before, after), "minor")

    def test_required_public_response_field_additions_break(self):
        for original_required in [None, [], ["value"]]:
            with self.subTest(required=original_required):
                before = spec()
                if original_required is not None:
                    schema(before)["required"] = original_required
                after = copy.deepcopy(before)
                schema(after)["properties"]["new"] = {"type": "integer"}
                schema(after)["required"] = [*(original_required or []), "new"]
                self.assertEqual(classify(before, after), "major")

    def test_new_required_request_fields_break(self):
        before = spec()
        before["paths"]["/sample"]["get"]["requestBody"] = {"content": {"application/json": {"schema": copy.deepcopy(schema(before))}}}
        after = copy.deepcopy(before)
        request_schema = after["paths"]["/sample"]["get"]["requestBody"]["content"]["application/json"]["schema"]
        request_schema["properties"]["new"] = {"type": "integer"}
        request_schema["required"] = ["new"]
        self.assertEqual(classify(before, after), "major")

    def test_absent_and_empty_required_are_equivalent(self):
        before = spec()
        after = copy.deepcopy(before)
        schema(after)["required"] = []
        self.assertEqual(classify(before, after), "patch")
        schema(after)["properties"]["new"] = {"type": "integer"}
        self.assertEqual(classify(before, after), "major")

    def test_existing_public_component_fields_break(self):
        before = spec()
        before["components"] = {"schemas": {"Sample": copy.deepcopy(schema(before))}}
        for required in [False, True]:
            with self.subTest(required=required):
                after = copy.deepcopy(before)
                after["components"]["schemas"]["Sample"]["properties"]["new"] = {"type": "string"}
                if required:
                    after["components"]["schemas"]["Sample"]["required"] = ["new"]
                self.assertEqual(classify(before, after), "major")

    def test_required_and_optional_parameter_additions_break(self):
        before = spec()
        after = copy.deepcopy(before)
        parameter = {"in": "query", "name": "limit", "schema": {"type": "integer"}}
        after["paths"]["/sample"]["get"]["parameters"].append(parameter)
        self.assertEqual(classify(before, after), "major")
        parameter["required"] = True
        self.assertEqual(classify(before, after), "major")
        del before["paths"]["/sample"]["get"]["parameters"]
        parameter.pop("required")
        self.assertEqual(classify(before, after), "major")
        before = copy.deepcopy(after)
        after["paths"]["/sample"]["get"]["parameters"].append({"in": "query", "name": "offset", "schema": {"type": "integer"}})
        self.assertEqual(classify(before, after), "major")

    def test_new_public_fields_require_explicit_major_dispatch(self):
        before = spec()
        after = copy.deepcopy(before)
        schema(after)["properties"]["new"] = {"type": "string"}
        for event, options in [("schedule", {}), ("workflow_dispatch", {"bump": "minor"}), ("workflow_dispatch", {"exact": "1.2.0"})]:
            with self.subTest(event=event, options=options), self.assertRaises(ValueError):
                plan(before, after, "1.1.1", "1.1.1", ["1.1.1"], event, **options)
        self.assertEqual(plan(before, after, "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", bump="major")["version"], "2.0.0")
        self.assertEqual(plan(before, after, "1.1.1", "1.1.1", ["1.1.1", "2.0.0"], "workflow_dispatch", exact="2.0.0")["version"], "2.0.0")

    def test_required_fields_type_changes_removals_and_renames_break(self):
        for change in ["required", "type", "remove", "rename", "description-field"]:
            with self.subTest(change=change):
                before = spec()
                after = copy.deepcopy(before)
                if change == "required":
                    schema(after)["required"] = ["value"]
                elif change == "type":
                    schema(after)["properties"]["value"]["type"] = "integer"
                elif change == "remove":
                    del schema(after)["properties"]["value"]
                elif change == "description-field":
                    del schema(after)["properties"]["description"]
                else:
                    after["paths"]["/sample"]["get"]["operationId"] = "getRenamed"
                self.assertEqual(classify(before, after), "major")

    def test_initial_release_and_unchanged_day(self):
        document = spec()
        self.assertEqual(plan(None, document, "1.1.1", None, [], "schedule")["version"], "1.1.1")
        self.assertIsNone(plan(document, document, "1.1.1", "1.1.1", ["1.1.1"], "schedule"))
        reformatted = json.loads(json.dumps(document, sort_keys=True, indent=4))
        self.assertIsNone(plan(document, reformatted, "1.1.1", "1.1.1", ["1.1.1"], "schedule"))

    def test_registry_floor_recovers_publish_before_push_failure(self):
        before = spec()
        after = copy.deepcopy(before)
        schema(after)["description"] = "New copy"
        self.assertEqual(plan(before, after, "1.1.1", "1.1.1", ["1.1.1", "1.1.2"], "schedule")["version"], "1.1.3")
        self.assertEqual(plan(None, before, "1.1.1", None, ["1.1.1"], "schedule")["version"], "1.1.2")

    def test_dispatch_validation_and_breaking_release(self):
        document = spec()
        self.assertEqual(plan(document, document, "1.1.1", "1.1.1", [], "workflow_dispatch", exact="1.2.0")["version"], "1.2.0")
        for exact, bump in [("1.1.1", ""), ("1.2.0", "patch"), ("", "invalid")]:
            with self.subTest(exact=exact, bump=bump), self.assertRaises(ValueError):
                plan(document, document, "1.1.1", "1.1.1", [], "workflow_dispatch", exact, bump)
        changed = copy.deepcopy(document)
        del schema(changed)["properties"]["value"]
        with self.assertRaises(ValueError):
            plan(document, changed, "1.1.1", "1.1.1", [], "schedule")
        with self.assertRaises(ValueError):
            plan(document, changed, "1.1.1", "1.1.1", [], "workflow_dispatch", exact="1.2.0")
        self.assertEqual(plan(document, changed, "1.1.1", "1.1.1", [], "workflow_dispatch", bump="major")["version"], "2.0.0")

    def test_exact_version_respects_new_endpoint_minimum(self):
        before = spec()
        after = copy.deepcopy(before)
        after["paths"]["/new"] = after["paths"]["/sample"]
        with self.assertRaises(ValueError):
            plan(before, after, "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", exact="1.1.2")
        for version in ["1.2.0", "2.0.0"]:
            with self.subTest(version=version):
                self.assertEqual(plan(before, after, "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", exact=version)["version"], version)
        self.assertEqual(plan(before, after, "1.1.1", "1.1.1", ["1.2.0"], "workflow_dispatch", exact="1.2.0")["version"], "1.2.0")

    def test_existing_registry_archive_must_match_all_package_files(self):
        files = {"src/lib.rs": b"pub struct Sample;", "Cargo.toml": b"manifest", ".cargo_vcs_info.json": b"commit-a"}
        other = dict(files, **{".cargo_vcs_info.json": b"commit-b"})
        verify_archive(archive(files), archive(other))
        other["src/lib.rs"] = b"pub struct Different;"
        with self.assertRaises(ValueError):
            verify_archive(archive(files), archive(other))
        del other["src/lib.rs"]
        with self.assertRaises(ValueError):
            verify_archive(archive(files), archive(other))
        with self.assertRaises(ValueError):
            archive_files(archive({}))

    def test_plan_command_reads_released_tag_baseline(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "specs").mkdir()
            baseline = root / "specs/openapi.json"
            baseline.write_text(json.dumps(spec()))
            (root / "Cargo.toml").write_text('[package]\nname = "roxyapi"\nversion = "1.1.1"\n')
            arguments = ["release.py", "plan", "--event", "schedule", "--notes", str(root / "notes")]
            for changed in [False, True]:
                current = spec()
                if changed:
                    schema(current)["description"] = "Updated documentation"
                baseline.write_text(json.dumps(current, indent=4))
                output = io.StringIO()
                with contextlib.chdir(root), patch.object(sys, "argv", arguments), patch("release.registry_versions", return_value=["1.1.1"]) as registry, patch("release.latest_tag", return_value="v1.1.1"), patch("release.subprocess.check_output", return_value=json.dumps(spec()).encode()) as git_show, contextlib.redirect_stdout(output):
                    main()
                git_show.assert_called_once_with(["git", "show", "v1.1.1:specs/openapi.json"])
                if changed:
                    registry.assert_called_once_with("roxyapi")
                else:
                    registry.assert_not_called()
                self.assertIn(f"release={str(changed).lower()}", output.getvalue())

    def test_existing_github_release_is_unchanged(self):
        with patch("release.request", return_value=io.BytesIO(b"{}")) as lookup, patch("release.subprocess.run") as command:
            self.assertFalse(ensure_github_release("v1.1.2", "RoxyAPI/sdk-rust", "test"))
        self.assertEqual(lookup.call_args.args[0], "https://api.github.com/repos/RoxyAPI/sdk-rust/releases/tags/v1.1.2")
        command.assert_not_called()

    def test_missing_github_release_uses_original_generated_notes(self):
        missing = http_error(404)
        notes = "RoxyAPI Rust SDK 1.1.2\n\nGenerated release notes.\n"
        with patch("release.request", side_effect=missing), patch("release.subprocess.run") as command, patch("release.tagged_spec") as tagged:
            self.assertTrue(ensure_github_release("v1.1.2", "RoxyAPI/sdk-rust", "test", notes))
        command.assert_called_once_with([
            "gh", "release", "create", "v1.1.2", "--repo", "RoxyAPI/sdk-rust", "--verify-tag", "--title", "v1.1.2",
            "--notes-file", "-", "--latest",
        ], input=notes, text=True, check=True)
        tagged.assert_not_called()

    def test_missing_github_release_recovers_notes_from_tagged_specs(self):
        before = spec()
        after = copy.deepcopy(before)
        after["paths"]["/new"] = after["paths"]["/sample"]
        missing = http_error(404)
        with patch("release.request", side_effect=missing), patch("release.latest_tag", return_value="v1.1.1") as previous, patch("release.tagged_spec", side_effect=lambda tag: {"v1.1.1": before, "v1.2.0": after}[tag]) as tagged, patch("release.subprocess.run") as command:
            self.assertTrue(ensure_github_release("v1.2.0", "RoxyAPI/sdk-rust", "test"))
        previous.assert_called_once_with("v1.2.0", exclude="v1.2.0")
        self.assertEqual({call.args[0] for call in tagged.call_args_list}, {"v1.1.1", "v1.2.0"})
        self.assertIn("`/new`", command.call_args.kwargs["input"])
        self.assertNotIn("`/sample`", command.call_args.kwargs["input"])

    def test_first_tag_recovery_has_no_previous_release(self):
        with patch("release.request", side_effect=http_error(404)), patch("release.latest_tag", return_value=None), patch("release.tagged_spec", return_value=spec()) as tagged, patch("release.subprocess.run") as command:
            self.assertTrue(ensure_github_release("v1.1.1", "RoxyAPI/sdk-rust", "test"))
        tagged.assert_called_once_with("v1.1.1")
        self.assertIn("RoxyAPI Rust SDK 1.1.1", command.call_args.kwargs["input"])
        self.assertIn("`/sample`", command.call_args.kwargs["input"])

    def test_github_lookup_errors_do_not_create_releases(self):
        for error in [*[http_error(code) for code in [401, 403, 429, 500]], URLError("connection failed")]:
            with self.subTest(error=error), patch("release.request", side_effect=error), patch("release.subprocess.run") as command:
                with self.assertRaises(type(error)):
                    ensure_github_release("v1.1.2", "RoxyAPI/sdk-rust", "test")
                command.assert_not_called()

    def test_github_creation_errors_are_visible(self):
        missing = http_error(404)
        with patch("release.request", side_effect=missing), patch("release.subprocess.run", side_effect=subprocess.CalledProcessError(1, ["gh", "release", "create"])):
            with self.assertRaises(subprocess.CalledProcessError):
                ensure_github_release("v1.1.2", "RoxyAPI/sdk-rust", "test", "Release notes")

    def test_github_reconciliation_never_enters_crate_publication(self):
        with tempfile.TemporaryDirectory() as directory:
            arguments = ["release.py", "github-release", "--notes", str(Path(directory) / "missing-notes")]
            with patch.object(sys, "argv", arguments), patch.dict("os.environ", {"GITHUB_REPOSITORY": "RoxyAPI/sdk-rust", "GH_TOKEN": "test"}), patch("release.latest_tag", return_value="v1.1.2"), patch("release.ensure_github_release") as reconcile, patch("release.registry_versions") as registry:
                main()
        reconcile.assert_called_once_with("v1.1.2", "RoxyAPI/sdk-rust", "test", None)
        registry.assert_not_called()


if __name__ == "__main__":
    unittest.main()
