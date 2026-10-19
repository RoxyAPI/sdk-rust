#!/usr/bin/env python3
"""Plan spec-driven releases and verify already published package contents."""

import argparse
import io
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile
import tomllib
from urllib.error import HTTPError
from urllib.request import Request, urlopen


VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\Z")
METHODS = {"get", "post", "put", "patch", "delete", "head", "options", "trace"}
COPY_KEYS = {"description", "summary", "title", "example", "examples", "externalDocs", "tags", "deprecated"}
MAP_KEYS = {"paths", "properties", "patternProperties", "schemas", "$defs", "headers", "content", "responses", "mapping"}


def semver(value):
    if not VERSION.fullmatch(value):
        raise ValueError(f"Expected stable X.Y.Z version, got {value!r}")
    return tuple(map(int, value.split(".")))


def bump_version(value, bump):
    major, minor, patch = semver(value)
    return {"major": f"{major + 1}.0.0", "minor": f"{major}.{minor + 1}.0", "patch": f"{major}.{minor}.{patch + 1}"}[bump]


def contract(value, mapping=False):
    if isinstance(value, dict):
        return {key: contract(item, not mapping and key in MAP_KEYS) for key, item in value.items() if mapping or (key not in COPY_KEYS and not key.startswith("x-"))}
    if isinstance(value, list):
        return [contract(item) for item in value]
    return value


def classify(old, new):
    """Conservatively require a major version for changes to existing contracts."""
    level = 0

    def compare(before, after, key="", mapping=False):
        nonlocal level
        if before == after:
            return
        if isinstance(before, dict) and isinstance(after, dict):
            removed = before.keys() - after.keys()
            added = after.keys() - before.keys()
            required = not mapping and isinstance(before.get("required", []), list) and isinstance(after.get("required", []), list)
            if required:
                previous_required = set(before.get("required", []))
                current_required = set(after.get("required", []))
                if previous_required != current_required:
                    level = 2
                removed -= {"required"}
                added -= {"required"}
            if removed:
                level = 2
            if added:
                if (mapping and key in {"paths", "schemas", "parameters", "requestBodies", "responses", "securitySchemes"}) or (key.startswith("/") and added <= METHODS):
                    level = max(level, 1)
                else:
                    level = 2
            for name in before.keys() & after.keys():
                if required and name == "required":
                    continue
                compare(before[name], after[name], name, not mapping and name in MAP_KEYS)
        elif key == "parameters" and isinstance(before, list) and isinstance(after, list):
            indexed_before = {(item.get("in"), item.get("name"), item.get("$ref")): item for item in before}
            indexed_after = {(item.get("in"), item.get("name"), item.get("$ref")): item for item in after}
            if indexed_before.keys() != indexed_after.keys():
                level = 2
            for name in indexed_before.keys() & indexed_after.keys():
                compare(indexed_before[name], indexed_after[name])
        else:
            level = 2

    compare(contract(old.get("paths", {}), True), contract(new.get("paths", {}), True), "paths", mapping=True)
    compare(contract(old.get("components", {}).get("schemas", {}), True), contract(new.get("components", {}).get("schemas", {}), True), "schemas", mapping=True)
    for key in {"parameters", "requestBodies", "responses", "securitySchemes"}:
        compare(contract(old.get("components", {}).get(key, {}), True), contract(new.get("components", {}).get(key, {}), True), key, mapping=True)
    compare(old.get("security"), new.get("security"))
    return ("patch", "minor", "major")[level]


def plan(old_spec, new_spec, manifest_version, last_version, published_versions, event, exact="", bump=""):
    semver(manifest_version)
    if exact and bump:
        raise ValueError("Choose an exact version or a version bump, not both")
    if bump and bump not in {"patch", "minor", "major"}:
        raise ValueError("Version bump must be patch, minor, or major")
    change = classify(old_spec, new_spec) if old_spec is not None else "patch"
    changed = old_spec != new_spec
    if event != "workflow_dispatch" and not changed:
        return None
    if change == "major" and (event != "workflow_dispatch" or (not exact and bump != "major")):
        raise ValueError("Breaking spec change requires an explicit major release")
    floor = max([manifest_version, *published_versions, *([last_version] if last_version else [])], key=semver)
    if exact:
        semver(exact)
        if (last_version and semver(exact) <= semver(last_version)) or semver(exact) < semver(floor):
            raise ValueError("Exact version must exceed the last release and not precede the manifest or registry")
        if last_version and semver(exact) < semver(bump_version(last_version, change)):
            raise ValueError(f"Spec change requires at least a {change} release")
        target = exact
    elif old_spec is None and not published_versions and not bump:
        target = manifest_version
    else:
        requested = bump or change
        if change == "minor" and requested == "patch":
            raise ValueError("Additive spec change requires at least a minor release")
        target = bump_version(floor, requested)
    return {"version": target, "change": change, "bootstrap": not published_versions}


def request(url, headers=None):
    headers = {"User-Agent": "RoxyAPI-sdk-rust-release (https://github.com/RoxyAPI/sdk-rust)", **(headers or {})}
    return urlopen(Request(url, headers=headers), timeout=45)


def latest_tag(ref="HEAD", exclude=None):
    tags = subprocess.check_output(["git", "tag", "--list", "v*", "--no-column", "--merged", ref], text=True).splitlines()
    if not any(tag != exclude for tag in tags):
        return None
    command = ["git", "describe", "--tags", "--abbrev=0", "--match", "v*"]
    if exclude:
        command.extend(["--exclude", exclude])
    tag = subprocess.check_output([*command, ref], text=True).strip()
    semver(tag[1:])
    return tag


def tagged_spec(tag):
    return json.loads(subprocess.check_output(["git", "show", f"{tag}:specs/openapi.json"]))


def release_notes(version, previous, current):
    added = sorted(set(current.get("paths", {})) - set((previous or {}).get("paths", {})))
    notes = f"RoxyAPI Rust SDK {version}\n\n"
    notes += "New API paths:\n\n" + "\n".join(f"- `{path}`" for path in added) if added else "Updated API types and documentation from the OpenAPI specification."
    return notes + "\n"


def ensure_github_release(tag, repository, token, notes=None):
    if not repository or not token:
        raise ValueError("GITHUB_REPOSITORY and GH_TOKEN are required for release reconciliation")
    semver(tag[1:])
    try:
        with request(f"https://api.github.com/repos/{repository}/releases/tags/{tag}", {
            "Accept": "application/vnd.github+json", "Authorization": f"Bearer {token}",
        }):
            return False
    except HTTPError as error:
        error.close()
        if error.code != 404:
            raise
    if notes is None:
        previous = latest_tag(tag, exclude=tag)
        notes = release_notes(tag[1:], tagged_spec(previous) if previous else None, tagged_spec(tag))
    subprocess.run([
        "gh", "release", "create", tag, "--repo", repository, "--verify-tag", "--title", tag,
        "--notes-file", "-", "--latest",
    ], input=notes, text=True, check=True)
    return True


def registry_versions(name):
    try:
        with request(f"https://crates.io/api/v1/crates/{name}") as response:
            data = json.load(response)
    except HTTPError as error:
        if error.code == 404:
            return []
        raise
    return [version["num"] for version in data["versions"] if VERSION.fullmatch(version["num"])]


def archive_files(data):
    files = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive.getmembers():
            if not member.isfile():
                continue
            _, separator, name = member.name.partition("/")
            if not separator or name == ".cargo_vcs_info.json":
                continue
            if name in files:
                raise ValueError(f"Duplicate package archive path: {name}")
            files[name] = archive.extractfile(member).read()
    if not files:
        raise ValueError("Package archive is empty")
    return files


def verify_archive(local, remote):
    if archive_files(local) != archive_files(remote):
        raise ValueError("The registry version contains different files; choose a new version")


def output(**values):
    lines = [f"{key}={str(value).lower() if isinstance(value, bool) else value}" for key, value in values.items()]
    print("\n".join(lines))
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as file:
            file.write("\n".join(lines) + "\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    prepare = subparsers.add_parser("plan")
    prepare.add_argument("--event", required=True, choices=["schedule", "workflow_dispatch"])
    prepare.add_argument("--version", default="")
    prepare.add_argument("--bump", default="")
    prepare.add_argument("--notes", type=Path, required=True)
    subparsers.add_parser("publication")
    reconcile = subparsers.add_parser("github-release")
    reconcile.add_argument("--notes", type=Path)
    args = parser.parse_args()
    if args.command == "github-release":
        tag = latest_tag()
        if tag:
            notes = args.notes.read_text() if args.notes and args.notes.exists() else None
            ensure_github_release(tag, os.environ.get("GITHUB_REPOSITORY"), os.environ.get("GH_TOKEN"), notes)
        return
    manifest_path = Path("Cargo.toml")
    manifest_text = manifest_path.read_text()
    package = tomllib.loads(manifest_text)["package"]
    if args.command == "publication":
        versions = registry_versions(package["name"])
        version = package["version"]
        if version in versions:
            metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version=1"]))
            archive = Path(metadata["target_directory"]) / "package" / f"{package['name']}-{version}.crate"
            with request(f"https://crates.io/api/v1/crates/{package['name']}/{version}/download") as response:
                verify_archive(archive.read_bytes(), response.read())
            output(publish=False)
        else:
            if not versions:
                raise ValueError("Initial publication requires cargo publish --locked, then configure crates.io Trusted Publishing for release.yml and environment crates-io")
            output(publish=True)
        return
    last = latest_tag()
    previous = tagged_spec(last) if last else None
    spec = json.loads(Path("specs/openapi.json").read_text())
    if args.event != "workflow_dispatch" and previous == spec:
        output(release=False)
        return
    versions = registry_versions(package["name"])
    result = plan(previous, spec, package["version"], last[1:] if last else None, versions, args.event, args.version, args.bump)
    section = re.search(r"(?ms)^\[package\]\s*\n(.*?)(?=^\[|\Z)", manifest_text)
    if section is None:
        raise ValueError("Cargo.toml has no package section")
    updated, count = re.subn(r'(?m)^version\s*=\s*"[^"\n]+"', f'version = "{result["version"]}"', section[0])
    if count != 1:
        raise ValueError("Cargo.toml must contain one package version")
    manifest_path.write_text(manifest_text[:section.start()] + updated + manifest_text[section.end():])
    args.notes.write_text(release_notes(result["version"], previous, spec))
    output(release=True, **result)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error)) from error
