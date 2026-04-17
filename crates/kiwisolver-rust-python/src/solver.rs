use std::collections::HashMap;

use cassowary::{
    AddConstraintError, AddEditVariableError, RemoveConstraintError, RemoveEditVariableError,
    Solver as CassowarySolver, SuggestValueError, Variable as CassowaryVariable,
};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::constraint::{resolve_strength, Constraint};
use crate::errors;
use crate::variable::Variable;

#[pyclass(module = "kiwisolver._kiwisolver_native", unsendable)]
pub struct Solver {
    solver: CassowarySolver,
    variables: HashMap<CassowaryVariable, Py<Variable>>,
    constraints: Vec<Py<Constraint>>,
    edit_variables: Vec<Py<Variable>>,
}

#[pymethods]
impl Solver {
    #[new]
    fn new() -> Self {
        Self {
            solver: CassowarySolver::new(),
            variables: HashMap::new(),
            constraints: Vec::new(),
            edit_variables: Vec::new(),
        }
    }

    #[pyo3(name = "addConstraint")]
    fn add_constraint(&mut self, py: Python<'_>, constraint: Py<Constraint>) -> PyResult<()> {
        let (backend, variables) = {
            let constraint_ref = constraint.bind(py).borrow();
            (
                constraint_ref.backend_constraint(),
                constraint_ref.tracked_variables(py),
            )
        };

        self.solver
            .add_constraint(backend)
            .map_err(|err| map_add_constraint_error(py, err, constraint.clone_ref(py)))?;

        for variable in variables {
            self.track_variable(py, variable);
        }
        self.constraints.push(constraint);

        Ok(())
    }

    #[pyo3(name = "removeConstraint")]
    fn remove_constraint(&mut self, py: Python<'_>, constraint: Py<Constraint>) -> PyResult<()> {
        let backend = constraint.bind(py).borrow().backend_constraint();
        self.solver
            .remove_constraint(&backend)
            .map_err(|err| map_remove_constraint_error(py, err, constraint.clone_ref(py)))?;
        self.constraints
            .retain(|existing| existing.as_ptr() != constraint.as_ptr());
        self.prune_tracked_variables(py);
        Ok(())
    }

    #[pyo3(name = "hasConstraint")]
    fn has_constraint(&self, py: Python<'_>, constraint: Py<Constraint>) -> bool {
        let backend = constraint.bind(py).borrow().backend_constraint();
        self.solver.has_constraint(&backend)
    }

    #[pyo3(name = "addEditVariable")]
    fn add_edit_variable(
        &mut self,
        py: Python<'_>,
        variable: Py<Variable>,
        strength: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let strength = resolve_strength(py, strength)?;
        let backend = Self::backend_variable(py, &variable);
        self.solver
            .add_edit_variable(backend, strength)
            .map_err(|err| map_add_edit_variable_error(py, err, variable.clone_ref(py)))?;
        self.track_variable(py, variable.clone_ref(py));
        self.edit_variables.push(variable);
        Ok(())
    }

    #[pyo3(name = "removeEditVariable")]
    fn remove_edit_variable(&mut self, py: Python<'_>, variable: Py<Variable>) -> PyResult<()> {
        let backend = Self::backend_variable(py, &variable);
        self.solver
            .remove_edit_variable(backend)
            .map_err(|err| map_remove_edit_variable_error(py, err, variable.clone_ref(py)))?;
        self.edit_variables
            .retain(|existing| existing.as_ptr() != variable.as_ptr());
        self.prune_tracked_variables(py);
        Ok(())
    }

    #[pyo3(name = "hasEditVariable")]
    fn has_edit_variable(&self, py: Python<'_>, variable: Py<Variable>) -> bool {
        let backend = Self::backend_variable(py, &variable);
        self.solver.has_edit_variable(&backend)
    }

    #[pyo3(name = "suggestValue")]
    fn suggest_value(
        &mut self,
        py: Python<'_>,
        variable: Py<Variable>,
        value: f64,
    ) -> PyResult<()> {
        let backend = Self::backend_variable(py, &variable);
        self.solver
            .suggest_value(backend, value)
            .map_err(|err| map_suggest_value_error(py, err, variable))
    }

    #[pyo3(name = "updateVariables")]
    fn update_variables(&mut self, py: Python<'_>) {
        let _ = self.solver.fetch_changes();

        for (&backend, variable) in &self.variables {
            let value = self.solver.get_value(backend);
            variable.bind(py).borrow_mut().set_current_value(value);
        }
    }

    fn reset(&mut self, py: Python<'_>) {
        self.solver.reset();

        for variable in self.variables.values() {
            variable.bind(py).borrow_mut().set_current_value(0.0);
        }

        self.variables.clear();
        self.constraints.clear();
        self.edit_variables.clear();
    }

    fn dump(&self, py: Python<'_>) -> PyResult<()> {
        let state = self.dumps(py)?;
        let sys = py.import("sys")?;
        sys.getattr("stdout")?.call_method1("write", (state,))?;
        Ok(())
    }

    fn dumps(&self, py: Python<'_>) -> PyResult<String> {
        let mut lines = self
            .solver
            .debug_dump()
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>();

        lines.push("Variables".to_owned());
        for line in self.variable_dump_lines(py) {
            lines.push(line);
        }
        lines.push(String::new());

        lines.push("Constraints".to_owned());
        for line in self.constraint_dump_lines(py) {
            lines.push(line);
        }
        lines.push(String::new());

        Ok(lines.join("\n"))
    }
}

impl Solver {
    fn backend_variable(py: Python<'_>, variable: &Py<Variable>) -> CassowaryVariable {
        variable.bind(py).borrow().backend_variable()
    }

    fn track_variable(&mut self, py: Python<'_>, variable: Py<Variable>) {
        let backend = Self::backend_variable(py, &variable);
        self.variables
            .entry(backend)
            .or_insert_with(|| variable.clone_ref(py));
    }

    fn active_variables(&self, py: Python<'_>) -> HashMap<CassowaryVariable, Py<Variable>> {
        let mut active_variables = HashMap::new();

        for constraint in &self.constraints {
            let constraint_ref = constraint.bind(py).borrow();
            for variable in constraint_ref.tracked_variables(py) {
                let backend = Self::backend_variable(py, &variable);
                active_variables.entry(backend).or_insert(variable);
            }
        }

        for variable in &self.edit_variables {
            let backend = Self::backend_variable(py, variable);
            active_variables
                .entry(backend)
                .or_insert_with(|| variable.clone_ref(py));
        }

        active_variables
    }

    fn prune_tracked_variables(&mut self, py: Python<'_>) {
        let active_variables = self.active_variables(py);
        self.variables
            .retain(|backend, _| active_variables.contains_key(backend));
    }

    fn variable_dump_lines(&self, py: Python<'_>) -> Vec<String> {
        let mut variables = self
            .variables
            .iter()
            .map(|(&backend, variable)| {
                let variable_ref = variable.bind(py).borrow();
                (
                    variable_ref.name_ref().to_owned(),
                    self.solver.get_value(backend),
                )
            })
            .collect::<Vec<_>>();
        variables.sort_by(|left, right| left.0.cmp(&right.0));

        if variables.is_empty() {
            return vec!["  <none>".to_owned()];
        }

        variables
            .into_iter()
            .map(|(name, value)| format!("  {name} = {value}"))
            .collect()
    }

    fn constraint_dump_lines(&self, py: Python<'_>) -> Vec<String> {
        let mut constraints = self
            .constraints
            .iter()
            .map(|constraint| {
                let constraint_ref = constraint.bind(py).borrow();
                let expression = constraint_ref.expression_data();
                let mut term_strings = expression
                    .terms
                    .iter()
                    .map(|term| {
                        let variable = term.variable.bind(py).borrow();
                        format!("{}*{}", term.coefficient, variable.name_ref())
                    })
                    .collect::<Vec<_>>();
                term_strings.sort();
                format!(
                    "  {} {} 0 [strength={}]",
                    if term_strings.is_empty() {
                        expression.constant.to_string()
                    } else if expression.constant == 0.0 {
                        term_strings.join(" + ")
                    } else {
                        format!("{} + {}", term_strings.join(" + "), expression.constant)
                    },
                    constraint_ref.op_str(),
                    constraint_ref.strength_value()
                )
            })
            .collect::<Vec<_>>();
        constraints.sort();

        if constraints.is_empty() {
            return vec!["  <none>".to_owned()];
        }

        constraints
    }
}

fn map_add_constraint_error(
    py: Python<'_>,
    err: AddConstraintError,
    constraint: Py<Constraint>,
) -> PyErr {
    match err {
        AddConstraintError::DuplicateConstraint => {
            errors::constraint_error(py, "DuplicateConstraint", constraint)
        }
        AddConstraintError::UnsatisfiableConstraint => {
            errors::constraint_error(py, "UnsatisfiableConstraint", constraint)
        }
        AddConstraintError::InternalSolverError(detail) => {
            PyRuntimeError::new_err(format!("internal solver error: {detail}"))
        }
    }
}

fn map_remove_constraint_error(
    py: Python<'_>,
    err: RemoveConstraintError,
    constraint: Py<Constraint>,
) -> PyErr {
    match err {
        RemoveConstraintError::UnknownConstraint => {
            errors::constraint_error(py, "UnknownConstraint", constraint)
        }
        RemoveConstraintError::InternalSolverError(detail) => {
            PyRuntimeError::new_err(format!("internal solver error: {detail}"))
        }
    }
}

fn map_add_edit_variable_error(
    py: Python<'_>,
    err: AddEditVariableError,
    variable: Py<Variable>,
) -> PyErr {
    match err {
        AddEditVariableError::DuplicateEditVariable => {
            errors::edit_variable_error(py, "DuplicateEditVariable", variable)
        }
        AddEditVariableError::BadRequiredStrength => exception(
            py,
            "BadRequiredStrength",
            "edit variable strength cannot be required",
        ),
    }
}

fn map_remove_edit_variable_error(
    py: Python<'_>,
    err: RemoveEditVariableError,
    variable: Py<Variable>,
) -> PyErr {
    match err {
        RemoveEditVariableError::UnknownEditVariable => {
            errors::edit_variable_error(py, "UnknownEditVariable", variable)
        }
        RemoveEditVariableError::InternalSolverError(detail) => {
            PyRuntimeError::new_err(format!("internal solver error: {detail}"))
        }
    }
}

fn map_suggest_value_error(
    py: Python<'_>,
    err: SuggestValueError,
    variable: Py<Variable>,
) -> PyErr {
    match err {
        SuggestValueError::UnknownEditVariable => {
            errors::edit_variable_error(py, "UnknownEditVariable", variable)
        }
        SuggestValueError::InternalSolverError(detail) => {
            PyRuntimeError::new_err(format!("internal solver error: {detail}"))
        }
    }
}

fn exception(py: Python<'_>, name: &str, message: &str) -> PyErr {
    match errors::get_exception_type(py, name) {
        Ok(exc) => PyErr::from_type(exc, message.to_owned()),
        Err(err) => err,
    }
}
