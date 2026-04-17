use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::constraint::Constraint;
use crate::variable::Variable;

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

pub fn get_exception_type<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyType>> {
    Ok(PyModule::import(py, "kiwisolver.exceptions")?
        .getattr(name)?
        .cast_into()?)
}

pub fn constraint_error(py: Python<'_>, name: &str, constraint: Py<Constraint>) -> PyErr {
    match get_exception_type(py, name) {
        Ok(exc) => PyErr::from_type(exc, (constraint,)),
        Err(err) => err,
    }
}

pub fn edit_variable_error(py: Python<'_>, name: &str, variable: Py<Variable>) -> PyErr {
    match get_exception_type(py, name) {
        Ok(exc) => PyErr::from_type(exc, (variable,)),
        Err(err) => err,
    }
}
