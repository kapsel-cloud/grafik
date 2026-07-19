#!/usr/bin/env python3
"""Package an attributable, deterministic browser proof in dist/."""

from hashlib import sha256
import json
from pathlib import Path
import platform
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent.parent
WEB = ROOT / "web"
DIST = ROOT / "dist"
BROWSER_FILES = (
    "index.html",
    "main.js",
    "style.css",
    "svg-adapter.js",
    "pkg/grafik.js",
    "pkg/grafik_bg.wasm",
)


def output(*command: str) -> str:
    return subprocess.run(
        command,
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


missing = [name for name in BROWSER_FILES if not (WEB / name).is_file()]
if missing:
    raise SystemExit(f"error[artifact]: missing generated input: {', '.join(missing)}")

shutil.rmtree(DIST, ignore_errors=True)
for name in BROWSER_FILES:
    destination = DIST / name
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(WEB / name, destination)

tree_state = "dirty" if output("git", "status", "--porcelain") else "clean"
manifest = {
    "artifact_format": 1,
    "classification": {
        "experimental": True,
        "intended_use": "browser proof; not production",
        "production_ready": False,
    },
    "source": {
        "revision": output("git", "rev-parse", "HEAD"),
        "tree_state": tree_state,
    },
    "tools": {
        "cargo": output("cargo", "--version"),
        "node": output("node", "--version"),
        "python": platform.python_version(),
        "rustc": output("rustc", "--version"),
        "wasm-bindgen": output("wasm-bindgen", "--version"),
    },
}
manifest_path = DIST / "artifact.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")

checksum_path = DIST / "SHA256SUMS"
artifact_files = sorted(
    path for path in DIST.rglob("*") if path.is_file() and path != checksum_path
)
checksum_lines = [
    f"{sha256(path.read_bytes()).hexdigest()}  {path.relative_to(DIST).as_posix()}"
    for path in artifact_files
]
checksum_path.write_text("\n".join(checksum_lines) + "\n", encoding="utf-8")
