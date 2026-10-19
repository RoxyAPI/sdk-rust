#!/usr/bin/env python3
import ast
import io
import pathlib
import re
import subprocess
import sys
import tokenize

ROOT = pathlib.Path(__file__).resolve().parents[1]
ALLOWED_HOSTS = {"roxyapi.com", "github.com", "raw.githubusercontent.com", "docs.rs", "crates.io", "rust-lang.org", "github.io"}
RULES = {
    "typography": r"[\u2018\u2019\u2013\u2014]|\s--\s|\b(?:it's|don't|doesn't|won't|we're|you're|let's)\b",
    "private path": r"/home/|~/per/|\bapp/src/|\bCLAUDE\.md\b|\bdocs/(?:todo|strategy|audits)/",
    "user reference": r"\bcustomers?\b",
    "dated claim": r"\b(?:verified|measured|checked|confirmed|corrected|retested)\b[^.]{0,60}\b20\d\d-\d\d-\d\d\b",
    "defect narrative": r"\b(?:used to|was wrong|the defect|before the fix|silently (?:dropped|lost|broke))\b",
    "external comparison": r"\b(?:verified|checked|compared|benchmarked)\s+against\s+(?!NASA\b)",
    "private surface": r"\badmin\s+(?:route|panel|dashboard)\b|\bprod(?:uction)?\s+(?:host|database|credentials)\b",
    "strategy": r"\b(?:distribution playbook|breadth edge|citation gravity|conversion endpoint)\b|\bso that\b[^.]{0,60}\b(?:cite|rank|recommend)\b",
    "access mechanics": r"\b(?:omit|omitting|without|missing|absent)\b[^.]{0,40}\b(?:api key|x-api-key|credential|bearer token)\b|\b(?:quota|allowance|free tier|rate limit)\b[^.]{0,40}\b(?:resets?|is counted|decrements?|per (?:ip|key|minute))\b",
}


def comment_lines(suffix, contents):
    if suffix == ".py":
        try:
            tree = ast.parse(contents)
        except SyntaxError:
            tree = None
        for node in ast.walk(tree) if tree else []:
            if isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
                for offset, line in enumerate(node.value.value.splitlines()):
                    yield node.lineno + offset, line
        try:
            for token in tokenize.generate_tokens(io.StringIO(contents).readline):
                if token.type == tokenize.COMMENT:
                    yield token.start[0], token.string
        except (tokenize.TokenError, IndentationError):
            pass
        return
    if suffix == ".rs":
        tokens = re.compile(r'r(?P<hashes>#{0,255})".*?"(?P=hashes)|\'(?:\\.|[^\'\\\n])\'|"(?:\\.|[^"\\])*"|(?P<line>//[^\n]*)|(?P<block>/\*)', re.S)
    else:
        tokens = re.compile(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|(?P<line>\#[^\n]*)', re.S)
    position = 0
    while match := tokens.search(contents, position):
        end = match.end()
        if suffix == ".rs" and match.lastgroup == "block":
            depth = 1
            for marker in re.finditer(r"/\*|\*/", contents[end:]):
                depth += 1 if marker[0] == "/*" else -1
                if depth == 0:
                    end += marker.end()
                    break
            else:
                end = len(contents)
        if match.lastgroup in {"line", "block"}:
            number = contents.count("\n", 0, match.start()) + 1
            for offset, line in enumerate(contents[match.start():end].splitlines()):
                yield number + offset, line
        position = end


def fenced_lines(fence, code):
    lines = comment_lines(fence[1], "\n".join(code)) if fence[1] else enumerate(code, 1)
    for offset, text in lines:
        yield fence[2] + offset, text


def prose_lines(path, contents):
    if path.suffix not in {".md", ".txt", ".cff"}:
        yield from comment_lines(path.suffix, contents)
        return
    fence = None
    code = []
    languages = {"rust": ".rs", "rs": ".rs", "python": ".py", "py": ".py", "bash": ".sh", "sh": ".sh", "shell": ".sh", "toml": ".toml", "yaml": ".yml", "yml": ".yml"}
    for number, line in enumerate(contents.splitlines(), 1):
        marker = re.match(r"\s*(`{3,}|~{3,})([\w+-]*)", line)
        if fence is None:
            if marker:
                fence = (marker[1], languages.get(marker[2]), number)
            else:
                yield number, line
        elif marker and marker[1][0] == fence[0][0] and len(marker[1]) >= len(fence[0]) and not marker[2]:
            yield from fenced_lines(fence, code)
            fence = None
            code = []
        else:
            code.append(line)
    if fence is not None:
        yield from fenced_lines(fence, code)


def violations(path, contents):
    for number, line in prose_lines(path, contents):
        for label, pattern in RULES.items():
            if re.search(pattern, line, re.I):
                yield number, label
        for host in re.findall(r"https?://([^/\s)\]>]+)", line):
            host = host.lower().split(":")[0]
            if not any(host == allowed or host.endswith("." + allowed) for allowed in ALLOWED_HOSTS):
                yield number, "external domain"


def main():
    files = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT
    ).decode().split("\0")
    failures = []
    for name in sorted(set(filter(None, files))):
        path = ROOT / name
        if name.startswith(("specs/", "src/generated/", "tools/generate/target/")) or path.name in {"Cargo.lock", "LICENSE"} or not path.is_file():
            continue
        try:
            contents = path.read_text()
        except UnicodeDecodeError:
            continue
        for line, label in violations(path, contents):
            failures.append(f"{name}:{line}: {label}")
    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1
    print("Public content checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
