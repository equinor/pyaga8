"""Checks of __version__ and of where pyaga8 is imported from."""

import importlib.metadata
import os
import sys
from pathlib import Path

import pytest

import pyaga8


def test_version_equals_installed_metadata():
    assert pyaga8.__version__ == importlib.metadata.version("pyaga8")


@pytest.mark.skipif(
    os.environ.get("PYAGA8_EXPECT_INSTALLED") != "1",
    reason="only checked when testing a built wheel or sdist (PYAGA8_EXPECT_INSTALLED=1)",
)
def test_imported_from_installed_distribution():
    """Fail if the tests run against anything else than the installed distribution.

    CI sets PYAGA8_EXPECT_INSTALLED=1 when it tests a built wheel or sdist in a clean virtual
    environment, with a copy of tests/ outside the checkout.
    """
    module_path = Path(pyaga8.__file__).resolve()
    venv_path = Path(sys.prefix).resolve()
    assert venv_path in module_path.parents, f"{module_path} is not inside {venv_path}"
    workspace = os.environ.get("GITHUB_WORKSPACE")
    if workspace:
        workspace_path = Path(workspace).resolve()
        assert workspace_path not in module_path.parents, (
            f"{module_path} is inside the checkout {workspace_path}"
        )
