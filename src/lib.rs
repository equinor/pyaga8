// MIT License

// Copyright (c) 2025 Roy Vegard Ovesen & Christian Hågenvik

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use aga8::composition::CompositionError;
use aga8::{composition, detail, gerg2008, DensityError};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

/// Convert Rust-side composition errors into ordinary Python exceptions.
fn composition_error_to_py_err(err: CompositionError) -> PyErr {
    match err {
        CompositionError::Empty => PyValueError::new_err("composition cannot be empty"),
        CompositionError::BadSum => {
            PyValueError::new_err("composition sum must be within 0.01 of 1.0")
        }
        CompositionError::Ok => PyRuntimeError::new_err("unexpected composition error state"),
    }
}

/// Convert Rust-side density errors into ordinary Python exceptions.
fn density_error_to_py_err(err: DensityError) -> PyErr {
    match err {
        DensityError::PressureTooLow => PyValueError::new_err("pressure is too low for density calculation"),
        DensityError::IterationFail => {
            PyRuntimeError::new_err("density calculation failed to converge")
        }
        DensityError::Ok => PyRuntimeError::new_err("unexpected density error state"),
    }
}

#[pyclass]
struct Gerg2008 {
    inner: gerg2008::Gerg2008,
}

#[pymethods]
impl Gerg2008 {
    #[new]
    fn new() -> Self {
        Gerg2008 {
            inner: gerg2008::Gerg2008::new(),
        }
    }

    // Properties
    #[setter]
    fn set_pressure(&mut self, pressure: f64) -> PyResult<()> {
        self.inner.p = pressure;
        Ok(())
    }

    #[getter]
    fn get_pressure(&self) -> PyResult<f64> {
        Ok(self.inner.p)
    }

    #[setter]
    fn set_temperature(&mut self, temperature: f64) -> PyResult<()> {
        self.inner.t = temperature;
        Ok(())
    }

    #[getter]
    fn get_temperature(&self) -> PyResult<f64> {
        Ok(self.inner.t)
    }

    #[setter]
    fn set_d(&mut self, d: f64) {
        self.inner.d = d;
    }

    #[getter]
    fn get_d(&self) -> f64 {
        self.inner.d
    }

    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z
    }

    #[getter]
    fn get_mm(&self) -> f64 {
        self.inner.mm
    }

    #[getter]
    fn get_dp_dd(&self) -> f64 {
        self.inner.dp_dd
    }

    #[getter]
    fn get_d2p_dd2(&self) -> f64 {
        self.inner.d2p_dd2
    }

    #[getter]
    fn get_d2p_dtd(&self) -> f64 {
        self.inner.d2p_dtd
    }

    #[getter]
    fn get_dp_dt(&self) -> f64 {
        self.inner.dp_dt
    }

    #[getter]
    fn get_u(&self) -> f64 {
        self.inner.u
    }

    #[getter]
    fn get_h(&self) -> f64 {
        self.inner.h
    }

    #[getter]
    fn get_s(&self) -> f64 {
        self.inner.s
    }

    #[getter]
    fn get_cv(&self) -> f64 {
        self.inner.cv
    }

    #[getter]
    fn get_cp(&self) -> f64 {
        self.inner.cp
    }

    #[getter]
    fn get_w(&self) -> f64 {
        self.inner.w
    }

    #[getter]
    fn get_g(&self) -> f64 {
        self.inner.g
    }

    #[getter]
    fn get_jt(&self) -> f64 {
        self.inner.jt
    }

    #[getter]
    fn get_kappa(&self) -> f64 {
        self.inner.kappa
    }

    // Functions
    fn calc_density(&mut self, flag: i32) -> PyResult<()> {
        self.inner.density(flag).map_err(density_error_to_py_err)
    }

    fn calc_pressure(&mut self) -> f64 {
        self.inner.pressure()
    }

    fn calc_properties(&mut self) {
        self.inner.properties();
    }

    fn calc_molar_mass(&mut self) {
        self.inner.molar_mass();
    }

    fn set_composition(&mut self, comp: &Composition) -> PyResult<()> {
        self.inner
            .set_composition(&comp.inner)
            .map_err(composition_error_to_py_err)
    }
}

#[pyclass]
struct Detail {
    inner: detail::Detail,
}

#[pymethods]
impl Detail {
    #[new]
    fn new() -> Self {
        Detail {
            inner: detail::Detail::new(),
        }
    }

    // Properties
    #[setter]
    fn set_pressure(&mut self, pressure: f64) -> PyResult<()> {
        self.inner.p = pressure;
        Ok(())
    }

    #[getter]
    fn get_pressure(&self) -> PyResult<f64> {
        Ok(self.inner.p)
    }

    #[setter]
    fn set_temperature(&mut self, temperature: f64) -> PyResult<()> {
        self.inner.t = temperature;
        Ok(())
    }

    #[getter]
    fn get_temperature(&self) -> PyResult<f64> {
        Ok(self.inner.t)
    }

    #[setter]
    fn set_d(&mut self, d: f64) {
        self.inner.d = d;
    }

    #[getter]
    fn get_d(&self) -> f64 {
        self.inner.d
    }

    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z
    }

    #[getter]
    fn get_mm(&self) -> f64 {
        self.inner.mm
    }

    #[getter]
    fn get_dp_dd(&self) -> f64 {
        self.inner.dp_dd
    }

    #[getter]
    fn get_d2p_dd2(&self) -> f64 {
        self.inner.d2p_dd2
    }

    #[getter]
    fn get_d2p_dtd(&self) -> f64 {
        self.inner.d2p_dtd
    }

    #[getter]
    fn get_dp_dt(&self) -> f64 {
        self.inner.dp_dt
    }

    #[getter]
    fn get_u(&self) -> f64 {
        self.inner.u
    }

    #[getter]
    fn get_h(&self) -> f64 {
        self.inner.h
    }

    #[getter]
    fn get_s(&self) -> f64 {
        self.inner.s
    }

    #[getter]
    fn get_cv(&self) -> f64 {
        self.inner.cv
    }

    #[getter]
    fn get_cp(&self) -> f64 {
        self.inner.cp
    }

    #[getter]
    fn get_w(&self) -> f64 {
        self.inner.w
    }

    #[getter]
    fn get_g(&self) -> f64 {
        self.inner.g
    }

    #[getter]
    fn get_jt(&self) -> f64 {
        self.inner.jt
    }

    #[getter]
    fn get_kappa(&self) -> f64 {
        self.inner.kappa
    }

    // Functions
    fn calc_density(&mut self) -> PyResult<()> {
        self.inner.density().map_err(density_error_to_py_err)
    }

    fn calc_pressure(&mut self) -> f64 {
        self.inner.pressure()
    }

    fn calc_properties(&mut self) {
        self.inner.properties();
    }

    fn calc_molar_mass(&mut self) {
        self.inner.molar_mass();
    }

    fn set_composition(&mut self, comp: &Composition) -> PyResult<()> {
        self.inner
            .set_composition(&comp.inner)
            .map_err(composition_error_to_py_err)
    }
}

#[pyclass]
struct Composition {
    inner: composition::Composition,
}

#[pymethods]
impl Composition {
    #[new]
    fn new() -> Self {
        Self {
            inner: { Default::default() },
        }
    }

    #[setter]
    fn set_methane(&mut self, value: f64) {
        self.inner.methane = value;
    }

    #[setter]
    fn set_nitrogen(&mut self, value: f64) {
        self.inner.nitrogen = value;
    }

    #[setter]
    fn set_carbon_dioxide(&mut self, value: f64) {
        self.inner.carbon_dioxide = value;
    }

    #[setter]
    fn set_ethane(&mut self, value: f64) {
        self.inner.ethane = value;
    }

    #[setter]
    fn set_propane(&mut self, value: f64) {
        self.inner.propane = value;
    }

    #[setter]
    fn set_isobutane(&mut self, value: f64) {
        self.inner.isobutane = value;
    }

    #[setter]
    fn set_n_butane(&mut self, value: f64) {
        self.inner.n_butane = value;
    }

    #[setter]
    fn set_isopentane(&mut self, value: f64) {
        self.inner.isopentane = value;
    }

    #[setter]
    fn set_n_pentane(&mut self, value: f64) {
        self.inner.n_pentane = value;
    }

    #[setter]
    fn set_hexane(&mut self, value: f64) {
        self.inner.hexane = value;
    }

    #[setter]
    fn set_heptane(&mut self, value: f64) {
        self.inner.heptane = value;
    }

    #[setter]
    fn set_octane(&mut self, value: f64) {
        self.inner.octane = value;
    }

    #[setter]
    fn set_nonane(&mut self, value: f64) {
        self.inner.nonane = value;
    }

    #[setter]
    fn set_decane(&mut self, value: f64) {
        self.inner.decane = value;
    }

    #[setter]
    fn set_hydrogen(&mut self, value: f64) {
        self.inner.hydrogen = value;
    }

    #[setter]
    fn set_oxygen(&mut self, value: f64) {
        self.inner.oxygen = value;
    }

    #[setter]
    fn set_carbon_monoxide(&mut self, value: f64) {
        self.inner.carbon_monoxide = value;
    }

    #[setter]
    fn set_water(&mut self, value: f64) {
        self.inner.water = value;
    }

    #[setter]
    fn set_hydrogen_sulfide(&mut self, value: f64) {
        self.inner.hydrogen_sulfide = value;
    }

    #[setter]
    fn set_helium(&mut self, value: f64) {
        self.inner.helium = value;
    }

    #[setter]
    fn set_argon(&mut self, value: f64) {
        self.inner.argon = value;
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyaga8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Gerg2008>()?;
    m.add_class::<Detail>()?;
    m.add_class::<Composition>()?;
    Ok(())
}
