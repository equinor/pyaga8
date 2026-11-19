"""Checks of the API docstrings (Rust doc comments in src/lib.rs) and of the type stub.

The expected units are those of the aga8 crate 0.6.1, which pyaga8 wraps without unit
conversion: https://docs.rs/aga8/0.6.1/aga8/gerg2008/struct.Gerg2008.html and
https://docs.rs/aga8/0.6.1/aga8/detail/struct.Detail.html (field documentation).
"""

import ast
import inspect
from pathlib import Path

import pytest

import pyaga8

CLASSES = [pyaga8.Gerg2008, pyaga8.Detail, pyaga8.Composition]

# Unit text that the docstring of each property must contain (from docs.rs, aga8 0.6.1).
EXPECTED_UNITS = {
    "temperature": "in K.",
    "pressure": "in kPa.",
    "d": "in mol/l.",
    "z": "(dimensionless)",
    "mm": "in g/mol.",
    "dp_dd": "in kPa/(mol/l).",
    "d2p_dd2": "in kPa/(mol/l)^2.",
    "d2p_dtd": "in\nkPa/(mol/l)/K.",
    "dp_dt": "in kPa/K.",
    "u": "in J/mol.",
    "h": "in J/mol.",
    "s": "in J/(mol-K).",
    "cv": "in J/(mol-K).",
    "cp": "in J/(mol-K).",
    "w": "in m/s.",
    "g": "in J/mol.",
    "jt": "in K/kPa.",
    "kappa": "(dimensionless)",
}

COMPONENTS = [
    "methane", "nitrogen", "carbon_dioxide", "ethane", "propane", "isobutane", "n_butane",
    "isopentane", "n_pentane", "hexane", "heptane", "octane", "nonane", "decane", "hydrogen",
    "oxygen", "carbon_monoxide", "water", "hydrogen_sulfide", "helium", "argon",
]


def public_members(cls):
    return {name: value for name, value in vars(cls).items() if not name.startswith("_")}


def test_module_and_classes_have_docstrings():
    assert "aga8 0.6.1" in pyaga8.__doc__
    for cls in CLASSES:
        assert cls.__doc__ and "aga8 0.6.1" in cls.__doc__, cls.__name__


@pytest.mark.parametrize("cls", CLASSES, ids=lambda c: c.__name__)
def test_every_public_member_has_a_docstring(cls):
    missing = [name for name, value in public_members(cls).items() if not (value.__doc__ or "").strip()]
    assert missing == []


@pytest.mark.parametrize("cls", [pyaga8.Gerg2008, pyaga8.Detail], ids=lambda c: c.__name__)
def test_property_docstrings_state_the_aga8_units(cls):
    members = public_members(cls)
    properties = {name for name, value in members.items() if not callable(value)}
    assert properties == set(EXPECTED_UNITS)
    for name, unit_text in EXPECTED_UNITS.items():
        assert unit_text in members[name].__doc__, f"{cls.__name__}.{name}: {members[name].__doc__!r}"


def test_composition_components_are_documented_as_mole_fractions():
    members = public_members(pyaga8.Composition)
    assert set(members) == set(COMPONENTS)
    for name in COMPONENTS:
        label = "n-" + name[2:] if name.startswith("n_") else name.replace("_", " ")
        assert members[name].__doc__ == f"Mole fraction of {label} (write-only)."


@pytest.mark.parametrize("cls", [pyaga8.Gerg2008, pyaga8.Detail], ids=lambda c: c.__name__)
def test_only_pressure_temperature_and_d_are_writable(cls):
    obj = cls()
    writable = set()
    for name, value in public_members(cls).items():
        if callable(value):
            continue
        try:
            setattr(obj, name, 1.0)
        except AttributeError:
            continue
        writable.add(name)
    assert writable == {"pressure", "temperature", "d"}


def installed_stub_path():
    return Path(pyaga8.__file__).with_suffix(".pyi")


def stub_docstrings():
    """Map 'Class.member' (and 'Class') to the docstring in the installed stub."""
    tree = ast.parse(installed_stub_path().read_text(encoding="utf-8"))
    docs = {}
    for node in tree.body:
        if not isinstance(node, ast.ClassDef):
            continue
        docs[node.name] = ast.get_docstring(node)
        previous = None
        for item in node.body:
            if isinstance(item, ast.FunctionDef) and item.name != "__init__":
                is_setter = any(isinstance(d, ast.Attribute) and d.attr == "setter" for d in item.decorator_list)
                if not is_setter:
                    docs[f"{node.name}.{item.name}"] = ast.get_docstring(item)
            elif isinstance(item, ast.AnnAssign):
                previous = item.target.id
                continue
            elif isinstance(item, ast.Expr) and isinstance(item.value, ast.Constant) and previous:
                docs[f"{node.name}.{previous}"] = inspect.cleandoc(item.value.value)
            previous = None
    return docs


def test_installed_stub_docstrings_match_runtime():
    assert installed_stub_path().is_file()
    expected = {}
    for cls in CLASSES:
        expected[cls.__name__] = inspect.cleandoc(cls.__doc__)
        for name, value in public_members(cls).items():
            expected[f"{cls.__name__}.{name}"] = inspect.cleandoc(value.__doc__)
    assert stub_docstrings() == expected


def test_installed_package_is_marked_typed():
    assert (Path(pyaga8.__file__).parent / "py.typed").is_file()
