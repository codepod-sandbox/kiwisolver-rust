use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::expression::{create_expression, Expression, ExpressionData};
use crate::strength;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
pub struct Constraint {
    expression: ExpressionData,
    op: String,
    strength: f64,
}

#[pymethods]
impl Constraint {
    #[new]
    #[pyo3(signature = (expression, op, strength=None))]
    fn new(
        py: Python<'_>,
        expression: Py<Expression>,
        op: &str,
        strength: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let expression = expression.bind(py).borrow().data.clone_ref(py);
        let strength = match strength {
            Some(value) => resolve_strength(value)?,
            None => strength::REQUIRED,
        };
        Ok(Self {
            expression,
            op: op.to_owned(),
            strength,
        })
    }

    fn expression(&self, py: Python<'_>) -> PyResult<Py<Expression>> {
        create_expression(py, self.expression.clone_ref(py))
    }

    fn op(&self) -> &str {
        &self.op
    }

    fn strength(&self) -> f64 {
        self.strength
    }

    fn violated(&self, py: Python<'_>) -> bool {
        let value = self.expression.value(py);
        match self.op.as_str() {
            "==" => value != 0.0,
            ">=" => value < 0.0,
            "<=" => value > 0.0,
            _ => true,
        }
    }

    fn __or__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Constraint>> {
        create_constraint(py, self.expression.clone_ref(py), &self.op, resolve_strength(other)?)
    }

    fn __ror__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Constraint>> {
        self.__or__(py, other)
    }
}

pub(crate) fn create_constraint(
    py: Python<'_>,
    expression: ExpressionData,
    op: &str,
    strength: f64,
) -> PyResult<Py<Constraint>> {
    Py::new(
        py,
        Constraint {
            expression,
            op: op.to_owned(),
            strength,
        },
    )
}

pub(crate) fn resolve_strength(value: &Bound<'_, PyAny>) -> PyResult<f64> {
    if let Ok(strength) = value.extract::<f64>() {
        return Ok(strength);
    }

    if let Ok(name) = value.extract::<&str>() {
        return match name {
            "weak" => Ok(strength::WEAK),
            "medium" => Ok(strength::MEDIUM),
            "strong" => Ok(strength::STRONG),
            "required" => Ok(strength::REQUIRED),
            _ => Err(PyTypeError::new_err("invalid strength name")),
        };
    }

    Err(PyTypeError::new_err("strength must be a number or known strength name"))
}
