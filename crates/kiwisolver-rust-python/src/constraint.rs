use cassowary::{Constraint as CassowaryConstraint, RelationalOperator};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::errors;
use crate::expression::{create_expression, Expression, ExpressionData};
use crate::strength;
use crate::variable::Variable;

#[pyclass(module = "kiwisolver._kiwisolver_native")]
pub struct Constraint {
    expression: ExpressionData,
    op: String,
    strength: f64,
    backend: CassowaryConstraint,
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
        validate_op(op)?;
        let strength = match strength {
            Some(value) => resolve_strength(py, value)?,
            None => strength::REQUIRED,
        };
        Ok(Self {
            backend: build_backend_constraint(py, &expression, op, strength)?,
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
        create_constraint(
            py,
            self.expression.clone_ref(py),
            &self.op,
            resolve_strength(py, other)?,
        )
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
    validate_op(op)?;
    Py::new(
        py,
        Constraint {
            backend: build_backend_constraint(py, &expression, op, strength)?,
            expression,
            op: op.to_owned(),
            strength,
        },
    )
}

impl Constraint {
    pub(crate) fn backend_constraint(&self) -> CassowaryConstraint {
        self.backend.clone()
    }

    pub(crate) fn tracked_variables(&self, py: Python<'_>) -> Vec<Py<Variable>> {
        self.expression.variables(py)
    }
}

fn validate_op(op: &str) -> PyResult<()> {
    match op {
        "==" | ">=" | "<=" => Ok(()),
        _ => Err(PyValueError::new_err(
            "constraint operator must be one of ==, >=, <=",
        )),
    }
}

fn build_backend_constraint(
    py: Python<'_>,
    expression: &ExpressionData,
    op: &str,
    strength: f64,
) -> PyResult<CassowaryConstraint> {
    Ok(CassowaryConstraint::new(
        expression.to_cassowary(py),
        relation_for_op(op)?,
        strength,
    ))
}

fn relation_for_op(op: &str) -> PyResult<RelationalOperator> {
    Ok(match op {
        "==" => RelationalOperator::Equal,
        ">=" => RelationalOperator::GreaterOrEqual,
        "<=" => RelationalOperator::LessOrEqual,
        _ => {
            return Err(PyValueError::new_err(
                "constraint operator must be one of ==, >=, <=",
            ))
        }
    })
}

pub(crate) fn resolve_strength(py: Python<'_>, value: &Bound<'_, PyAny>) -> PyResult<f64> {
    if let Ok(strength) = value.extract::<f64>() {
        return validate_strength_value(py, strength);
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

    Err(PyTypeError::new_err(
        "strength must be a number or known strength name",
    ))
}

fn validate_strength_value(py: Python<'_>, value: f64) -> PyResult<f64> {
    if (0.0..=strength::REQUIRED).contains(&value) {
        return Ok(value);
    }

    let exc = errors::get_exception_type(py, "BadRequiredStrength")?;
    Err(PyErr::from_type(
        exc,
        "constraint strength must be within [0, required]",
    ))
}
