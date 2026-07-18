#!/usr/bin/env sh
set -eu

python3 - <<'PY'
from pathlib import Path

violations = []
for path in sorted(Path("src").rglob("*.rs")):
    for number, line in enumerate(path.read_bytes().splitlines(), 1):
        if len(line) > 100:
            violations.append(f"{path}:{number}: {len(line)} bytes")
if violations:
    raise SystemExit("error[rust-width]:\n" + "\n".join(violations))
PY
