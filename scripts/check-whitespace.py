#!/usr/bin/env python3
"""Check whitespace in every tracked or unignored repository file."""

from pathlib import Path
import subprocess

result = subprocess.run(
    ["git", "ls-files", "--cached", "--others", "--exclude-standard"],
    check=True,
    capture_output=True,
    text=True,
)
violations: list[str] = []
for name in result.stdout.splitlines():
    path = Path(name)
    data = path.read_bytes()
    if b"\0" in data:
        continue
    for number, line in enumerate(data.splitlines(keepends=True), 1):
        content = line.removesuffix(b"\n").removesuffix(b"\r")
        if content.endswith((b" ", b"\t")):
            violations.append(f"{path}:{number}: trailing whitespace")
    if data and not data.endswith(b"\n"):
        violations.append(f"{path}: missing final newline")

if violations:
    raise SystemExit("error[whitespace]:\n" + "\n".join(violations))
