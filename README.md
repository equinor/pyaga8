# pyaga8

Python bindings for the AGA8 GERG-2008 and DETAIL gas property equations, built on the Rust [`aga8`](https://crates.io/crates/aga8) port of NIST's AGA8 code.

GitHub: https://github.com/equinor/pyaga8

## Description

`pyaga8` is a low-level Python interface to the AGA8 calculations. It exposes the underlying GERG-2008 and DETAIL state objects and methods directly, with Rust used for performance via `pyo3`.

Higher-level libraries such as [`pvtlib`](https://pypi.org/project/pvtlib/) build on top of `pyaga8` to provide workflows such as PT, PH, PS and rhoT calculations:
https://github.com/equinor/pvtlib/tree/main/examples/01%20Gas%20Properties%20from%20AGA8

Documentation: https://equinor.github.io/pyaga8/

## Units

`pyaga8` does no unit conversion. Inputs and outputs use the units of the Rust
[`aga8`](https://docs.rs/aga8/0.6.1/aga8/) crate 0.6.1: temperature in K, pressure in kPa,
molar concentration (`d`) in mol/l, molar mass in g/mol, internal energy, enthalpy and Gibbs
energy in J/mol, entropy and heat capacities in J/(mol-K), speed of sound in m/s and
Joule-Thomson coefficient in K/kPa. Compositions are mole fractions. The docstrings
(`help(pyaga8.Gerg2008)`) list the unit of every property.

## Installation

```sh
pip install pyaga8
```

## Minimal usage

```python
import pyaga8

detail = pyaga8.Detail()
comp = pyaga8.Composition()
comp.methane = 0.9
comp.nitrogen = 0.1

detail.set_composition(comp)
detail.temperature = 400.0
detail.pressure = 50_000.0
detail.calc_density()

print(detail.d)
```

## Error handling

Python-facing API methods raise normal Python exceptions when inputs or state are invalid; they do not intentionally panic the Python interpreter.

- `set_composition(...)` raises `ValueError` for invalid mixtures, for example an empty composition or a sum that is not close enough to `1.0`.
- `calc_density(...)` / `calc_density()` raises ordinary Python exceptions based on the underlying failure, typically `ValueError` or `RuntimeError`.

`pyaga8` is intentionally a lower-level library. Higher-level packages may choose to translate selected failures to `NaN` or other domain-specific behavior, but that translation is not part of `pyaga8` itself.
