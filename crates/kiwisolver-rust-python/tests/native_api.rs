use cassowary::strength::REQUIRED;
use cassowary::{AddConstraintError, Constraint, RelationalOperator, Solver, Variable};

#[test]
fn cassowary_backend_solves_equalities_and_detects_duplicates() {
    let width = Variable::new();
    let constraint = Constraint::new(width - 42.0, RelationalOperator::Equal, REQUIRED);
    let mut solver = Solver::new();

    solver.add_constraint(constraint.clone()).unwrap();

    assert_eq!(solver.get_value(width), 42.0);

    let duplicate = solver.add_constraint(constraint);
    assert!(matches!(
        duplicate,
        Err(AddConstraintError::DuplicateConstraint)
    ));
}
