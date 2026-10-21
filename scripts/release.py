#!/usr/bin/env python3
"""Plan releases from the shipped crate contents and verify already published package contents."""

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
COPY_KEYS = {"description", "summary", "title", "example", "examples", "externalDocs", "tags", "deprecated"}
MAP_KEYS = {"paths", "properties", "patternProperties", "schemas", "$defs", "headers", "content", "responses", "mapping"}
CONSTRUCTOR = re.compile(r"^impl ([A-Za-z_][A-Za-z0-9_]*) \{\n(?:[ \t]*///[^\n]*\n)*[ \t]*pub fn new\(([^)]*)\) -> Self \{", re.M)


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


def additive(old, new):
    """Whether the contract gained paths, schemas, properties, parameters or enum values."""
    def grew(before, after, key="", mapping=False):
        if isinstance(before, dict) and isinstance(after, dict):
            return bool(after.keys() - before.keys()) or any(grew(before[name], after[name], name, not mapping and name in MAP_KEYS) for name in before.keys() & after.keys())
        if isinstance(before, list) and isinstance(after, list):
            if key == "required":
                return False
            if key == "enum":
                return any(value not in before for value in after)
            if key == "parameters":
                index = lambda items: {(item.get("in"), item.get("name"), item.get("$ref")): item for item in items if isinstance(item, dict)}
                previous, current = index(before), index(after)
                return bool(current.keys() - previous.keys()) or any(grew(previous[name], current[name]) for name in previous.keys() & current.keys())
            return len(after) > len(before) or any(grew(a, b) for a, b in zip(before, after))
        return False

    surface = lambda document: contract({key: (document or {}).get(key, {}) for key in ["paths", "components"]}, True)
    return grew(surface(old), surface(new), mapping=True)


def rust_api_break(baseline):
    """Ask cargo-semver-checks whether the crate breaks the public API of a published version."""
    result = subprocess.run(["cargo", "semver-checks", "check-release", "--baseline-version", baseline, "--release-type", "minor"])
    if result.returncode not in {0, 100}:
        raise ValueError(f"cargo-semver-checks could not compare against {baseline} (exit {result.returncode})")
    return result.returncode == 100


def generated_sources(tag=None):
    """Generated Rust sources by file name, from a release tag or from the working tree."""
    if tag is None:
        return {path.name: path.read_text() for path in sorted(Path("src/generated").glob("*.rs"))}
    names = subprocess.check_output(["git", "ls-tree", "--name-only", f"{tag}:src/generated"], text=True).split()
    return {name: subprocess.check_output(["git", "show", f"{tag}:src/generated/{name}"], text=True) for name in names if name.endswith(".rs")}


def parameters(text):
    """Ordered (name, type) pairs of a constructor parameter list, split on top-level commas."""
    items, depth, current = [], 0, ""
    for character in text:
        depth += (character in "<([") - (character in ">)]")
        if character == "," and depth == 0:
            items.append(current)
            current = ""
        else:
            current += character
    pairs = []
    for item in [*items, current]:
        item = " ".join(item.split())
        if not item:
            continue
        name, separator, kind = item.partition(":")
        if not separator or not name.strip().removeprefix("r#").isidentifier() or not kind.strip():
            raise ValueError(f"Cannot parse generated constructor parameter {item!r}")
        pairs.append((name.strip(), kind.strip()))
    return tuple(pairs)


def constructors(sources):
    """Every generated `pub fn new` signature by type; fails when any constructor escapes the expected format."""
    found = {}
    for file, text in sources.items():
        matches = CONSTRUCTOR.findall(text)
        if len(matches) != text.count("pub fn new("):
            raise ValueError(f"Cannot parse every generated constructor in src/generated/{file}; the constructor guard in scripts/release.py must learn the new format")
        for name, signature in matches:
            if name in found:
                raise ValueError(f"Generated type {name} has more than one constructor")
            found[name] = parameters(signature)
    return found


def constructor_breaks(previous, current):
    """Existing constructors whose ordered parameter names or types changed, or that disappeared.

    Same-typed parameters swapped by a reordered required list still compile for callers and send swapped
    values, which cargo-semver-checks cannot see. A constructor on a new type is additive.
    """
    render = lambda signature: ", ".join(f"{name}: {kind}" for name, kind in signature)
    return [
        f"{name}::new({render(signature)}) " + (f"is now {name}::new({render(current[name])})" if name in current else "was removed")
        for name, signature in sorted(previous.items()) if current.get(name) != signature
    ]


def plan(change, manifest_version, last_version, published_versions, event, exact="", bump="", detail=""):
    """Choose the release version for a change level of patch, minor or major."""
    semver(manifest_version)
    if exact and bump:
        raise ValueError("Choose an exact version or a version bump, not both")
    if bump and bump not in {"patch", "minor", "major"}:
        raise ValueError("Version bump must be patch, minor, or major")
    if change == "major" and (event != "workflow_dispatch" or (not exact and bump != "major")):
        raise ValueError("The public Rust API changed incompatibly since the last published version. Scheduled releases never publish a major version; dispatch the release with version_bump=major after reviewing the change" + (f"\n{detail}" if detail else ""))
    floor = max([manifest_version, *published_versions, *([last_version] if last_version else [])], key=semver)
    if exact:
        semver(exact)
        if (last_version and semver(exact) <= semver(last_version)) or semver(exact) < semver(floor):
            raise ValueError("Exact version must exceed the last release and not precede the manifest or registry")
        if last_version and semver(exact) < semver(bump_version(last_version, change)):
            raise ValueError(f"This change requires at least a {change} release")
        target = exact
    elif last_version is None and not published_versions and not bump:
        target = manifest_version
    else:
        requested = bump or change
        if change == "minor" and requested == "patch":
            raise ValueError("Additive change requires at least a minor release")
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


def registry_crate(name, version):
    with request(f"https://crates.io/api/v1/crates/{name}/{version}/download") as response:
        return response.read()


def local_crate(name, version):
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version=1"]))
    return (Path(metadata["target_directory"]) / "package" / f"{name}-{version}.crate").read_bytes()


def shipped(files, name):
    """Package members with the crate version masked, so equal contents compare equal across versions."""
    masked = dict(files)
    for manifest in ["Cargo.toml", "Cargo.toml.orig"]:
        if manifest in masked:
            masked[manifest] = re.sub(rb'(?ms)(^\[package\]\n(?:(?!^\[).)*?^version\s*=\s*)"[^"\n]*"', rb'\1"0.0.0"', masked[manifest], count=1)
    if "Cargo.lock" in masked:
        masked["Cargo.lock"] = re.sub(rb'(name = "' + re.escape(name.encode()) + rb'"\nversion = )"[^"\n]*"', rb'\1"0.0.0"', masked["Cargo.lock"])
    return masked


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
    name = package["name"]
    if args.command == "publication":
        versions = registry_versions(name)
        version = package["version"]
        if version in versions:
            verify_archive(local_crate(name, version), registry_crate(name, version))
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
    versions = registry_versions(name)
    change, breaks = "patch", []
    if versions:
        baseline = max(versions, key=semver)
        subprocess.run(["cargo", "package", "--locked", "--allow-dirty", "--no-verify", "--quiet"], check=True)
        current = shipped(archive_files(local_crate(name, package["version"])), name)
        if current == shipped(archive_files(registry_crate(name, baseline)), name):
            if args.event != "workflow_dispatch":
                output(release=False)
                return
        else:
            if last:
                breaks = constructor_breaks(constructors(generated_sources(last)), constructors(generated_sources()))
            if breaks or rust_api_break(baseline):
                change = "major"
            elif additive(previous, spec):
                change = "minor"
    result = plan(change, package["version"], last[1:] if last else None, versions, args.event, args.version, args.bump, "\n".join(breaks))
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
