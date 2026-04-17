mod errors;
mod strength;
mod variable;

use pyo3::prelude::*;
use variable::Variable;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
struct Term;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
struct Expression;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
struct Constraint;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
struct Solver;

#[pymodule]
#[pyo3(name = "_kiwisolver_native")]
fn kiwisolver(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Variable>()?;
    m.add_class::<Term>()?;
    m.add_class::<Expression>()?;
    m.add_class::<Constraint>()?;
    m.add_class::<Solver>()?;
    errors::register(m)?;
    strength::register(_py, m)?;
    Ok(())
}
