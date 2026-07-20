#!/usr/bin/env python3
"""Validate the attributable browser artifact through its published files."""

from hashlib import sha256
import json
from pathlib import Path
import subprocess

DIST = Path("dist")
REQUIRED_BROWSER_FILES = {
    "index.html",
    "lab.html",
    "lab.js",
    "lab.css",
    "scene-adapter.js",
    "style.css",
    "pkg/grafik.js",
    "pkg/grafik_bg.wasm",
}


def fail(message: str) -> None:
    raise SystemExit(f"error[artifact]: {message}")


manifest_path = DIST / "artifact.json"
checksum_path = DIST / "SHA256SUMS"
if not manifest_path.is_file() or not checksum_path.is_file():
    fail("artifact.json and SHA256SUMS must exist")

manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
if manifest.get("artifact_format") != 1:
    fail("artifact_format must be 1")
if manifest.get("classification") != {
    "experimental": True,
    "production_ready": False,
    "intended_use": "browser proof; not production",
}:
    fail("classification must state the experimental, non-production boundary")

revision = subprocess.run(
    ["git", "rev-parse", "HEAD"],
    check=True,
    capture_output=True,
    text=True,
).stdout.strip()
if manifest.get("source", {}).get("revision") != revision:
    fail("source revision must match HEAD")
if manifest.get("source", {}).get("tree_state") not in {"clean", "dirty"}:
    fail("source tree state must be explicit")

expected_tools = {"cargo", "node", "python", "rustc", "wasm-bindgen"}
if set(manifest.get("tools", {})) != expected_tools:
    fail("tool versions must cover cargo, node, python, rustc, and wasm-bindgen")

artifact_files = {
    path.relative_to(DIST).as_posix()
    for path in DIST.rglob("*")
    if path.is_file() and path != checksum_path
}
missing = REQUIRED_BROWSER_FILES - artifact_files
if missing:
    fail(f"missing browser files: {', '.join(sorted(missing))}")

expected_lines = []
for name in sorted(artifact_files):
    digest = sha256((DIST / name).read_bytes()).hexdigest()
    expected_lines.append(f"{digest}  {name}")
actual_lines = checksum_path.read_text(encoding="utf-8").splitlines()
if actual_lines != expected_lines:
    fail("SHA256SUMS must exactly cover every other artifact file in sorted order")

print("Grafik browser artifact passed")
