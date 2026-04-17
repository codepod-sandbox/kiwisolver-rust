use pyo3::prelude::*;

pub const REQUIRED: f64 = 1_001_001_000.0;
pub const STRONG: f64 = 1_000_000.0;
pub const MEDIUM: f64 = 1_000.0;
pub const WEAK: f64 = 1.0;

#[pyclass(module = "kiwisolver._kiwisolver_native", frozen)]
pub struct Strength;

#[pymethods]
impl Strength {
    #[getter]
    fn weak(&self) -> f64 {
        WEAK
    }

    #[getter]
    fn medium(&self) -> f64 {
        MEDIUM
    }

    #[getter]
    fn strong(&self) -> f64 {
        STRONG
    }

    #[getter]
    fn required(&self) -> f64 {
        REQUIRED
    }

    #[pyo3(signature = (a, b, c, weight=1.0))]
    fn create(&self, a: f64, b: f64, c: f64, weight: f64) -> f64 {
        ((a * STRONG) + (b * MEDIUM) + c) * weight
    }
}

pub fn register(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("strength", Py::new(py, Strength)?)?;
    Ok(())
}
