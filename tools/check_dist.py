"""Check that a dist/ folder holds exactly the expected release files of pyaga8.

Expected: one sdist and 20 wheels, CPython 3.10-3.14 for each of the four platforms
Linux x86_64 and aarch64 (manylinux2014), Windows x64 and macOS arm64. The version is read
from Cargo.toml.

Usage::

    python tools/check_dist.py dist
"""

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

PYTHON_TAGS = ["cp310", "cp311", "cp312", "cp313", "cp314"]

PLATFORM_TAGS = [
    "manylinux_2_17_x86_64.manylinux2014_x86_64",
    "manylinux_2_17_aarch64.manylinux2014_aarch64",
    "win_amd64",
    "macosx_11_0_arm64",
]


def expected_files(version):
    """Return the set of file names that a release of ``version`` must contain."""
    names = {f"pyaga8-{version}.tar.gz"}
    for python_tag in PYTHON_TAGS:
        for platform_tag in PLATFORM_TAGS:
            names.add(f"pyaga8-{version}-{python_tag}-{python_tag}-{platform_tag}.whl")
    return names


def main(argv):
    if len(argv) != 2:
        print(__doc__)
        return 2
    dist = Path(argv[1])
    with open(ROOT / "Cargo.toml", "rb") as f:
        version = tomllib.load(f)["package"]["version"]
    expected = expected_files(version)
    found = {p.name for p in dist.iterdir() if p.is_file()}
    missing = sorted(expected - found)
    unexpected = sorted(found - expected)
    for name in sorted(found):
        print(("  ok         " if name in expected else "  UNEXPECTED ") + name)
    for name in missing:
        print("  MISSING    " + name)
    if missing or unexpected:
        print(f"FAILED: {len(missing)} missing, {len(unexpected)} unexpected (expected {len(expected)} files)")
        return 1
    print(f"OK: {len(found)} files ({len(expected) - 1} wheels + 1 sdist) for version {version}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
