import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi


# Upstream: tests/SolverTest.cpp :: SolvingWithStrength
def test_solving_with_strength_prefers_required_constraint_over_weak_preference():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    solver = kiwi.Solver()

    solver.addConstraint(first + second == 0)
    solver.addConstraint(first == 10)
    solver.addConstraint((second >= 0) | kiwi.strength.weak)

    solver.updateVariables()

    assert first.value() == 10
    assert second.value() == -10


# Upstream: tests/SolverTest.cpp :: SolvingWithStrength
def test_solving_with_strength_prefers_stronger_optional_constraint():
    first = kiwi.Variable("foo")
    second = kiwi.Variable("bar")
    solver = kiwi.Solver()

    solver.addConstraint(first + second == 0)
    solver.addConstraint((first >= 10) | kiwi.strength.medium)
    solver.addConstraint((second == 2) | kiwi.strength.strong)

    solver.updateVariables()

    assert first.value() == -2
    assert second.value() == 2
