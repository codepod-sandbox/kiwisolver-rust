import importlib
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi
import pytest


native = importlib.import_module("kiwisolver._kiwisolver_native")


def test_variable_name_round_trip():
    var = kiwi.Variable("width")
    assert var.name() == "width"


def test_strength_required_is_numeric():
    assert isinstance(kiwi.strength.required, (int, float))
    assert isinstance(native.strength.required, (int, float))


def test_duplicate_constraint_error_exists():
    assert hasattr(kiwi, "DuplicateConstraint")
    assert hasattr(native, "DuplicateConstraint")


def test_native_duplicate_constraint_matches_public_exception():
    assert native.DuplicateConstraint is kiwi.DuplicateConstraint

    with pytest.raises(kiwi.DuplicateConstraint):
        raise native.DuplicateConstraint("duplicate")


def test_strength_create_rejects_out_of_range_components():
    with pytest.raises(kiwi.BadRequiredStrength):
        native.strength.create(1001, 0, 0)


def test_variable_multiplication_creates_term():
    width = kiwi.Variable("width")

    term = 2 * width

    assert isinstance(term, kiwi.Term)


def test_expression_addition_creates_expression():
    width = kiwi.Variable("width")

    expr = 2 * width + 10

    assert isinstance(expr, kiwi.Expression)


def test_constraint_creation_with_required_strength():
    width = kiwi.Variable("width")

    constraint = (width >= 10) | "required"

    assert isinstance(constraint, kiwi.Constraint)


def test_variable_negation_creates_term():
    width = kiwi.Variable("width")

    term = -width

    assert isinstance(term, kiwi.Term)


def test_variable_division_creates_term():
    width = kiwi.Variable("width")

    term = width / 2

    assert isinstance(term, kiwi.Term)


def test_constraint_rejects_unknown_operator():
    width = kiwi.Variable("width")
    expr = 2 * width + 10

    with pytest.raises(ValueError):
        native.Constraint(expr, "!=")


def test_constraint_rejects_out_of_range_numeric_strength():
    width = kiwi.Variable("width")
    expr = 2 * width + 10

    with pytest.raises(kiwi.BadRequiredStrength):
        native.Constraint(expr, "==", native.strength.required + 1)


def test_solver_updates_variable_values():
    width = kiwi.Variable("width")
    solver = kiwi.Solver()

    solver.addConstraint(width == 42)
    solver.updateVariables()

    assert width.value() == 42


def test_duplicate_constraint_raises():
    width = kiwi.Variable("width")
    constraint = width >= 10
    solver = kiwi.Solver()

    solver.addConstraint(constraint)

    with pytest.raises(kiwi.DuplicateConstraint):
        solver.addConstraint(constraint)
