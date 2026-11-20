"""Tests of tools/release_check.py, the version check of the release workflow.

The tool is part of the repository, not of the package. The tests are skipped when tests/ is run
from a copy outside the checkout (the wheel and sdist tests in CI).
"""

import importlib.util
import io
import sys
from pathlib import Path

import pytest

TOOL = Path(__file__).resolve().parents[1] / "tools" / "release_check.py"
if not TOOL.is_file():
    pytest.skip("tools/release_check.py is only available in the repository checkout", allow_module_level=True)

_spec = importlib.util.spec_from_file_location("release_check", TOOL)
release_check = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(release_check)

# The release tags of equinor/pyaga8 on 2026-11-20 (git ls-remote --tags origin).
REPO_TAGS = [f"0.1.{patch}" for patch in range(4, 19)]


def test_parse_version():
    assert release_check.parse_version("0.2.0") == (0, 2, 0)
    assert release_check.parse_version("10.0.12") == (10, 0, 12)


@pytest.mark.parametrize("text", [
    "0.2", "0.2.0.1", "v0.2.0", "0.2.0-rc1", "0.2.0rc1", "00.2.0", "0.02.0", "0.2.00", " 0.2.0", "", "a.b.c",
])
def test_parse_version_rejects_anything_but_major_minor_patch(text):
    with pytest.raises(ValueError):
        release_check.parse_version(text)


def test_highest_release_tag_of_repo_tags_is_numeric():
    # String comparison would give 0.1.9; numerically 0.1.18 is the highest.
    assert release_check.highest_release_tag(REPO_TAGS) == (0, 1, 18)


def test_highest_release_tag_ignores_non_release_tags_and_blanks():
    assert release_check.highest_release_tag(["", "  0.1.4 ", "latest", "v9.9.9", "docs-1"]) == (0, 1, 4)
    assert release_check.highest_release_tag([]) is None
    assert release_check.highest_release_tag(["v1.0.0", "nightly"]) is None


@pytest.mark.parametrize("bad_tag", ["0.1", "0.1.19-rc1", "1.0.0.0", "01.0.0"])
def test_highest_release_tag_rejects_malformed_digit_tags(bad_tag):
    with pytest.raises(ValueError):
        release_check.highest_release_tag(REPO_TAGS + [bad_tag])


def test_first_release_0_2_0_is_accepted_with_repo_tags():
    assert release_check.check_release_version("0.2.0", REPO_TAGS) == "0.2.0"
    assert release_check.check_release_version("0.1.19", REPO_TAGS) == "0.1.19"
    assert release_check.check_release_version("1.0.0", REPO_TAGS) == "1.0.0"


@pytest.mark.parametrize("version", ["0.1.18", "0.1.9", "0.1.4", "0.0.1"])
def test_version_not_above_highest_tag_is_rejected(version):
    with pytest.raises(ValueError, match="not higher than the highest release tag 0.1.18"):
        release_check.check_release_version(version, REPO_TAGS)


def test_invalid_cargo_version_is_rejected():
    with pytest.raises(ValueError, match="MAJOR.MINOR.PATCH"):
        release_check.check_release_version("0.2.0-beta.1", REPO_TAGS)


def test_any_version_is_higher_than_no_tags():
    assert release_check.check_release_version("0.0.1", []) == "0.0.1"


def _run_main(monkeypatch, capsys, tmp_path, cargo_version, tags):
    pytest.importorskip("tomllib", reason="reading Cargo.toml needs Python 3.11+")
    cargo_toml = tmp_path / "Cargo.toml"
    cargo_toml.write_text(f'[package]\nname = "pyaga8"\nversion = "{cargo_version}"\n', encoding="utf-8")
    monkeypatch.setattr(sys, "stdin", io.StringIO("\n".join(tags) + "\n"))
    exit_code = release_check.main(["--cargo-toml", str(cargo_toml)])
    return exit_code, capsys.readouterr()


def test_main_prints_version_when_releasable(monkeypatch, capsys, tmp_path):
    exit_code, output = _run_main(monkeypatch, capsys, tmp_path, "0.2.0", REPO_TAGS)
    assert exit_code == 0
    assert output.out == "0.2.0\n"


def test_main_fails_when_version_is_already_tagged(monkeypatch, capsys, tmp_path):
    exit_code, output = _run_main(monkeypatch, capsys, tmp_path, "0.1.18", REPO_TAGS)
    assert exit_code == 1
    assert output.out == ""
    assert "not higher than the highest release tag 0.1.18" in output.err


def test_repository_cargo_toml_version_is_major_minor_patch():
    pytest.importorskip("tomllib", reason="reading Cargo.toml needs Python 3.11+")
    version = release_check.read_cargo_version(TOOL.parents[1] / "Cargo.toml")
    release_check.parse_version(version)
