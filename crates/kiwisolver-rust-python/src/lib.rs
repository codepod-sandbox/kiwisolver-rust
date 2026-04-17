use pyo3::prelude::*;

#[pyclass]
struct Variable;

#[pyclass]
struct Term;

#[pyclass]
struct Expression;

#[pyclass]
struct Constraint;

#[pyclass]
struct Solver;

#[pymodule]
fn _kiwisolver_native(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Variable>()?;
    m.add_class::<Term>()?;
    m.add_class::<Expression>()?;
    m.add_class::<Constraint>()?;
    m.add_class::<Solver>()?;
    Ok(())
}
