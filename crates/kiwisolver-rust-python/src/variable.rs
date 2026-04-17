use cassowary::Variable as CassowaryVariable;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::constraint;
use crate::expression::{self, Expression};

#[pyclass(module = "kiwisolver._kiwisolver_native")]
pub struct Variable {
    name: String,
    value: f64,
    context: Option<Py<PyAny>>,
    backend: CassowaryVariable,
}

#[pymethods]
impl Variable {
    #[new]
    #[pyo3(signature = (name="", context=None))]
    fn new(name: &str, context: Option<Py<PyAny>>) -> Self {
        Self {
            name: name.to_owned(),
            value: 0.0,
            context,
            backend: CassowaryVariable::new(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    #[pyo3(name = "setName")]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    fn value(&self) -> f64 {
        self.value
    }

    fn context(&self, py: Python<'_>) -> Py<PyAny> {
        self.context
            .as_ref()
            .map(|context| context.clone_ref(py))
            .unwrap_or_else(|| py.None())
    }

    #[pyo3(name = "setContext", signature = (context=None))]
    fn set_context(&mut self, context: Option<Py<PyAny>>) {
        self.context = context;
    }

    fn __repr__(&self) -> String {
        format!("Variable('{}')", self.name)
    }

    fn __neg__(slf: Py<Self>, py: Python<'_>) -> PyResult<Py<expression::Term>> {
        expression::create_term(py, slf, -1.0)
    }

    fn __mul__(slf: Py<Self>, py: Python<'_>, other: f64) -> PyResult<Py<expression::Term>> {
        expression::create_term(py, slf, other)
    }

    fn __rmul__(slf: Py<Self>, py: Python<'_>, other: f64) -> PyResult<Py<expression::Term>> {
        expression::create_term(py, slf, other)
    }

    fn __truediv__(slf: Py<Self>, py: Python<'_>, other: f64) -> PyResult<Py<expression::Term>> {
        let other = expression::checked_divisor(other)?;
        expression::create_term(py, slf, 1.0 / other)
    }

    fn __add__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<Expression>> {
        expression::create_expression(
            py,
            expression::ExpressionData::from_variable(slf)
                .add(py, &expression::operand_to_expression(other)?),
        )
    }

    fn __radd__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<Expression>> {
        expression::create_expression(
            py,
            expression::operand_to_expression(other)?
                .add(py, &expression::ExpressionData::from_variable(slf)),
        )
    }

    fn __sub__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<Expression>> {
        expression::create_expression(
            py,
            expression::ExpressionData::from_variable(slf)
                .subtract(py, &expression::operand_to_expression(other)?),
        )
    }

    fn __rsub__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<Expression>> {
        expression::create_expression(
            py,
            expression::operand_to_expression(other)?
                .subtract(py, &expression::ExpressionData::from_variable(slf)),
        )
    }

    fn __eq__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            expression::ExpressionData::from_variable(slf)
                .subtract(py, &expression::operand_to_expression(other)?),
            "==",
            crate::strength::REQUIRED,
        )
    }

    fn __ge__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            expression::ExpressionData::from_variable(slf)
                .subtract(py, &expression::operand_to_expression(other)?),
            ">=",
            crate::strength::REQUIRED,
        )
    }

    fn __le__(
        slf: Py<Self>,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            expression::ExpressionData::from_variable(slf)
                .subtract(py, &expression::operand_to_expression(other)?),
            "<=",
            crate::strength::REQUIRED,
        )
    }
}

impl Variable {
    pub(crate) fn current_value(&self) -> f64 {
        self.value
    }

    pub(crate) fn backend_variable(&self) -> CassowaryVariable {
        self.backend
    }

    pub(crate) fn set_current_value(&mut self, value: f64) {
        self.value = value;
    }
}
