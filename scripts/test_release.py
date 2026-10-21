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

from release import additive, archive_files, bump_version, constructor_breaks, constructors, ensure_github_release, latest_tag, main, plan, rust_api_break, semver, shipped, verify_archive


def spec():
    return {"paths": {"/sample": {"get": {"operationId": "getSample", "parameters": [], "responses": {"200": {"description": "Result", "content": {"application/json": {"schema": {"type": "object", "properties": {"value": {"type": "string"}, "description": {"type": "string"}}}}}}}}}}}


def generated(*constructors):
    """A generated types.rs holding one constructor per (type, parameter list) pair, in the emitted format."""
    return {"types.rs": "".join(
        f"impl {name} {{\n    /// Builds the value from its required fields, in specification order.\n    pub fn new({parameters}) -> Self {{\n        todo!()\n    }}\n}}\n"
        for name, parameters in constructors
    )}


RELEASED = generated(("BirthRequest", "date: String, latitude: f64, longitude: f64"))


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

    def test_copy_only_changes_are_not_additive(self):
        before = spec()
        after = copy.deepcopy(before)
        schema(after)["properties"]["value"]["description"] = "Value description"
        after["paths"]["/sample"]["get"]["tags"] = ["Samples"]
        after["info"] = {"description": "New overview"}
        self.assertFalse(additive(before, after))

    def test_constraint_type_and_removal_changes_are_not_additive(self):
        for change in ["constraint", "type", "remove", "required"]:
            with self.subTest(change=change):
                before = spec()
                after = copy.deepcopy(before)
                if change == "constraint":
                    schema(after)["properties"]["value"]["maxLength"] = 20
                    schema(before)["properties"]["value"]["maxLength"] = 10
                elif change == "type":
                    schema(after)["properties"]["value"]["type"] = "integer"
                elif change == "remove":
                    del schema(after)["properties"]["value"]
                else:
                    schema(after)["required"] = ["value"]
                    schema(before)["required"] = []
                self.assertFalse(additive(before, after))

    def test_new_paths_methods_components_fields_parameters_and_enum_values_are_additive(self):
        for change in ["path", "method", "component", "field", "parameter", "enum"]:
            with self.subTest(change=change):
                before = spec()
                schema(before)["properties"]["value"]["enum"] = ["a"]
                after = copy.deepcopy(before)
                if change == "path":
                    after["paths"]["/new"] = after["paths"]["/sample"]
                elif change == "method":
                    after["paths"]["/sample"]["post"] = after["paths"]["/sample"]["get"]
                elif change == "component":
                    after["components"] = {"schemas": {"NewResult": {"type": "object"}}}
                elif change == "field":
                    schema(after)["properties"]["new"] = {"type": "string"}
                elif change == "parameter":
                    after["paths"]["/sample"]["get"]["parameters"].append({"in": "query", "name": "limit", "schema": {"type": "integer"}})
                else:
                    schema(after)["properties"]["value"]["enum"].append("b")
                self.assertTrue(additive(before, after))
        self.assertTrue(additive(None, spec()))

    def test_schema_keyword_names_are_valid_property_names(self):
        before = spec()
        schema(before)["properties"]["properties"] = {"type": "string"}
        after = copy.deepcopy(before)
        schema(after)["properties"]["properties"]["description"] = "Documentation"
        self.assertFalse(additive(before, after))
        schema(after)["properties"]["properties"]["format"] = "date"
        self.assertTrue(additive(before, after))

    def test_semver_checks_exit_codes(self):
        for code, broken in [(0, False), (100, True)]:
            with self.subTest(code=code), patch("release.subprocess.run", return_value=subprocess.CompletedProcess([], code)) as command:
                self.assertEqual(rust_api_break("1.1.2"), broken)
            command.assert_called_once_with(["cargo", "semver-checks", "check-release", "--baseline-version", "1.1.2", "--release-type", "minor"])
        with patch("release.subprocess.run", return_value=subprocess.CompletedProcess([], 101)), self.assertRaises(ValueError):
            rust_api_break("1.1.2")

    def test_major_changes_refuse_scheduled_and_require_explicit_major_dispatch(self):
        for event, options in [("schedule", {}), ("workflow_dispatch", {}), ("workflow_dispatch", {"bump": "minor"}), ("workflow_dispatch", {"exact": "1.2.0"})]:
            with self.subTest(event=event, options=options), self.assertRaisesRegex(ValueError, "major"):
                plan("major", "1.1.1", "1.1.1", ["1.1.1"], event, **options)
        self.assertEqual(plan("major", "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", bump="major")["version"], "2.0.0")
        self.assertEqual(plan("major", "1.1.1", "1.1.1", ["1.1.1", "2.0.0"], "workflow_dispatch", exact="2.0.0")["version"], "2.0.0")

    def test_scheduled_patch_and_minor_publish_unattended(self):
        self.assertEqual(plan("patch", "1.1.2", "1.1.2", ["1.1.2"], "schedule")["version"], "1.1.3")
        self.assertEqual(plan("minor", "1.1.2", "1.1.2", ["1.1.2"], "schedule")["version"], "1.2.0")

    def test_initial_release(self):
        self.assertEqual(plan("patch", "1.1.1", None, [], "schedule")["version"], "1.1.1")

    def test_registry_floor_recovers_publish_before_push_failure(self):
        self.assertEqual(plan("patch", "1.1.1", "1.1.1", ["1.1.1", "1.1.2"], "schedule")["version"], "1.1.3")
        self.assertEqual(plan("patch", "1.1.1", None, ["1.1.1"], "schedule")["version"], "1.1.2")

    def test_dispatch_validation(self):
        self.assertEqual(plan("patch", "1.1.1", "1.1.1", [], "workflow_dispatch", exact="1.2.0")["version"], "1.2.0")
        for exact, bump in [("1.1.1", ""), ("1.2.0", "patch"), ("", "invalid")]:
            with self.subTest(exact=exact, bump=bump), self.assertRaises(ValueError):
                plan("patch", "1.1.1", "1.1.1", [], "workflow_dispatch", exact, bump)
        with self.assertRaises(ValueError):
            plan("minor", "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", bump="patch")

    def test_exact_version_respects_minor_minimum(self):
        with self.assertRaises(ValueError):
            plan("minor", "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", exact="1.1.2")
        for version in ["1.2.0", "2.0.0"]:
            with self.subTest(version=version):
                self.assertEqual(plan("minor", "1.1.1", "1.1.1", ["1.1.1"], "workflow_dispatch", exact=version)["version"], version)
        self.assertEqual(plan("minor", "1.1.1", "1.1.1", ["1.2.0"], "workflow_dispatch", exact="1.2.0")["version"], "1.2.0")

    def test_shipped_masks_only_the_crate_version(self):
        manifest = b'[package]\nname = "roxyapi"\nkeywords = ["a"]\nversion = "1.1.2"\n\n[dependencies.serde]\nversion = "1.0"\n'
        lock = b'[[package]]\nname = "roxyapi"\nversion = "1.1.2"\n\n[[package]]\nname = "serde"\nversion = "1.0.0"\n'
        masked = shipped({"Cargo.toml": manifest, "Cargo.toml.orig": manifest, "Cargo.lock": lock, "src/lib.rs": b"1.1.2"}, "roxyapi")
        self.assertEqual(masked["Cargo.toml"], manifest.replace(b'version = "1.1.2"', b'version = "0.0.0"'))
        self.assertEqual(masked["Cargo.toml.orig"], masked["Cargo.toml"])
        self.assertEqual(masked["Cargo.lock"], lock.replace(b'version = "1.1.2"', b'version = "0.0.0"'))
        self.assertEqual(masked["src/lib.rs"], b"1.1.2")
        bumped = {"Cargo.toml": manifest.replace(b"1.1.2", b"1.1.3"), "Cargo.lock": lock.replace(b"1.1.2", b"1.1.3")}
        self.assertEqual(shipped(bumped, "roxyapi"), {key: masked[key] for key in bumped})

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

    def test_constructor_parser_reads_single_and_multi_line_signatures(self):
        sources = generated(("Empty", ""), ("Rooms", "r#type: RoomType, items: Vec<Item>, extra: HashMap<String, Value>"))
        sources["types.rs"] += "impl Wide {\n    pub fn new(\n        date: String,\n        latitude: f64,\n    ) -> Self {\n        todo!()\n    }\n}\n"
        self.assertEqual(constructors(sources), {
            "Empty": (),
            "Rooms": (("r#type", "RoomType"), ("items", "Vec<Item>"), ("extra", "HashMap<String, Value>")),
            "Wide": (("date", "String"), ("latitude", "f64")),
        })

    def test_unparseable_constructor_fails_loudly(self):
        for text in [
            "impl Sample {\n    pub fn other() {}\n    pub fn new(date: String) -> Self {\n        todo!()\n    }\n}\n",
            "impl<T> Sample<T> {\n    pub fn new(value: T) -> Self {\n        todo!()\n    }\n}\n",
            "impl Sample {\n    pub fn new(pair: (f64, f64)) -> Self {\n        todo!()\n    }\n}\n",
            generated(("Sample", "date String"))["types.rs"],
            generated(("Sample", "date: String"))["types.rs"] * 2,
        ]:
            with self.subTest(text=text), self.assertRaises(ValueError):
                constructors({"types.rs": text})

    def test_constructor_breaks_cover_reorder_rename_type_and_removal_only(self):
        before = constructors(generated(("Birth", "date: String, latitude: f64, longitude: f64"), ("Gone", "")))
        self.assertEqual(constructor_breaks(before, before), [])
        after = constructors(generated(("Birth", "date: String, longitude: f64, latitude: f64"), ("Gone", ""), ("Added", "name: String")))
        self.assertEqual(constructor_breaks(before, after), ["Birth::new(date: String, latitude: f64, longitude: f64) is now Birth::new(date: String, longitude: f64, latitude: f64)"])
        for parameters in ["day: String, latitude: f64, longitude: f64", "date: String, latitude: String, longitude: f64", "date: String, latitude: f64"]:
            with self.subTest(parameters=parameters):
                self.assertEqual(len(constructor_breaks(before, constructors(generated(("Birth", parameters), ("Gone", ""))))), 1)
        self.assertEqual(constructor_breaks(before, constructors(generated(("Birth", "date: String, latitude: f64, longitude: f64")))), ["Gone::new() was removed"])

    def run_plan(self, current, crate_matches=False, api_break=False, event="schedule", sources=RELEASED, bump=""):
        """Run the plan command against a v1.1.2 tag holding spec() and registry versions 1.1.2."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "specs").mkdir()
            (root / "specs/openapi.json").write_text(current)
            (root / "Cargo.toml").write_text('[package]\nname = "roxyapi"\nversion = "1.1.2"\n')
            released = {"src/lib.rs": b"released", "Cargo.toml": b'[package]\nversion = "1.1.2"\n'}
            local = released if crate_matches else dict(released, **{"src/lib.rs": b"changed"})
            arguments = ["release.py", "plan", "--event", event, "--notes", str(root / "notes"), "--bump", bump]
            output = io.StringIO()
            with contextlib.chdir(root), patch.object(sys, "argv", arguments), \
                    patch("release.latest_tag", return_value="v1.1.2"), \
                    patch("release.tagged_spec", return_value=spec()) as tagged, \
                    patch("release.registry_versions", return_value=["1.1.1", "1.1.2"]) as registry, \
                    patch("release.subprocess.run") as package, \
                    patch("release.local_crate", return_value=archive(local)), \
                    patch("release.registry_crate", return_value=archive(released)) as download, \
                    patch("release.rust_api_break", return_value=api_break) as checks, \
                    patch("release.generated_sources", side_effect=lambda tag=None: RELEASED if tag == "v1.1.2" else sources), \
                    contextlib.redirect_stdout(output):
                main()
                manifest = (root / "Cargo.toml").read_text()
            tagged.assert_called_once_with("v1.1.2")
            return output.getvalue(), manifest, registry, package, download, checks

    def test_formatting_only_spec_change_skips_registry_and_release(self):
        result, manifest, registry, package, _, checks = self.run_plan(json.dumps(spec(), sort_keys=True, indent=4))
        self.assertIn("release=false", result)
        self.assertIn('version = "1.1.2"', manifest)
        registry.assert_not_called()
        package.assert_not_called()
        checks.assert_not_called()

    def test_description_only_change_with_identical_crate_is_not_released(self):
        current = spec()
        current["info"] = {"description": "New overview"}
        result, manifest, registry, package, download, checks = self.run_plan(json.dumps(current), crate_matches=True)
        self.assertIn("release=false", result)
        self.assertIn('version = "1.1.2"', manifest)
        registry.assert_called_once_with("roxyapi")
        package.assert_called_once()
        download.assert_called_once_with("roxyapi", "1.1.2")
        checks.assert_not_called()

    def test_documentation_change_in_the_crate_is_a_scheduled_patch(self):
        current = spec()
        schema(current)["properties"]["value"]["description"] = "Updated documentation"
        result, manifest, *_ = self.run_plan(json.dumps(current))
        self.assertIn("release=true", result)
        self.assertIn("change=patch", result)
        self.assertIn('version = "1.1.3"', manifest)

    def test_additive_field_is_a_scheduled_minor(self):
        current = spec()
        schema(current)["properties"]["new"] = {"type": "string"}
        result, manifest, _, _, _, checks = self.run_plan(json.dumps(current))
        checks.assert_called_once_with("1.1.2")
        self.assertIn("change=minor", result)
        self.assertIn('version = "1.2.0"', manifest)

    def test_removal_breaking_the_rust_api_refuses_on_schedule(self):
        current = spec()
        del schema(current)["properties"]["value"]
        with self.assertRaisesRegex(ValueError, "version_bump=major"):
            self.run_plan(json.dumps(current), api_break=True)

    def test_reordered_existing_constructor_refuses_on_schedule_without_a_rust_api_break(self):
        current = spec()
        schema(current)["properties"]["value"]["description"] = "Updated documentation"
        reordered = generated(("BirthRequest", "date: String, longitude: f64, latitude: f64"))
        message = "BirthRequest::new\\(date: String, latitude: f64, longitude: f64\\) is now BirthRequest::new\\(date: String, longitude: f64, latitude: f64\\)"
        with self.assertRaisesRegex(ValueError, f"version_bump=major(.|\\n)*{message}"):
            self.run_plan(json.dumps(current), sources=reordered)
        result, manifest, *_ = self.run_plan(json.dumps(current), event="workflow_dispatch", sources=reordered, bump="major")
        self.assertIn("change=major", result)
        self.assertIn('version = "2.0.0"', manifest)

    def test_renamed_constructor_parameter_refuses_on_schedule(self):
        current = spec()
        schema(current)["properties"]["value"]["description"] = "Updated documentation"
        with self.assertRaisesRegex(ValueError, "BirthRequest::new\\(date: String, lat: f64, longitude: f64\\)"):
            self.run_plan(json.dumps(current), sources=generated(("BirthRequest", "date: String, lat: f64, longitude: f64")))

    def test_constructor_on_a_new_type_is_not_major(self):
        current = spec()
        schema(current)["properties"]["new"] = {"type": "string"}
        added = {"types.rs": RELEASED["types.rs"] + generated(("NewRequest", "name: String"))["types.rs"]}
        result, manifest, _, _, _, checks = self.run_plan(json.dumps(current), sources=added)
        checks.assert_called_once_with("1.1.2")
        self.assertIn("change=minor", result)
        self.assertIn('version = "1.2.0"', manifest)

    def test_unparseable_current_constructor_fails_the_plan(self):
        current = spec()
        schema(current)["properties"]["value"]["description"] = "Updated documentation"
        with self.assertRaisesRegex(ValueError, "Cannot parse"):
            self.run_plan(json.dumps(current), sources={"types.rs": "impl<T> Sample<T> {\n    pub fn new(value: T) -> Self {\n        todo!()\n    }\n}\n"})

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
