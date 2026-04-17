import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi


def assert_term_shape(term, variable, coefficient):
    assert term.variable() is variable
    assert term.coefficient() == coefficient


def assert_expression_shape(expression, constant, terms):
    assert expression.constant() == constant
    actual_terms = expression.terms()
    assert len(actual_terms) == len(terms)
    for actual, (variable, coefficient) in zip(actual_terms, terms):
        assert_term_shape(actual, variable, coefficient)


# Upstream: tests/StrengthTest.cpp :: AccessingPredefinedStrength
def test_strength_accessing_predefined_strength_matches_upstream():
    assert kiwi.strength.weak < kiwi.strength.medium
    assert kiwi.strength.medium < kiwi.strength.strong
    assert kiwi.strength.strong < kiwi.strength.required


# Upstream: tests/ConstraintTest.cpp :: ConstraintCreationEQ
def test_constraint_creation_eq_matches_upstream():
    variable = kiwi.Variable("foo")
    constraint = kiwi.Constraint(variable + 1, "==")

    assert constraint.strength() == kiwi.strength.required
    assert constraint.op() == "=="
    assert_expression_shape(constraint.expression(), 1, [(variable, 1)])


# Upstream: tests/ConstraintTest.cpp :: ConstraintCreationLE
def test_constraint_creation_le_matches_upstream():
    variable = kiwi.Variable("foo")
    constraint = kiwi.Constraint(variable + 1, "<=")

    assert constraint.strength() == kiwi.strength.required
    assert constraint.op() == "<="
    assert_expression_shape(constraint.expression(), 1, [(variable, 1)])


# Upstream: tests/ConstraintTest.cpp :: ConstraintCreationGE
def test_constraint_creation_ge_matches_upstream():
    variable = kiwi.Variable("foo")
    constraint = kiwi.Constraint(variable + 1, ">=")

    assert constraint.strength() == kiwi.strength.required
    assert constraint.op() == ">="
    assert_expression_shape(constraint.expression(), 1, [(variable, 1)])


# Upstream: tests/VariableTest.cpp :: VariableMethods
def test_variable_methods_match_upstream():
    variable = kiwi.Variable()

    assert variable.name() == ""

    variable.setName("foo")
    assert variable.name() == "foo"
    assert variable.value() == 0

    assert variable.context() is None
    context = object()
    variable.setContext(context)
    assert variable.context() is context


# Upstream: tests/VariableTest.cpp :: VariableNeg
def test_variable_neg_matches_upstream():
    variable = kiwi.Variable("foo")

    assert_term_shape(-variable, variable, -1)


# Upstream: tests/VariableTest.cpp :: VariableMul
def test_variable_mul_matches_upstream():
    variable = kiwi.Variable("foo")

    assert_term_shape(variable * 2, variable, 2)
    assert_term_shape(2 * variable, variable, 2)


# Upstream: tests/VariableTest.cpp :: VariableDivision
def test_variable_division_matches_upstream():
    variable = kiwi.Variable("foo")

    assert_term_shape(variable / 2, variable, 0.5)


# Upstream: tests/VariableTest.cpp :: VariableAddition
def test_variable_addition_matches_upstream():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")

    assert_expression_shape(first + 2, 2, [(first, 1)])
    assert_expression_shape(2 + first, 2, [(first, 1)])
    assert_expression_shape(first + second, 0, [(first, 1), (second, 1)])


# Upstream: tests/VariableTest.cpp :: VariableSubtraction
def test_variable_subtraction_matches_upstream():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")

    assert_expression_shape(first - 2, -2, [(first, 1)])
    assert_expression_shape(2 - first, 2, [(first, -1)])
    assert_expression_shape(first - second, 0, [(first, 1), (second, -1)])


# Upstream: tests/VariableTest.cpp :: VariableRichCompareOperations
def test_variable_rich_compare_operations_match_upstream():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")

    less_equal = second + 1 <= first
    equal = second + 1 == first
    greater_equal = second + 1 >= first

    assert less_equal.op() == "<="
    assert less_equal.strength() == kiwi.strength.required
    assert_expression_shape(less_equal.expression(), 1, [(second, 1), (first, -1)])

    assert equal.op() == "=="
    assert equal.strength() == kiwi.strength.required
    assert_expression_shape(equal.expression(), 1, [(second, 1), (first, -1)])

    assert greater_equal.op() == ">="
    assert greater_equal.strength() == kiwi.strength.required
    assert_expression_shape(greater_equal.expression(), 1, [(second, 1), (first, -1)])


# Upstream: tests/TermTest.cpp :: TermCreation
def test_term_creation_matches_upstream():
    variable = kiwi.Variable("foo")

    assert_term_shape(kiwi.Term(variable), variable, 1)
    assert_term_shape(kiwi.Term(variable, 100), variable, 100)


# Upstream: tests/TermTest.cpp :: TermNeg
def test_term_neg_matches_upstream():
    variable = kiwi.Variable("foo")
    term = kiwi.Term(variable, 10)

    assert_term_shape(-term, variable, -10)


# Upstream: tests/TermTest.cpp :: TermMul
def test_term_mul_matches_upstream():
    variable = kiwi.Variable("foo")
    term = kiwi.Term(variable, 10)

    assert_term_shape(term * 2, variable, 20)
    assert_term_shape(2 * term, variable, 20)


# Upstream: tests/TermTest.cpp :: TermDiv
def test_term_div_matches_upstream():
    variable = kiwi.Variable("foo")
    term = kiwi.Term(variable, 10)

    assert_term_shape(term / 2, variable, 5)


# Upstream: tests/TermTest.cpp :: TermAdd
def test_term_add_matches_upstream():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    term = kiwi.Term(first, 10)
    second_term = kiwi.Term(second)

    assert_expression_shape(term + 2, 2, [(first, 10)])
    assert_expression_shape(2 + term, 2, [(first, 10)])
    assert_expression_shape(term + second, 0, [(first, 10), (second, 1)])
    assert_expression_shape(second + term, 0, [(second, 1), (first, 10)])
    assert_expression_shape(term + second_term, 0, [(first, 10), (second, 1)])


# Upstream: tests/TermTest.cpp :: TermSub
def test_term_sub_matches_upstream():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    term = kiwi.Term(first, 10)
    second_term = kiwi.Term(second)

    assert_expression_shape(term - 2, -2, [(first, 10)])
    assert_expression_shape(2 - term, 2, [(first, -10)])
    assert_expression_shape(term - second, 0, [(first, 10), (second, -1)])
    assert_expression_shape(second - term, 0, [(second, 1), (first, -10)])
    assert_expression_shape(term - second_term, 0, [(first, 10), (second, -1)])


# Upstream: tests/TermTest.cpp :: TermRichCompareOperations
def test_term_rich_compare_operations_match_upstream():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    first_term = kiwi.Term(first, 10)
    second_term = kiwi.Term(second, 20)

    less_equal = first_term <= second_term + 1
    equal = first_term == second_term + 1
    greater_equal = first_term >= second_term + 1

    assert less_equal.op() == "<="
    assert less_equal.strength() == kiwi.strength.required
    assert_expression_shape(less_equal.expression(), -1, [(first, 10), (second, -20)])

    assert equal.op() == "=="
    assert equal.strength() == kiwi.strength.required
    assert_expression_shape(equal.expression(), -1, [(first, 10), (second, -20)])

    assert greater_equal.op() == ">="
    assert greater_equal.strength() == kiwi.strength.required
    assert_expression_shape(greater_equal.expression(), -1, [(first, 10), (second, -20)])
