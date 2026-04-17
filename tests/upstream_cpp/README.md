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
- `tests/python/test_upstream_solver_tranche2.py`
  Covers `SolverTest.cpp :: SolvingWithStrength`.
- `tests/python/test_upstream_solver_tranche3.py`
  Covers `SolverTest.cpp :: SuggestingValuesForEditVariables`, `SolverTest.cpp :: SolvingUnderConstrainedSystem`, `SolverTest.cpp :: HandlingInfeasibleConstraints`, and `SolverTest.cpp :: ConstraintViolated`.
- `tests/python/test_upstream_surface_tranche4.py`
  Covers `StrengthTest.cpp :: AccessingPredefinedStrength`, `ConstraintTest.cpp :: ConstraintCreationEQ`, `ConstraintTest.cpp :: ConstraintCreationLE`, `ConstraintTest.cpp :: ConstraintCreationGE`, `VariableTest.cpp :: VariableMethods`, `VariableTest.cpp :: VariableNeg`, `VariableTest.cpp :: VariableMul`, `VariableTest.cpp :: VariableDivision`, `VariableTest.cpp :: VariableAddition`, `VariableTest.cpp :: VariableSubtraction`, `VariableTest.cpp :: VariableRichCompareOperations`, `TermTest.cpp :: TermCreation`, `TermTest.cpp :: TermNeg`, `TermTest.cpp :: TermMul`, `TermTest.cpp :: TermDiv`, `TermTest.cpp :: TermAdd`, `TermTest.cpp :: TermSub`, and `TermTest.cpp :: TermRichCompareOperations`.
- `tests/python/test_upstream_simplex_tranche5.py`
  Covers `SimplexTest.cpp :: Maximization`.
- `tests/python/test_upstream_solver_tranche6.py`
  Covers `SolverTest.cpp :: SolverCreation` and `SolverTest.cpp :: DumpingSolver`.
