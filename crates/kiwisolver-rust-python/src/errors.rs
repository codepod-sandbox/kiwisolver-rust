use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(_kiwisolver_native, BadRequiredStrength, PyException);
create_exception!(_kiwisolver_native, DuplicateConstraint, PyException);
create_exception!(_kiwisolver_native, DuplicateEditVariable, PyException);
create_exception!(_kiwisolver_native, UnknownConstraint, PyException);
create_exception!(_kiwisolver_native, UnknownEditVariable, PyException);
create_exception!(_kiwisolver_native, UnsatisfiableConstraint, PyException);

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("BadRequiredStrength", m.py().get_type::<BadRequiredStrength>())?;
    m.add("DuplicateConstraint", m.py().get_type::<DuplicateConstraint>())?;
    m.add(
        "DuplicateEditVariable",
        m.py().get_type::<DuplicateEditVariable>(),
    )?;
    m.add("UnknownConstraint", m.py().get_type::<UnknownConstraint>())?;
    m.add("UnknownEditVariable", m.py().get_type::<UnknownEditVariable>())?;
    m.add(
        "UnsatisfiableConstraint",
        m.py().get_type::<UnsatisfiableConstraint>(),
    )?;
    Ok(())
}
