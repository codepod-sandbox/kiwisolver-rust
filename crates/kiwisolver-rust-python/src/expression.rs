use cassowary::{Expression as CassowaryExpression, Term as CassowaryTerm};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyTuple};

use crate::constraint;
use crate::variable::Variable;

pub(crate) struct TermData {
    pub(crate) variable: Py<Variable>,
    pub(crate) coefficient: f64,
}

pub(crate) struct ExpressionData {
    pub(crate) terms: Vec<TermData>,
    pub(crate) constant: f64,
}

impl TermData {
    pub(crate) fn clone_ref(&self, py: Python<'_>) -> Self {
        Self {
            variable: self.variable.clone_ref(py),
            coefficient: self.coefficient,
        }
    }

    pub(crate) fn to_cassowary(&self, py: Python<'_>) -> CassowaryTerm {
        let variable = self.variable.bind(py).borrow().backend_variable();
        CassowaryTerm {
            variable,
            coefficient: self.coefficient,
        }
    }
}

impl ExpressionData {
    pub(crate) fn from_variable(variable: Py<Variable>) -> Self {
        Self {
            terms: vec![TermData {
                variable,
                coefficient: 1.0,
            }],
            constant: 0.0,
        }
    }

    pub(crate) fn from_term(term: TermData) -> Self {
        Self {
            terms: vec![term],
            constant: 0.0,
        }
    }

    pub(crate) fn clone_ref(&self, py: Python<'_>) -> Self {
        Self {
            terms: self.terms.iter().map(|term| term.clone_ref(py)).collect(),
            constant: self.constant,
        }
    }

    pub(crate) fn negated(&self, py: Python<'_>) -> Self {
        self.scaled(py, -1.0)
    }

    pub(crate) fn scaled(&self, py: Python<'_>, factor: f64) -> Self {
        Self {
            terms: self
                .terms
                .iter()
                .map(|term| TermData {
                    variable: term.variable.clone_ref(py),
                    coefficient: term.coefficient * factor,
                })
                .collect(),
            constant: self.constant * factor,
        }
    }

    pub(crate) fn add(&self, py: Python<'_>, other: &Self) -> Self {
        let mut terms = self
            .terms
            .iter()
            .map(|term| term.clone_ref(py))
            .collect::<Vec<_>>();
        terms.extend(other.terms.iter().map(|term| term.clone_ref(py)));
        Self {
            terms,
            constant: self.constant + other.constant,
        }
    }

    pub(crate) fn subtract(&self, py: Python<'_>, other: &Self) -> Self {
        self.add(py, &other.negated(py))
    }

    pub(crate) fn value(&self, py: Python<'_>) -> f64 {
        self.terms.iter().fold(self.constant, |acc, term| {
            let variable = term.variable.bind(py).borrow();
            acc + (term.coefficient * variable.current_value())
        })
    }

    pub(crate) fn to_cassowary(&self, py: Python<'_>) -> CassowaryExpression {
        CassowaryExpression::new(
            self.terms
                .iter()
                .map(|term| term.to_cassowary(py))
                .collect(),
            self.constant,
        )
    }

    pub(crate) fn variables(&self, py: Python<'_>) -> Vec<Py<Variable>> {
        self.terms
            .iter()
            .map(|term| term.variable.clone_ref(py))
            .collect()
    }
}

#[pyclass(module = "kiwisolver._kiwisolver_native")]
pub struct Term {
    pub(crate) data: TermData,
}

#[pymethods]
impl Term {
    #[new]
    #[pyo3(signature = (variable, coefficient=1.0))]
    fn new(variable: Py<Variable>, coefficient: f64) -> Self {
        Self {
            data: TermData {
                variable,
                coefficient,
            },
        }
    }

    fn coefficient(&self) -> f64 {
        self.data.coefficient
    }

    fn variable(&self, py: Python<'_>) -> Py<Variable> {
        self.data.variable.clone_ref(py)
    }

    fn value(&self, py: Python<'_>) -> f64 {
        self.data.coefficient * self.data.variable.bind(py).borrow().current_value()
    }

    fn __neg__(&self, py: Python<'_>) -> PyResult<Py<Term>> {
        create_term(py, self.data.variable.clone_ref(py), -self.data.coefficient)
    }

    fn __add__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(
            py,
            ExpressionData::from_term(self.data.clone_ref(py))
                .add(py, &operand_to_expression(other)?),
        )
    }

    fn __radd__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(
            py,
            operand_to_expression(other)?
                .add(py, &ExpressionData::from_term(self.data.clone_ref(py))),
        )
    }

    fn __sub__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(
            py,
            ExpressionData::from_term(self.data.clone_ref(py))
                .subtract(py, &operand_to_expression(other)?),
        )
    }

    fn __rsub__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(
            py,
            operand_to_expression(other)?
                .subtract(py, &ExpressionData::from_term(self.data.clone_ref(py))),
        )
    }

    fn __mul__(&self, py: Python<'_>, other: f64) -> PyResult<Py<Term>> {
        create_term(
            py,
            self.data.variable.clone_ref(py),
            self.data.coefficient * other,
        )
    }

    fn __rmul__(&self, py: Python<'_>, other: f64) -> PyResult<Py<Term>> {
        self.__mul__(py, other)
    }

    fn __truediv__(&self, py: Python<'_>, other: f64) -> PyResult<Py<Term>> {
        create_term(
            py,
            self.data.variable.clone_ref(py),
            self.data.coefficient / other,
        )
    }

    fn __eq__(
        &self,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            ExpressionData::from_term(self.data.clone_ref(py))
                .subtract(py, &operand_to_expression(other)?),
            "==",
            crate::strength::REQUIRED,
        )
    }

    fn __ge__(
        &self,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            ExpressionData::from_term(self.data.clone_ref(py))
                .subtract(py, &operand_to_expression(other)?),
            ">=",
            crate::strength::REQUIRED,
        )
    }

    fn __le__(
        &self,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            ExpressionData::from_term(self.data.clone_ref(py))
                .subtract(py, &operand_to_expression(other)?),
            "<=",
            crate::strength::REQUIRED,
        )
    }
}

#[pyclass(module = "kiwisolver._kiwisolver_native")]
pub struct Expression {
    pub(crate) data: ExpressionData,
}

#[pymethods]
impl Expression {
    #[new]
    #[pyo3(signature = (terms, constant=0.0))]
    fn new(py: Python<'_>, terms: Vec<Py<Term>>, constant: f64) -> Self {
        let terms = terms
            .into_iter()
            .map(|term| term.bind(py).borrow().data.clone_ref(py))
            .collect();
        Self {
            data: ExpressionData { terms, constant },
        }
    }

    fn constant(&self) -> f64 {
        self.data.constant
    }

    fn terms(&self, py: Python<'_>) -> PyResult<Py<PyTuple>> {
        let terms = self
            .data
            .terms
            .iter()
            .map(|term| create_term(py, term.variable.clone_ref(py), term.coefficient))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(PyTuple::new(py, terms)?.unbind())
    }

    fn value(&self, py: Python<'_>) -> f64 {
        self.data.value(py)
    }

    fn __neg__(&self, py: Python<'_>) -> PyResult<Py<Expression>> {
        create_expression(py, self.data.negated(py))
    }

    fn __add__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(py, self.data.add(py, &operand_to_expression(other)?))
    }

    fn __radd__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(py, operand_to_expression(other)?.add(py, &self.data))
    }

    fn __sub__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(py, self.data.subtract(py, &operand_to_expression(other)?))
    }

    fn __rsub__(&self, py: Python<'_>, other: &Bound<'_, PyAny>) -> PyResult<Py<Expression>> {
        create_expression(py, operand_to_expression(other)?.subtract(py, &self.data))
    }

    fn __mul__(&self, py: Python<'_>, other: f64) -> PyResult<Py<Expression>> {
        create_expression(py, self.data.scaled(py, other))
    }

    fn __rmul__(&self, py: Python<'_>, other: f64) -> PyResult<Py<Expression>> {
        self.__mul__(py, other)
    }

    fn __truediv__(&self, py: Python<'_>, other: f64) -> PyResult<Py<Expression>> {
        create_expression(py, self.data.scaled(py, 1.0 / other))
    }

    fn __eq__(
        &self,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            self.data.subtract(py, &operand_to_expression(other)?),
            "==",
            crate::strength::REQUIRED,
        )
    }

    fn __ge__(
        &self,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            self.data.subtract(py, &operand_to_expression(other)?),
            ">=",
            crate::strength::REQUIRED,
        )
    }

    fn __le__(
        &self,
        py: Python<'_>,
        other: &Bound<'_, PyAny>,
    ) -> PyResult<Py<constraint::Constraint>> {
        constraint::create_constraint(
            py,
            self.data.subtract(py, &operand_to_expression(other)?),
            "<=",
            crate::strength::REQUIRED,
        )
    }
}

pub(crate) fn create_term(
    py: Python<'_>,
    variable: Py<Variable>,
    coefficient: f64,
) -> PyResult<Py<Term>> {
    Py::new(
        py,
        Term {
            data: TermData {
                variable,
                coefficient,
            },
        },
    )
}

pub(crate) fn create_expression(py: Python<'_>, data: ExpressionData) -> PyResult<Py<Expression>> {
    Py::new(py, Expression { data })
}

pub(crate) fn operand_to_expression(other: &Bound<'_, PyAny>) -> PyResult<ExpressionData> {
    let py = other.py();

    if let Ok(value) = other.extract::<f64>() {
        return Ok(ExpressionData {
            terms: Vec::new(),
            constant: value,
        });
    }

    if let Ok(variable) = other.extract::<Py<Variable>>() {
        return Ok(ExpressionData::from_variable(variable));
    }

    if let Ok(term) = other.extract::<Py<Term>>() {
        return Ok(ExpressionData::from_term(
            term.bind(py).borrow().data.clone_ref(py),
        ));
    }

    if let Ok(expression) = other.extract::<Py<Expression>>() {
        return Ok(expression.bind(py).borrow().data.clone_ref(py));
    }

    Err(PyTypeError::new_err(
        "expected a number, Variable, Term, or Expression",
    ))
}
