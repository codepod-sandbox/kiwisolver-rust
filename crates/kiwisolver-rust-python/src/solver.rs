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

    fn dump(&self, py: Python<'_>) {
        println!("{}", self.dumps(py));
    }

    fn dumps(&self, py: Python<'_>) -> String {
        let mut variable_entries = self
            .active_variables(py)
            .into_values()
            .map(|variable| {
                let variable_ref = variable.bind(py).borrow();
                format!(
                    "{} = {}",
                    variable_ref.name_ref(),
                    variable_ref.current_value()
                )
            })
            .collect::<Vec<_>>();
        variable_entries.sort();

        let mut constraint_entries = self
            .constraints
            .iter()
            .map(|constraint| {
                let constraint_ref = constraint.bind(py).borrow();
                format!(
                    "{} @ {}",
                    constraint_ref.op_str(),
                    constraint_ref.strength_value()
                )
            })
            .collect::<Vec<_>>();
        constraint_entries.sort();

        let mut edit_variable_entries = self
            .edit_variables
            .iter()
            .map(|variable| variable.bind(py).borrow().name_ref().to_owned())
            .collect::<Vec<_>>();
        edit_variable_entries.sort();

        let objective_body = format!("  edit_variables: {}", edit_variable_entries.len());
        let tableau_body = format!("  rows: {}", self.constraints.len());
        let variables_body = if variable_entries.is_empty() {
            "  <none>".to_owned()
        } else {
            format!("  {}", variable_entries.join("\n  "))
        };
        let constraints_body = format!("  count: {}", constraint_entries.len());

        format!(
            "Objective\n{objective_body}\nTableau\n{tableau_body}\nVariables\n{variables_body}\nConstraints\n{constraints_body}"
        )
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
