mod constraint;
mod errors;
mod expression;
mod solver;
mod strength;
mod variable;

pub use constraint::Constraint;
pub use expression::{Expression, Term};
use pyo3::prelude::*;
pub use solver::Solver;
pub use variable::Variable;

#[pymodule]
#[pyo3(name = "_kiwisolver_native")]
fn kiwisolver(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Variable>()?;
    m.add_class::<Term>()?;
    m.add_class::<Expression>()?;
    m.add_class::<Constraint>()?;
    m.add_class::<Solver>()?;
    errors::register(_py, m)?;
    strength::register(_py, m)?;
    Ok(())
}
