# pyaga8

Python bindings for the AGA8 GERG-2008 and DETAIL gas property equations, built on the Rust [`aga8`](https://crates.io/crates/aga8) port of NIST's AGA8 code.

GitHub: https://github.com/equinor/pyaga8

## Description

`pyaga8` is a low-level Python interface to the AGA8 calculations. It exposes the underlying GERG-2008 and DETAIL state objects and methods directly, with Rust used for performance via `pyo3`.

Higher-level libraries such as [`pvtlib`](https://pypi.org/project/pvtlib/) build on top of `pyaga8` to provide workflows such as PT, PH, PS and rhoT calculations:
https://github.com/equinor/pvtlib/blob/main/examples/gas_properties_from_aga8.py

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
