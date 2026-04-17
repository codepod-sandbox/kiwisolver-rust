import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi
import pytest


def assert_term_shape(term, variable, coefficient):
    assert term.variable() is variable
    assert term.coefficient() == pytest.approx(coefficient)


def assert_expression_shape(expression, constant, terms):
    assert expression.constant() == pytest.approx(constant)
    actual_terms = expression.terms()
    assert len(actual_terms) == len(terms)
    for actual, (variable, coefficient) in zip(actual_terms, terms):
        assert_term_shape(actual, variable, coefficient)


# Upstream: tests/ExpressionTest.cpp :: ExpressionCreation
def test_expression_creation_preserves_constant_term_order_and_coefficients():
    foo = kiwi.Variable("foo")
    bar = kiwi.Variable("bar")
    aux = kiwi.Variable("aux")

    expression_without_constant = kiwi.Expression(
        [kiwi.Term(foo, 1), kiwi.Term(bar, 2), kiwi.Term(aux, 3)]
    )
    expression_with_constant = kiwi.Expression(
        [kiwi.Term(foo, 1), kiwi.Term(bar, 2), kiwi.Term(aux, 3)],
        10,
    )

    assert_expression_shape(
        expression_without_constant,
        0,
        [(foo, 1), (bar, 2), (aux, 3)],
    )
    assert_expression_shape(
        expression_with_constant,
        10,
        [(foo, 1), (bar, 2), (aux, 3)],
    )


# Upstream: tests/ExpressionTest.cpp :: ExpressionNeg
def test_expression_negation_flips_constant_and_coefficients():
    foo = kiwi.Variable("foo")

    negated = -(kiwi.Term(foo, 10) + 5)

    assert_expression_shape(negated, -5, [(foo, -10)])


# Upstream: tests/ExpressionTest.cpp :: ExpressionMul
def test_expression_multiplication_scales_terms_from_both_sides():
    foo = kiwi.Variable("foo")
    expression = kiwi.Term(foo, 10) + 5

    assert_expression_shape(expression * 2.0, 10, [(foo, 20)])
    assert_expression_shape(2.0 * expression, 10, [(foo, 20)])


# Upstream: tests/ExpressionTest.cpp :: ExpressionDiv
def test_expression_division_scales_terms_and_constant():
    foo = kiwi.Variable("foo")
    expression = kiwi.Term(foo, 10) + 5

    assert_expression_shape(expression / 2, 2.5, [(foo, 5)])


# Upstream: tests/ExpressionTest.cpp :: ExpressionRichCompareOperations
def test_expression_rich_compare_creates_required_constraints():
    foo = kiwi.Variable("foo")
    bar = kiwi.Variable("bar")
    left = kiwi.Term(foo, 10) + 5
    right = bar - 10

    less_equal = left <= right
    equal = left == right
    greater_equal = left >= right

    assert less_equal.op() == "<="
    assert less_equal.strength() == pytest.approx(kiwi.strength.required)
    assert_expression_shape(less_equal.expression(), 15, [(foo, 10), (bar, -1)])

    assert equal.op() == "=="
    assert equal.strength() == pytest.approx(kiwi.strength.required)

    assert greater_equal.op() == ">="
    assert greater_equal.strength() == pytest.approx(kiwi.strength.required)


# Upstream: tests/SolverTest.cpp :: ManagingConstraints
def test_solver_constraint_management_matches_upstream_contract():
    solver = kiwi.Solver()
    foo = kiwi.Variable("foo")
    lower_bound = foo >= 1
    upper_bound = foo <= 0

    assert solver.hasConstraint(lower_bound) is False

    solver.addConstraint(lower_bound)
    assert solver.hasConstraint(lower_bound) is True

    with pytest.raises(kiwi.DuplicateConstraint) as duplicate_exc:
        solver.addConstraint(lower_bound)
    assert duplicate_exc.value.constraint is lower_bound

    with pytest.raises(kiwi.UnknownConstraint) as unknown_exc:
        solver.removeConstraint(upper_bound)
    assert unknown_exc.value.constraint is upper_bound

    with pytest.raises(kiwi.UnsatisfiableConstraint) as unsat_exc:
        solver.addConstraint(upper_bound)
    assert unsat_exc.value.constraint is upper_bound

    solver.removeConstraint(lower_bound)
    assert solver.hasConstraint(lower_bound) is False

    solver.addConstraint(upper_bound)
    assert solver.hasConstraint(upper_bound) is True

    solver.reset()
    assert solver.hasConstraint(upper_bound) is False
