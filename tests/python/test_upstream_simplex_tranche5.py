import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi
import pytest


# Upstream: tests/SimplexTest.cpp :: Maximization
def test_simplex_maximization_matches_upstream():
    first = kiwi.Variable("x1")
    second = kiwi.Variable("x2")
    third = kiwi.Variable("x3")
    objective = kiwi.Variable("z")

    solver = kiwi.Solver()
    solver.addConstraint(first >= 0)
    solver.addConstraint(second >= 0)
    solver.addConstraint(third >= 0)
    solver.addConstraint(2 * first - 5 * second <= 11)
    solver.addConstraint(-first + 3 * second + third == 7)
    solver.addConstraint(first - 8 * second + 4 * third >= 33)
    solver.addConstraint(objective == -2 * first + 7 * second + 4 * third)
    solver.addEditVariable(objective, kiwi.strength.weak)
    solver.suggestValue(objective, 1e6)
    solver.updateVariables()

    assert first.value() == pytest.approx(13, abs=1e-4)
    assert second.value() == pytest.approx(3, abs=1e-4)
    assert third.value() == pytest.approx(11, abs=1e-4)
    assert objective.value() == pytest.approx(39, abs=1e-4)
