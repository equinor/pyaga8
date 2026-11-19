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
use pyo3::exceptions::{PyRuntimeError, PyValueError, PyAssertionError};
use pyo3::prelude::*;

/// Convert Rust-side composition errors into ordinary Python exceptions.
fn composition_error_to_py_err(err: CompositionError) -> PyErr {
    match err {
        CompositionError::Empty => PyValueError::new_err("composition cannot be empty"),
        CompositionError::BadSum => {
            PyValueError::new_err("composition sum must be within 0.01 of 1.0")
        }
        CompositionError::Ok => PyAssertionError::new_err("Not an error - should never happen"),
    }
}

/// Convert Rust-side density errors into ordinary Python exceptions.
fn density_error_to_py_err(err: DensityError) -> PyErr {
    match err {
        DensityError::PressureTooLow => PyValueError::new_err("pressure is too low for density calculation"),
        DensityError::IterationFail => {
            PyRuntimeError::new_err("density calculation failed to converge")
        }
        DensityError::Ok => PyAssertionError::new_err("Not an error - should never happen"),
    }
}

/// GERG-2008 equation of state, AGA Report No. 8, Part 2, First Edition, April 2017.
///
/// Wrapper around ``aga8::gerg2008::Gerg2008`` in the Rust crate aga8 0.6.1. The units are those of
/// the aga8 crate (https://docs.rs/aga8/0.6.1/aga8/gerg2008/struct.Gerg2008.html):
///
/// - ``temperature``: K
/// - ``pressure``: kPa
/// - ``d`` (molar concentration): mol/l
/// - ``mm`` (molar mass): g/mol
/// - ``u``, ``h``, ``g`` (internal energy, enthalpy, Gibbs energy): J/mol
/// - ``s``, ``cv``, ``cp`` (entropy, isochoric and isobaric heat capacity): J/(mol-K)
/// - ``w`` (speed of sound): m/s
/// - ``jt`` (Joule-Thomson coefficient): K/kPa
/// - ``z`` (compressibility factor) and ``kappa`` (isentropic exponent): dimensionless
///
/// The object holds the state. Set the composition with ``set_composition``, set
/// ``temperature`` and ``pressure``, call ``calc_density`` and then ``calc_properties``,
/// and read the results from the properties of this object. No unit conversion is done.
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

    /// Pressure in kPa.
    ///
    /// Input to ``calc_density``.
    #[getter]
    fn get_pressure(&self) -> PyResult<f64> {
        Ok(self.inner.p)
    }

    #[setter]
    fn set_temperature(&mut self, temperature: f64) -> PyResult<()> {
        self.inner.t = temperature;
        Ok(())
    }

    /// Temperature in K.
    #[getter]
    fn get_temperature(&self) -> PyResult<f64> {
        Ok(self.inner.t)
    }

    #[setter]
    fn set_d(&mut self, d: f64) {
        self.inner.d = d;
    }

    /// Molar concentration (molar density) in mol/l.
    ///
    /// Result of ``calc_density``; input to ``calc_pressure`` and ``calc_properties``.
    #[getter]
    fn get_d(&self) -> f64 {
        self.inner.d
    }

    /// Compressibility factor (dimensionless).
    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z
    }

    /// Molar mass in g/mol.
    #[getter]
    fn get_mm(&self) -> f64 {
        self.inner.mm
    }

    /// First derivative of pressure with respect to density at constant temperature
    /// in kPa/(mol/l).
    #[getter]
    fn get_dp_dd(&self) -> f64 {
        self.inner.dp_dd
    }

    /// Second derivative of pressure with respect to density at constant
    /// temperature in kPa/(mol/l)^2.
    #[getter]
    fn get_d2p_dd2(&self) -> f64 {
        self.inner.d2p_dd2
    }

    /// Second derivative of pressure with respect to temperature and density in
    /// kPa/(mol/l)/K.
    #[getter]
    fn get_d2p_dtd(&self) -> f64 {
        self.inner.d2p_dtd
    }

    /// First derivative of pressure with respect to temperature at constant density
    /// in kPa/K.
    #[getter]
    fn get_dp_dt(&self) -> f64 {
        self.inner.dp_dt
    }

    /// Internal energy in J/mol.
    #[getter]
    fn get_u(&self) -> f64 {
        self.inner.u
    }

    /// Enthalpy in J/mol.
    #[getter]
    fn get_h(&self) -> f64 {
        self.inner.h
    }

    /// Entropy in J/(mol-K).
    #[getter]
    fn get_s(&self) -> f64 {
        self.inner.s
    }

    /// Isochoric heat capacity in J/(mol-K).
    #[getter]
    fn get_cv(&self) -> f64 {
        self.inner.cv
    }

    /// Isobaric heat capacity in J/(mol-K).
    #[getter]
    fn get_cp(&self) -> f64 {
        self.inner.cp
    }

    /// Speed of sound in m/s.
    #[getter]
    fn get_w(&self) -> f64 {
        self.inner.w
    }

    /// Gibbs energy in J/mol.
    #[getter]
    fn get_g(&self) -> f64 {
        self.inner.g
    }

    /// Joule-Thomson coefficient in K/kPa.
    #[getter]
    fn get_jt(&self) -> f64 {
        self.inner.jt
    }

    /// Isentropic exponent (dimensionless).
    #[getter]
    fn get_kappa(&self) -> f64 {
        self.inner.kappa
    }

    // Functions
    /// Calculate the molar density ``d`` (mol/l) from ``temperature`` (K) and ``pressure`` (kPa).
    ///
    /// Iterative solution of the GERG-2008 equation of state. Updates ``d`` and ``z``; for
    /// ``flag > 0`` the 2-phase check also runs ``calc_properties``. Call ``calc_properties``
    /// afterwards to get all the other properties at the converged state.
    ///
    /// Parameters
    /// ----------
    /// flag : int
    ///     Passed to ``aga8::gerg2008::Gerg2008::density(iflag)``. In aga8 0.6.1 the
    ///     iteration starts from the ideal-gas density, or for ``flag == 2`` from three times
    ///     the pseudo-critical density (liquid region). If ``d`` is negative before the call,
    ///     ``abs(d)`` is used as the start value instead. For ``flag > 0`` the converged state
    ///     is also checked for a possible 2-phase state, and the calculation fails if the check
    ///     fails. Use ``0`` for no check.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the calculation fails to converge, or the 2-phase check fails. ``d`` then holds
    ///     the ideal-gas density.
    fn calc_density(&mut self, flag: i32) -> PyResult<()> {
        self.inner.density(flag).map_err(density_error_to_py_err)
    }

    /// Calculate the pressure from ``temperature`` (K) and ``d`` (mol/l).
    ///
    /// Also updates ``z``. The ``pressure`` property is not changed.
    ///
    /// Returns
    /// -------
    /// float
    ///     Pressure in kPa.
    fn calc_pressure(&mut self) -> f64 {
        self.inner.pressure()
    }

    /// Calculate the thermodynamic properties from ``temperature`` (K) and ``d`` (mol/l).
    ///
    /// Updates ``mm``, ``z``, ``dp_dd``, ``d2p_dd2``, ``d2p_dtd``, ``dp_dt``, ``u``, ``h``,
    /// ``s``, ``cv``, ``cp``, ``w``, ``g``, ``jt`` and ``kappa``. If ``d`` is not known, call
    /// ``calc_density`` first. Returns None.
    fn calc_properties(&mut self) {
        self.inner.properties();
    }

    /// Calculate the molar mass of the current composition.
    ///
    /// The result is stored in ``mm`` (g/mol). Returns None.
    fn calc_molar_mass(&mut self) {
        self.inner.molar_mass();
    }

    /// Check and set the gas composition.
    ///
    /// The mole fractions are copied from ``comp``. Later changes to ``comp`` have no effect
    /// until ``set_composition`` is called again.
    ///
    /// Parameters
    /// ----------
    /// comp : Composition
    ///     Gas composition in mole fractions.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the composition is empty (sum of the mole fractions is zero), or if the sum of the mole
    ///     fractions differs from 1.0 by more than 0.01.
    fn set_composition(&mut self, comp: &Composition) -> PyResult<()> {
        self.inner
            .set_composition(&comp.inner)
            .map_err(composition_error_to_py_err)
    }
}

/// AGA8 DETAIL equation of state, AGA Report No. 8, Part 1, Third Edition, April 2017.
///
/// Wrapper around ``aga8::detail::Detail`` in the Rust crate aga8 0.6.1. The units are those of
/// the aga8 crate (https://docs.rs/aga8/0.6.1/aga8/detail/struct.Detail.html):
///
/// - ``temperature``: K
/// - ``pressure``: kPa
/// - ``d`` (molar concentration): mol/l
/// - ``mm`` (molar mass): g/mol
/// - ``u``, ``h``, ``g`` (internal energy, enthalpy, Gibbs energy): J/mol
/// - ``s``, ``cv``, ``cp`` (entropy, isochoric and isobaric heat capacity): J/(mol-K)
/// - ``w`` (speed of sound): m/s
/// - ``jt`` (Joule-Thomson coefficient): K/kPa
/// - ``z`` (compressibility factor) and ``kappa`` (isentropic exponent): dimensionless
///
/// The object holds the state. Set the composition with ``set_composition``, set
/// ``temperature`` and ``pressure``, call ``calc_density`` and then ``calc_properties``,
/// and read the results from the properties of this object. No unit conversion is done.
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

    /// Pressure in kPa.
    ///
    /// Input to ``calc_density``.
    /// ``calc_properties`` overwrites it with the pressure calculated from
    /// ``temperature`` and ``d``.
    #[getter]
    fn get_pressure(&self) -> PyResult<f64> {
        Ok(self.inner.p)
    }

    #[setter]
    fn set_temperature(&mut self, temperature: f64) -> PyResult<()> {
        self.inner.t = temperature;
        Ok(())
    }

    /// Temperature in K.
    #[getter]
    fn get_temperature(&self) -> PyResult<f64> {
        Ok(self.inner.t)
    }

    #[setter]
    fn set_d(&mut self, d: f64) {
        self.inner.d = d;
    }

    /// Molar concentration (molar density) in mol/l.
    ///
    /// Result of ``calc_density``; input to ``calc_pressure`` and ``calc_properties``.
    #[getter]
    fn get_d(&self) -> f64 {
        self.inner.d
    }

    /// Compressibility factor (dimensionless).
    #[getter]
    fn get_z(&self) -> f64 {
        self.inner.z
    }

    /// Molar mass in g/mol.
    #[getter]
    fn get_mm(&self) -> f64 {
        self.inner.mm
    }

    /// First derivative of pressure with respect to density at constant temperature
    /// in kPa/(mol/l).
    #[getter]
    fn get_dp_dd(&self) -> f64 {
        self.inner.dp_dd
    }

    /// Second derivative of pressure with respect to density at constant
    /// temperature in kPa/(mol/l)^2.
    #[getter]
    fn get_d2p_dd2(&self) -> f64 {
        self.inner.d2p_dd2
    }

    /// Second derivative of pressure with respect to temperature and density in
    /// kPa/(mol/l)/K.
    ///
    /// Not calculated by the DETAIL implementation in aga8 0.6.1:
    /// ``calc_properties`` sets it to 0.0.
    #[getter]
    fn get_d2p_dtd(&self) -> f64 {
        self.inner.d2p_dtd
    }

    /// First derivative of pressure with respect to temperature at constant density
    /// in kPa/K.
    #[getter]
    fn get_dp_dt(&self) -> f64 {
        self.inner.dp_dt
    }

    /// Internal energy in J/mol.
    #[getter]
    fn get_u(&self) -> f64 {
        self.inner.u
    }

    /// Enthalpy in J/mol.
    #[getter]
    fn get_h(&self) -> f64 {
        self.inner.h
    }

    /// Entropy in J/(mol-K).
    #[getter]
    fn get_s(&self) -> f64 {
        self.inner.s
    }

    /// Isochoric heat capacity in J/(mol-K).
    #[getter]
    fn get_cv(&self) -> f64 {
        self.inner.cv
    }

    /// Isobaric heat capacity in J/(mol-K).
    #[getter]
    fn get_cp(&self) -> f64 {
        self.inner.cp
    }

    /// Speed of sound in m/s.
    #[getter]
    fn get_w(&self) -> f64 {
        self.inner.w
    }

    /// Gibbs energy in J/mol.
    #[getter]
    fn get_g(&self) -> f64 {
        self.inner.g
    }

    /// Joule-Thomson coefficient in K/kPa.
    #[getter]
    fn get_jt(&self) -> f64 {
        self.inner.jt
    }

    /// Isentropic exponent (dimensionless).
    #[getter]
    fn get_kappa(&self) -> f64 {
        self.inner.kappa
    }

    // Functions
    /// Calculate the molar density ``d`` (mol/l) from ``temperature`` (K) and ``pressure`` (kPa).
    ///
    /// Iterative solution of the DETAIL equation of state, starting from the ideal-gas density,
    /// or from ``abs(d)`` if ``d`` is negative before the call. Updates ``d`` and ``z``; call
    /// ``calc_properties`` afterwards for the other properties.
    ///
    /// No checks are made to determine the phase boundary. If the state point is 2-phase, the
    /// density represents a metastable state (aga8 0.6.1 documentation).
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the pressure is too low (zero). ``d`` is then set to 0.0.
    /// RuntimeError
    ///     If the calculation fails to converge. ``d`` then holds the ideal-gas density.
    fn calc_density(&mut self) -> PyResult<()> {
        self.inner.density().map_err(density_error_to_py_err)
    }

    /// Calculate the pressure from ``temperature`` (K) and ``d`` (mol/l).
    ///
    /// Also updates ``z``. The ``pressure`` property is not changed.
    ///
    /// Returns
    /// -------
    /// float
    ///     Pressure in kPa.
    fn calc_pressure(&mut self) -> f64 {
        self.inner.pressure()
    }

    /// Calculate the thermodynamic properties from ``temperature`` (K) and ``d`` (mol/l).
    ///
    /// Updates ``mm``, ``z``, ``pressure``, ``dp_dd``, ``d2p_dd2``, ``d2p_dtd`` (set to 0.0),
    /// ``dp_dt``, ``u``, ``h``, ``s``, ``cv``, ``cp``, ``w``, ``g``, ``jt`` and ``kappa``. If ``d``
    /// is not known, call ``calc_density`` first. Returns None.
    fn calc_properties(&mut self) {
        self.inner.properties();
    }

    /// Calculate the molar mass of the current composition.
    ///
    /// The result is stored in ``mm`` (g/mol). Returns None.
    fn calc_molar_mass(&mut self) {
        self.inner.molar_mass();
    }

    /// Check and set the gas composition.
    ///
    /// The mole fractions are copied from ``comp``. Later changes to ``comp`` have no effect
    /// until ``set_composition`` is called again.
    ///
    /// Parameters
    /// ----------
    /// comp : Composition
    ///     Gas composition in mole fractions.
    ///
    /// Raises
    /// ------
    /// ValueError
    ///     If the composition is empty (sum of the mole fractions is zero), or if the sum of the mole
    ///     fractions differs from 1.0 by more than 0.01.
    fn set_composition(&mut self, comp: &Composition) -> PyResult<()> {
        self.inner
            .set_composition(&comp.inner)
            .map_err(composition_error_to_py_err)
    }
}

/// Gas composition of 21 components, in mole fractions.
///
/// Wrapper around ``aga8::composition::Composition`` in the Rust crate aga8 0.6.1
/// (https://docs.rs/aga8/0.6.1/aga8/composition/struct.Composition.html). The unit of each component is mole fraction, so the sum of all
/// components should be 1.0. All components are 0.0 after construction.
///
/// The components can only be set, not read back. ``Gerg2008.set_composition`` and
/// ``Detail.set_composition`` raise ``ValueError`` if the composition is empty or if the sum
/// differs from 1.0 by more than 0.01.
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

    /// Mole fraction of methane (write-only).
    #[setter]
    fn set_methane(&mut self, value: f64) {
        self.inner.methane = value;
    }

    /// Mole fraction of nitrogen (write-only).
    #[setter]
    fn set_nitrogen(&mut self, value: f64) {
        self.inner.nitrogen = value;
    }

    /// Mole fraction of carbon dioxide (write-only).
    #[setter]
    fn set_carbon_dioxide(&mut self, value: f64) {
        self.inner.carbon_dioxide = value;
    }

    /// Mole fraction of ethane (write-only).
    #[setter]
    fn set_ethane(&mut self, value: f64) {
        self.inner.ethane = value;
    }

    /// Mole fraction of propane (write-only).
    #[setter]
    fn set_propane(&mut self, value: f64) {
        self.inner.propane = value;
    }

    /// Mole fraction of isobutane (write-only).
    #[setter]
    fn set_isobutane(&mut self, value: f64) {
        self.inner.isobutane = value;
    }

    /// Mole fraction of n-butane (write-only).
    #[setter]
    fn set_n_butane(&mut self, value: f64) {
        self.inner.n_butane = value;
    }

    /// Mole fraction of isopentane (write-only).
    #[setter]
    fn set_isopentane(&mut self, value: f64) {
        self.inner.isopentane = value;
    }

    /// Mole fraction of n-pentane (write-only).
    #[setter]
    fn set_n_pentane(&mut self, value: f64) {
        self.inner.n_pentane = value;
    }

    /// Mole fraction of hexane (write-only).
    #[setter]
    fn set_hexane(&mut self, value: f64) {
        self.inner.hexane = value;
    }

    /// Mole fraction of heptane (write-only).
    #[setter]
    fn set_heptane(&mut self, value: f64) {
        self.inner.heptane = value;
    }

    /// Mole fraction of octane (write-only).
    #[setter]
    fn set_octane(&mut self, value: f64) {
        self.inner.octane = value;
    }

    /// Mole fraction of nonane (write-only).
    #[setter]
    fn set_nonane(&mut self, value: f64) {
        self.inner.nonane = value;
    }

    /// Mole fraction of decane (write-only).
    #[setter]
    fn set_decane(&mut self, value: f64) {
        self.inner.decane = value;
    }

    /// Mole fraction of hydrogen (write-only).
    #[setter]
    fn set_hydrogen(&mut self, value: f64) {
        self.inner.hydrogen = value;
    }

    /// Mole fraction of oxygen (write-only).
    #[setter]
    fn set_oxygen(&mut self, value: f64) {
        self.inner.oxygen = value;
    }

    /// Mole fraction of carbon monoxide (write-only).
    #[setter]
    fn set_carbon_monoxide(&mut self, value: f64) {
        self.inner.carbon_monoxide = value;
    }

    /// Mole fraction of water (write-only).
    #[setter]
    fn set_water(&mut self, value: f64) {
        self.inner.water = value;
    }

    /// Mole fraction of hydrogen sulfide (write-only).
    #[setter]
    fn set_hydrogen_sulfide(&mut self, value: f64) {
        self.inner.hydrogen_sulfide = value;
    }

    /// Mole fraction of helium (write-only).
    #[setter]
    fn set_helium(&mut self, value: f64) {
        self.inner.helium = value;
    }

    /// Mole fraction of argon (write-only).
    #[setter]
    fn set_argon(&mut self, value: f64) {
        self.inner.argon = value;
    }
}

/// Python bindings for the AGA8 GERG-2008 and DETAIL equations of state.
///
/// Built on the Rust crate aga8 0.6.1 (https://docs.rs/aga8/0.6.1/aga8/). The inputs and outputs
/// use the units of that crate: temperature in K, pressure in kPa, molar concentration in mol/l,
/// molar mass in g/mol, energies in J/mol, entropy and heat capacities in J/(mol-K), speed of sound
/// in m/s and Joule-Thomson coefficient in K/kPa. Compositions are mole fractions.
#[pymodule]
fn pyaga8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Gerg2008>()?;
    m.add_class::<Detail>()?;
    m.add_class::<Composition>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
