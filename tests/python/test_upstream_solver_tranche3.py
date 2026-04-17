import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi


# Upstream: tests/SolverTest.cpp :: SuggestingValuesForEditVariables
def test_solver_suggesting_values_for_edit_variables_matches_upstream():
    solver = kiwi.Solver()
    first = kiwi.Variable("foo")

    solver.addEditVariable(first, kiwi.strength.medium)
    solver.addConstraint((first == 1) | kiwi.strength.weak)
    solver.suggestValue(first, 2)
    solver.updateVariables()
    assert first.value() == 2

    solver.reset()
    second = kiwi.Variable("bar")

    solver.addEditVariable(second, kiwi.strength.weak)
    solver.addConstraint(first + second == 0)
    solver.addConstraint(second <= -1)
    solver.addConstraint((second >= 0) | kiwi.strength.weak)
    solver.suggestValue(second, 0)
    solver.updateVariables()

    assert second.value() == -1


# Upstream: tests/SolverTest.cpp :: SolvingUnderConstrainedSystem
def test_solver_solving_under_constrained_system_matches_upstream():
    solver = kiwi.Solver()
    variable = kiwi.Variable("foo")
    constraint = 2 * variable + 1 >= 0

    solver.addEditVariable(variable, kiwi.strength.weak)
    solver.addConstraint(constraint)
    solver.suggestValue(variable, 10)
    solver.updateVariables()

    assert constraint.expression().value() == 21
    assert constraint.expression().terms()[0].value() == 20
    assert variable.value() == 10


# Upstream: tests/SolverTest.cpp :: HandlingInfeasibleConstraints
def test_solver_handling_infeasible_constraints_matches_upstream():
    middle = kiwi.Variable("xm")
    left = kiwi.Variable("xl")
    right = kiwi.Variable("xr")
    solver = kiwi.Solver()

    solver.addEditVariable(middle, kiwi.strength.strong)
    solver.addEditVariable(left, kiwi.strength.weak)
    solver.addEditVariable(right, kiwi.strength.weak)
    solver.addConstraint(2 * middle == left + right)
    solver.addConstraint(left + 20 <= right)
    solver.addConstraint(left >= -10)
    solver.addConstraint(right <= 100)

    solver.suggestValue(middle, 40)
    solver.suggestValue(right, 50)
    solver.suggestValue(left, 30)
    solver.suggestValue(middle, 60)
    solver.suggestValue(middle, 90)
    solver.updateVariables()

    assert left.value() + right.value() == 2 * middle.value()
    assert left.value() == 80
    assert right.value() == 100


# Upstream: tests/SolverTest.cpp :: ConstraintViolated
def test_constraint_violated_matches_upstream():
    solver = kiwi.Solver()
    variable = kiwi.Variable("foo")

    required = (variable >= 10) | kiwi.strength.required
    weak = (variable <= -5) | kiwi.strength.weak

    solver.addConstraint(required)
    solver.addConstraint(weak)
    solver.updateVariables()

    assert variable.value() == 10
    assert required.violated() is False
    assert weak.violated() is True
