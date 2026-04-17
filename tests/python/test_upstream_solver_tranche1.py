import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi
import pytest


# Upstream: tests/StrengthTest.cpp :: CreatingStrength
def test_strength_create_is_monotonic_across_components_and_weight():
    weak = kiwi.strength.create(0, 0, 1)
    medium = kiwi.strength.create(0, 1, 0)
    strong = kiwi.strength.create(1, 0, 0)
    weighted_strong = kiwi.strength.create(1, 0, 0, 4)

    assert weak < medium < strong < kiwi.strength.required
    assert strong < weighted_strong


# Upstream: tests/ConstraintTest.cpp :: ConstraintCreationWithStrength
def test_constraint_creation_preserves_explicit_strengths():
    variable = kiwi.Variable("foo")

    for strength in (
        kiwi.strength.weak,
        kiwi.strength.medium,
        kiwi.strength.strong,
        kiwi.strength.required,
    ):
        constraint = kiwi.Constraint(variable + 1, "==", strength)
        assert constraint.strength() == strength


# Upstream: tests/ConstraintTest.cpp :: ConstraintOrOperator
def test_constraint_or_operator_preserves_explicit_strengths():
    variable = kiwi.Variable("foo")
    constraint = kiwi.Constraint(variable + 1, "==")

    for strength in (
        kiwi.strength.weak,
        kiwi.strength.medium,
        kiwi.strength.strong,
        kiwi.strength.required,
        kiwi.strength.create(1, 1, 0),
    ):
        rebound = constraint | strength
        assert rebound.strength() == strength


# Upstream: tests/SolverTest.cpp :: ManagingEditVariable
def test_solver_edit_variable_management_matches_upstream_contract():
    solver = kiwi.Solver()
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")

    assert solver.hasEditVariable(first) is False

    solver.addEditVariable(first, kiwi.strength.weak)
    assert solver.hasEditVariable(first) is True

    with pytest.raises(kiwi.DuplicateEditVariable) as duplicate_exc:
        solver.addEditVariable(first, kiwi.strength.medium)
    assert duplicate_exc.value.edit_variable is first

    with pytest.raises(kiwi.UnknownEditVariable) as unknown_remove_exc:
        solver.removeEditVariable(second)
    assert unknown_remove_exc.value.edit_variable is second

    solver.removeEditVariable(first)
    assert solver.hasEditVariable(first) is False

    with pytest.raises(kiwi.BadRequiredStrength):
        solver.addEditVariable(first, kiwi.strength.required)

    solver.addEditVariable(second, kiwi.strength.strong)
    assert solver.hasEditVariable(second) is True

    with pytest.raises(kiwi.UnknownEditVariable) as unknown_suggest_exc:
        solver.suggestValue(first, 10)
    assert unknown_suggest_exc.value.edit_variable is first

    solver.reset()
    assert solver.hasEditVariable(second) is False


def test_solver_dumps_is_explicitly_non_upstream_fallback_summary():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    solver = kiwi.Solver()

    solver.addEditVariable(second, kiwi.strength.weak)
    solver.addConstraint(first + second == 0)
    solver.addConstraint(second <= -1)
    solver.addConstraint((second >= 0) | kiwi.strength.weak)
    solver.updateVariables()

    with pytest.raises(kiwi.UnsatisfiableConstraint):
        solver.addConstraint(second >= 1)

    state = solver.dumps()

    assert "Fallback solver summary" in state
    assert "backend dump unavailable" in state
    assert "foo" in state
    assert "bar" in state


def test_solver_dumps_does_not_retain_stale_variable_names_after_solver_is_emptied():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    solver = kiwi.Solver()
    constraint = first + second == 0

    solver.addEditVariable(second, kiwi.strength.weak)
    solver.addConstraint(constraint)

    active_state = solver.dumps()
    assert "foo" in active_state
    assert "bar" in active_state

    solver.removeConstraint(constraint)
    solver.removeEditVariable(second)

    emptied_state = solver.dumps()
    assert "foo" not in emptied_state
    assert "bar" not in emptied_state
