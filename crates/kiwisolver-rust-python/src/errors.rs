use pyo3::prelude::*;
use pyo3::types::PyType;

const EXCEPTION_NAMES: &[&str] = &[
    "BadRequiredStrength",
    "DuplicateConstraint",
    "DuplicateEditVariable",
    "UnknownConstraint",
    "UnknownEditVariable",
    "UnsatisfiableConstraint",
];

pub fn register(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let exceptions = PyModule::import(py, "kiwisolver.exceptions")?;

    for name in EXCEPTION_NAMES {
        m.add(*name, exceptions.getattr(*name)?)?;
    }

    Ok(())
}

pub fn get_exception_type<'py>(
    py: Python<'py>,
    name: &str,
) -> PyResult<Bound<'py, PyType>> {
    Ok(PyModule::import(py, "kiwisolver.exceptions")?
        .getattr(name)?
        .cast_into()?)
}
