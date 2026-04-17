# Upstream Solver Corpus

The audited `kiwisolver 1.5.0` release snapshot does not ship a standalone Python
test suite. The authoritative upstream behavior corpus is the C++ solver test set:

- `tests/SimplexTest.cpp`
- `tests/VariableTest.cpp`
- `tests/TermTest.cpp`
- `tests/ExpressionTest.cpp`
- `tests/ConstraintTest.cpp`
- `tests/StrengthTest.cpp`
- `tests/SolverTest.cpp`

Later tasks should translate these cases into Rust-host and Python compatibility
tests rather than assume an upstream Python suite exists to vendor verbatim.

Current local tranche coverage:
- `tests/python/test_upstream_solver_tranche1.py`
  Covers `StrengthTest.cpp :: CreatingStrength`, `ConstraintTest.cpp :: ConstraintCreationWithStrength`, `ConstraintTest.cpp :: ConstraintOrOperator`, and `SolverTest.cpp :: ManagingEditVariable`.
- `tests/python/test_upstream_expression_tranche2.py`
  Covers `ExpressionTest.cpp :: ExpressionCreation`, `ExpressionTest.cpp :: ExpressionNeg`, `ExpressionTest.cpp :: ExpressionMul`, `ExpressionTest.cpp :: ExpressionDiv`, `ExpressionTest.cpp :: ExpressionAddition`, `ExpressionTest.cpp :: ExpressionSubtraction`, `ExpressionTest.cpp :: ExpressionRichCompareOperations`, and `SolverTest.cpp :: ManagingConstraints`.
