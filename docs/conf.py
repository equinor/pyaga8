"""Sphinx configuration for the pyaga8 documentation site.

Build locally (from the repository root, with the extension built and installed in the
active environment, e.g. ``maturin develop``)::

    pip install .[docs]
    sphinx-build -W --keep-going -b html docs docs/_build/html

The landing page is README.md. The API reference is generated from the docstrings of the
compiled extension module, which come from the Rust doc comments in src/lib.rs.
"""

import pyaga8

project = "pyaga8"
author = "Roy Vegard Ovesen, Christian Hågenvik"
copyright = "2025, Roy Vegard Ovesen & Christian Hågenvik"
release = pyaga8.__version__
version = release

extensions = [
    "myst_parser",
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
]

source_suffix = {
    ".rst": "restructuredtext",
    ".md": "markdown",
}

exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

autosummary_generate = True
autodoc_default_options = {
    "members": True,
    "member-order": "bysource",
}
napoleon_google_docstring = False
napoleon_numpy_docstring = True

myst_heading_anchors = 3

html_theme = "furo"
html_title = f"pyaga8 {release}"
