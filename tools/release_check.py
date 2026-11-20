"""Check that the version in Cargo.toml can be released.

Used by the release workflow, ``.github/workflows/release.yml``. pyaga8 takes its version from
``Cargo.toml`` (``[package] version``); releases are tagged with that version, unprefixed
(``MAJOR.MINOR.PATCH``, for example ``0.1.18``). The version in ``Cargo.toml`` can be released only
if

- it is exactly ``MAJOR.MINOR.PATCH`` (digits only, no leading zeros, no pre-release suffix), and
- it is strictly higher than the highest existing release tag, compared numerically
  (``0.1.10`` is higher than ``0.1.9``). This also rules out a version that is already tagged.

Usage, with the git tags on standard input, one per line::

    git tag --list | python tools/release_check.py --cargo-toml Cargo.toml

prints the version, for example ``0.2.0``, and exits with 0; otherwise prints the reason and exits
with 1.

Tags that do not start with a digit are not release tags and are ignored. A tag that does, but is
not exactly ``MAJOR.MINOR.PATCH``, is an error rather than being skipped: it was most likely meant
as a release, and skipping it could make the next release go backwards.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

RELEASE_VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)")
LOOKS_LIKE_RELEASE_TAG = re.compile(r"[0-9]")


def parse_version(text: str) -> tuple[int, int, int]:
    """Read a release version.

    Parameters
    ----------
    text : str
        ``MAJOR.MINOR.PATCH`` with no leading zeros, for example ``"0.2.0"``.

    Returns
    -------
    tuple of int
        ``(major, minor, patch)``.

    Raises
    ------
    ValueError
        If ``text`` is not exactly ``MAJOR.MINOR.PATCH``.
    """
    match = RELEASE_VERSION.fullmatch(text)
    if match is None:
        raise ValueError(f"{text!r} is not a release version of the form MAJOR.MINOR.PATCH, e.g. 0.2.0")
    return int(match.group(1)), int(match.group(2)), int(match.group(3))


def highest_release_tag(tags: list[str]) -> tuple[int, int, int] | None:
    """Find the highest released version among a list of git tags.

    Parameters
    ----------
    tags : list of str
        Git tag names. Blank entries and surrounding whitespace are ignored. Tags that do not start
        with a digit are not release tags and are skipped.

    Returns
    -------
    tuple of int or None
        ``(major, minor, patch)`` of the highest release tag, compared numerically, or ``None`` if
        there are no release tags.

    Raises
    ------
    ValueError
        If a tag starts with a digit but is not exactly ``MAJOR.MINOR.PATCH``.
    """
    versions = []
    for tag in tags:
        tag = tag.strip()
        if LOOKS_LIKE_RELEASE_TAG.match(tag):
            versions.append(parse_version(tag))
    return max(versions) if versions else None


def check_release_version(cargo_version: str, tags: list[str]) -> str:
    """Check that ``cargo_version`` is a valid release version above every release tag.

    Parameters
    ----------
    cargo_version : str
        The version in ``Cargo.toml``.
    tags : list of str
        The existing git tag names.

    Returns
    -------
    str
        ``cargo_version``, unchanged.

    Raises
    ------
    ValueError
        If ``cargo_version`` is not ``MAJOR.MINOR.PATCH``, if a tag is malformed (see
        :func:`highest_release_tag`), or if ``cargo_version`` is not strictly higher than the
        highest release tag.
    """
    version = parse_version(cargo_version)
    highest = highest_release_tag(tags)
    if highest is not None and version <= highest:
        highest_text = ".".join(str(part) for part in highest)
        raise ValueError(
            f"Cargo.toml version {cargo_version} is not higher than the highest release tag "
            f"{highest_text}. Bump the version in Cargo.toml and Cargo.lock in a pull request first."
        )
    return cargo_version


def read_cargo_version(cargo_toml: Path) -> str:
    """Return ``[package] version`` from a ``Cargo.toml`` file.

    Needs Python 3.11 or later (``tomllib``); the release workflow runs it with 3.13. The import is
    here so that the other functions, and their tests, also work on Python 3.10.
    """
    import tomllib

    with open(cargo_toml, "rb") as f:
        return tomllib.load(f)["package"]["version"]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--cargo-toml", type=Path, default=Path("Cargo.toml"))
    args = parser.parse_args(argv)

    tags = sys.stdin.read().splitlines()
    try:
        version = check_release_version(read_cargo_version(args.cargo_toml), tags)
    except ValueError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print(version)
    return 0


if __name__ == "__main__":
    sys.exit(main())
