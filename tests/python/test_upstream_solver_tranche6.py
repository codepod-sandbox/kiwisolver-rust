import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi
import pytest


# Upstream: tests/SolverTest.cpp :: SolverCreation
def test_solver_creation_matches_upstream():
    solver = kiwi.Solver()

    assert isinstance(solver, kiwi.Solver)


# Upstream: tests/SolverTest.cpp :: DumpingSolver
def test_solver_dumping_matches_upstream_headers(capsys):
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

    assert "Objective" in state
    assert "Tableau" in state
    assert "Variables" in state
    assert "Constraints" in state
    assert "bar =" in state
    assert "1*bar + 1*foo == 0" in state

    solver.dump()
    captured = capsys.readouterr()

    assert captured.out == state
