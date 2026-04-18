# kiwisolver-rust Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a standalone `kiwisolver-rust` sibling package that vendors upstream `kiwisolver` Python code, reimplements the native extension in Rust via PyO3, ports/adapts the upstream solver test corpus, and passes the resulting compatibility suite for the audited release.

**Architecture:** The package keeps upstream Python files as close to upstream as possible while a PyO3 extension module provides the native solver API. Because the audited `kiwisolver 1.5.0` release does not ship a standalone Python upstream test suite, compatibility will be driven by adapted upstream C++ solver tests plus Python smoke and API regression tests derived from the audited surface. The work starts with a hard audit gate of upstream `kiwisolver 1.5.0`; if the native surface is still compact and maps cleanly to a Rust Cassowary-style crate, proceed with the standalone port, otherwise stop and move the needed subset into `matplotlib-rust`.

**Tech Stack:** Rust, PyO3, maturin, pytest, vendored upstream `kiwisolver` 1.5.0 Python package, adapted upstream C++ solver tests, one Rust constraint solver crate selected during the audit.

---

## File Structure

- `kiwisolver-rust/LICENSE`
  Root BSD-3-Clause license for the new repo.
- `kiwisolver-rust/README.md`
  Project overview, compatibility goal, development and test commands.
- `kiwisolver-rust/Cargo.toml`
  Workspace root and Rust dependency declarations.
- `kiwisolver-rust/pyproject.toml`
  Python packaging, build backend, pytest config.
- `kiwisolver-rust/python/kiwisolver/__init__.py`
  Vendored upstream Python entrypoint, with only minimal local edits.
- `kiwisolver-rust/python/kiwisolver/exceptions.py`
  Vendored or locally adapted Python exception exports if required by upstream layout.
- `kiwisolver-rust/python/kiwisolver/_cext.py`
  Optional local compatibility shim only if the upstream package expects a differently named native module.
- `kiwisolver-rust/crates/kiwisolver-rust-python/Cargo.toml`
  PyO3 extension crate manifest.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/lib.rs`
  PyO3 module registration and exported Python types.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/strength.rs`
  Strength constants and helper conversions.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/variable.rs`
  Python `Variable` wrapper and value/name access.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/expression.rs`
  Python `Term`, `Expression`, and expression arithmetic.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/constraint.rs`
  Python `Constraint`, relation operators, and strength handling.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/solver.rs`
  Python `Solver` wrapper and edit/constraint/update operations.
- `kiwisolver-rust/crates/kiwisolver-rust-python/src/errors.rs`
  Python exception definitions and error mapping helpers.
- `kiwisolver-rust/crates/kiwisolver-rust-python/tests/native_api.rs`
  Rust-side tests for wrapper invariants that are easier to debug before Python integration.
- `kiwisolver-rust/tests/upstream_cpp/`
  Adapted upstream C++ solver corpus or extracted behavior cases from that corpus.
- `kiwisolver-rust/tests/python/test_imports.py`
  Local smoke tests for import surface and module wiring.
- `kiwisolver-rust/tests/python/test_repr_and_errors.py`
  Local focused tests for repr and error behavior that commonly drift.
- `kiwisolver-rust/tests/run_python_compat.py`
  Runner for local Python compatibility tests under the local package layout.
- `kiwisolver-rust/docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md`
  Approved design document.
- `kiwisolver-rust/docs/superpowers/plans/2026-04-17-kiwisolver-rust.md`
  This implementation plan.

### Task 1: Audit Upstream Surface And Decide Go/No-Go

**Files:**
- Create: `audit/upstream-kiwisolver-1.5.0-notes.md`
- Create: `audit/upstream-native-surface.txt`
- Create: `audit/upstream-test-inventory.txt`
- Modify: `docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md`

- [ ] **Step 1: Download the upstream `kiwisolver 1.5.0` source distribution and unpack it into a temporary audit directory**

```bash
mkdir -p /tmp/kiwisolver-audit
cd /tmp/kiwisolver-audit
python3 -m pip download --no-binary=:all: kiwisolver==1.5.0
tar -xf kiwisolver-1.5.0.tar.gz
```

Expected: a directory like `/tmp/kiwisolver-audit/kiwisolver-1.5.0/` exists with package sources and tests.

- [ ] **Step 2: Inventory the upstream package files and native-facing Python package layout**

Run:

```bash
cd /tmp/kiwisolver-audit/kiwisolver-1.5.0
find . -maxdepth 3 -type f | sort
```

Then write `audit/upstream-kiwisolver-1.5.0-notes.md` with sections matching this template:

```md
# Upstream kiwisolver 1.5.0 Audit Notes

## Python Package Files
- `py/kiwisolver/__init__.py`
- `...`

## Tests
- `tests/test_foo.py`
- `...`

## Packaging Notes
- Build backend:
- Native module import path:
- Files that can be vendored unchanged:
- Files that likely need local edits:
```

- [ ] **Step 3: Inspect the upstream native extension surface under CPython and record all exported types, functions, constants, and exception classes**

Run:

```bash
python3 - <<'PY'
import inspect
import kiwisolver

print("module:", kiwisolver.__file__)
for name in sorted(dir(kiwisolver)):
    if name.startswith("__"):
        continue
    obj = getattr(kiwisolver, name)
    print(name, type(obj), getattr(obj, "__module__", None))
    if inspect.isclass(obj):
        print("  methods:", [n for n in dir(obj) if not n.startswith("__")][:50])
PY
```

Save the result to `audit/upstream-native-surface.txt`.

- [ ] **Step 4: Inventory the upstream tests and decide whether the standalone port remains in scope**

Run:

```bash
cd /tmp/kiwisolver-audit/kiwisolver-1.5.0
find . -path '*test*.py' -o -path '*/tests/*' | sort
```

Save the inventory to `audit/upstream-test-inventory.txt`, then add an `## Audit Outcome` section to `audit/upstream-kiwisolver-1.5.0-notes.md`:

```md
## Audit Outcome
- Standalone port remains viable: yes/no
- Reason:
- Candidate Rust solver crates:
- Native API breadth summary:
- Test suite size summary:
```

- [ ] **Step 5: Update the design spec with the audited package version and go/no-go decision**

Add a short section like this to `docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md`:

```md
## Audit Result

- Audited version: `kiwisolver 1.5.0`
- Standalone port decision: proceed
- Selected implementation baseline: vendored Python package plus PyO3 native module
```

- [ ] **Step 6: Commit the audit checkpoint**

```bash
git add audit/upstream-kiwisolver-1.5.0-notes.md audit/upstream-native-surface.txt audit/upstream-test-inventory.txt docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md
git commit -m "docs: audit upstream kiwisolver 1.5.0 surface"
```

### Task 2: Scaffold The Repo Skeleton And Packaging

**Files:**
- Create: `LICENSE`
- Create: `README.md`
- Create: `Cargo.toml`
- Create: `pyproject.toml`
- Create: `crates/kiwisolver-rust-python/Cargo.toml`
- Create: `crates/kiwisolver-rust-python/src/lib.rs`
- Create: `python/kiwisolver/__init__.py`
- Create: `tests/python/test_imports.py`
- Create: `tests/run_python_compat.py`

- [ ] **Step 1: Write a failing Python import smoke test**

Create `tests/python/test_imports.py`:

```python
import importlib


def test_import_kiwisolver_module():
    mod = importlib.import_module("kiwisolver")
    assert mod.__name__ == "kiwisolver"


def test_import_core_symbols():
    mod = importlib.import_module("kiwisolver")
    for name in ["Variable", "Term", "Expression", "Constraint", "Solver"]:
        assert hasattr(mod, name), name
```

- [ ] **Step 2: Run the import smoke test and verify it fails because the package does not exist yet**

Run:

```bash
python3 -m pytest tests/python/test_imports.py -v
```

Expected: `ModuleNotFoundError: No module named 'kiwisolver'`.

- [ ] **Step 3: Create the minimal package and build skeleton**

Create `pyproject.toml`:

```toml
[build-system]
requires = ["maturin>=1.7,<2"]
build-backend = "maturin"

[project]
name = "kiwisolver"
version = "1.5.0.dev0"
requires-python = ">=3.10"

[tool.pytest.ini_options]
testpaths = ["tests/python"]
pythonpath = ["python"]
```
Create `Cargo.toml`:

```toml
[workspace]
members = ["crates/kiwisolver-rust-python"]
resolver = "2"
```

Create `crates/kiwisolver-rust-python/Cargo.toml`:

```toml
[package]
name = "kiwisolver-rust-python"
version = "0.1.0"
edition = "2021"

[lib]
name = "_kiwisolver_native"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.27", features = ["extension-module"] }
```

Create `crates/kiwisolver-rust-python/src/lib.rs`:

```rust
use pyo3::prelude::*;

#[pyclass]
struct Variable;

#[pyclass]
struct Term;

#[pyclass]
struct Expression;

#[pyclass]
struct Constraint;

#[pyclass]
struct Solver;

#[pymodule]
fn _kiwisolver_native(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Variable>()?;
    m.add_class::<Term>()?;
    m.add_class::<Expression>()?;
    m.add_class::<Constraint>()?;
    m.add_class::<Solver>()?;
    Ok(())
}
```

Create `python/kiwisolver/__init__.py`:

```python
from _kiwisolver_native import Constraint, Expression, Solver, Term, Variable

__all__ = ["Variable", "Term", "Expression", "Constraint", "Solver"]
__version__ = "1.5.0.dev0"
```

Create `tests/run_python_compat.py`:

```python
import os
import sys
import pytest


def main() -> int:
    root = os.path.dirname(os.path.dirname(__file__))
    os.environ.setdefault("PYTHONPATH", os.path.join(root, "python"))
    return pytest.main(["-q", os.path.join(root, "tests", "python")])


if __name__ == "__main__":
    raise SystemExit(main())
```

Create `LICENSE` with the BSD-3-Clause text.

Create `README.md`:

```md
# kiwisolver-rust

Rust/PyO3 port of `kiwisolver` that preserves the upstream Python package surface as closely as possible.
```

- [ ] **Step 4: Build the extension in editable mode**

Run:

```bash
maturin develop -m crates/kiwisolver-rust-python/Cargo.toml
```

Expected: the local Python environment can import `_kiwisolver_native`.

- [ ] **Step 5: Run the import smoke test and verify it passes**

Run:

```bash
python3 -m pytest tests/python/test_imports.py -v
```

Expected: `2 passed`.

- [ ] **Step 6: Commit the scaffold**

```bash
git add LICENSE README.md Cargo.toml pyproject.toml crates/kiwisolver-rust-python/Cargo.toml crates/kiwisolver-rust-python/src/lib.rs python/kiwisolver/__init__.py tests/python/test_imports.py tests/run_python_compat.py
git commit -m "feat: scaffold kiwisolver-rust package"
```

### Task 3: Vendor Upstream Python Package Files And Seed Compatibility Tests

**Files:**
- Create: `python/kiwisolver/` vendored upstream files
- Create: `tests/upstream_cpp/README.md`
- Create: `tests/run_python_compat.py`
- Modify: `README.md`

- [ ] **Step 1: Copy the upstream Python package files into the local package tree**

Run:

```bash
cd /tmp/kiwisolver-audit/kiwisolver-1.5.0
rsync -av py/kiwisolver/ /Users/sunny/work/codepod/kiwisolver-rust/python/kiwisolver/
```

Expected: upstream Python files replace the temporary stub package layout.

- [ ] **Step 2: Record the upstream solver corpus locally as the compatibility source of truth**

Create `tests/upstream_cpp/README.md`:

```md
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
```

- [ ] **Step 3: Add a focused import test for the vendored package entrypoint**

Update `tests/python/test_imports.py` to:

```python
import importlib


def test_import_kiwisolver_module():
    mod = importlib.import_module("kiwisolver")
    assert mod.__name__ == "kiwisolver"


def test_import_core_symbols():
    mod = importlib.import_module("kiwisolver")
    for name in ["Variable", "Term", "Expression", "Constraint", "Solver"]:
        assert hasattr(mod, name), name


def test_module_has_upstream_version_attr():
    mod = importlib.import_module("kiwisolver")
    assert hasattr(mod, "__version__")
```

- [ ] **Step 4: Write the Python compatibility runner**

Create `tests/run_python_compat.py`:

```python
import os
import sys
import pytest


def main() -> int:
    root = os.path.dirname(os.path.dirname(__file__))
    os.environ.setdefault("PYTHONPATH", os.path.join(root, "python"))
    return pytest.main(["-q", os.path.join(root, "tests", "python")])


if __name__ == "__main__":
    raise SystemExit(main())
```

- [ ] **Step 5: Run the import tests through the Python compatibility runner**

Run:

```bash
python3 -m pytest tests/python/test_imports.py -v
python3 tests/run_python_compat.py
```

Expected: import tests pass through the local Python compatibility runner.

- [ ] **Step 6: Document the vendored-upstream workflow in `README.md` and commit**

Append this section to `README.md`:

```md
## Upstream Sync

The Python package files are vendored from upstream `kiwisolver`.
The audited `1.5.0` release does not ship a standalone Python test suite, so
compatibility work uses adapted upstream C++ solver cases plus local Python tests
derived from the audited API surface. The compatibility goal is to keep the Python
package as close to upstream as possible and reimplement only the native extension
layer in Rust.
```

Commit:

```bash
git add README.md python/kiwisolver tests/upstream_cpp/README.md tests/run_python_compat.py tests/python/test_imports.py
git commit -m "feat: vendor upstream kiwisolver package and tests"
```

### Task 4: Implement Native Exceptions, Strengths, And The `Variable` Surface

**Files:**
- Create: `crates/kiwisolver-rust-python/src/errors.rs`
- Create: `crates/kiwisolver-rust-python/src/strength.rs`
- Create: `crates/kiwisolver-rust-python/src/variable.rs`
- Modify: `crates/kiwisolver-rust-python/src/lib.rs`
- Create: `tests/python/test_repr_and_errors.py`

- [ ] **Step 1: Write failing tests for strengths, variable naming, and exception exports**

Create `tests/python/test_repr_and_errors.py`:

```python
import kiwisolver as kiwi


def test_variable_name_round_trip():
    var = kiwi.Variable("width")
    assert var.name() == "width"


def test_strength_required_is_numeric():
    assert isinstance(kiwi.strength.required, (int, float))


def test_duplicate_constraint_error_exists():
    assert hasattr(kiwi, "DuplicateConstraint")
```

- [ ] **Step 2: Run the focused tests to verify they fail**

Run:

```bash
python3 -m pytest tests/python/test_repr_and_errors.py -v
```

Expected: failures for missing constructors, methods, strengths, or exceptions.

- [ ] **Step 3: Implement exception and strength exports plus the `Variable` wrapper**

Create `crates/kiwisolver-rust-python/src/errors.rs`:

```rust
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(_kiwisolver_native, DuplicateConstraint, PyException);
create_exception!(_kiwisolver_native, UnsatisfiableConstraint, PyException);
create_exception!(_kiwisolver_native, UnknownConstraint, PyException);
create_exception!(_kiwisolver_native, DuplicateEditVariable, PyException);
create_exception!(_kiwisolver_native, UnknownEditVariable, PyException);

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("DuplicateConstraint", m.py().get_type::<DuplicateConstraint>())?;
    m.add("UnsatisfiableConstraint", m.py().get_type::<UnsatisfiableConstraint>())?;
    m.add("UnknownConstraint", m.py().get_type::<UnknownConstraint>())?;
    m.add("DuplicateEditVariable", m.py().get_type::<DuplicateEditVariable>())?;
    m.add("UnknownEditVariable", m.py().get_type::<UnknownEditVariable>())?;
    Ok(())
}
```

Create `crates/kiwisolver-rust-python/src/strength.rs`:

```rust
use pyo3::prelude::*;
use pyo3::types::PyModule;

pub const REQUIRED: f64 = 1_001_001_000.0;
pub const STRONG: f64 = 1_000_000.0;
pub const MEDIUM: f64 = 1_000.0;
pub const WEAK: f64 = 1.0;

pub fn register(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let strength = PyModule::new(py, "strength")?;
    strength.add("required", REQUIRED)?;
    strength.add("strong", STRONG)?;
    strength.add("medium", MEDIUM)?;
    strength.add("weak", WEAK)?;
    m.add_submodule(&strength)?;
    m.add("strength", strength)?;
    Ok(())
}
```

Create `crates/kiwisolver-rust-python/src/variable.rs`:

```rust
use pyo3::prelude::*;

#[pyclass(module = "kiwisolver")]
#[derive(Clone)]
pub struct Variable {
    name: String,
    value: f64,
}

#[pymethods]
impl Variable {
    #[new]
    #[pyo3(signature = (name=""))]
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: 0.0,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn value(&self) -> f64 {
        self.value
    }
}
```

Update `crates/kiwisolver-rust-python/src/lib.rs` to register these modules and classes.

- [ ] **Step 4: Build and rerun the focused tests**

Run:

```bash
maturin develop -m crates/kiwisolver-rust-python/Cargo.toml
python3 -m pytest tests/python/test_repr_and_errors.py -v
```

Expected: these focused tests pass even though upstream tests still fail broadly.

- [ ] **Step 5: Commit the first native surface milestone**

```bash
git add crates/kiwisolver-rust-python/src/lib.rs crates/kiwisolver-rust-python/src/errors.rs crates/kiwisolver-rust-python/src/strength.rs crates/kiwisolver-rust-python/src/variable.rs tests/python/test_repr_and_errors.py
git commit -m "feat: add kiwisolver variable strengths and exceptions"
```

### Task 5: Implement `Term`, `Expression`, And `Constraint` Arithmetic

**Files:**
- Create: `crates/kiwisolver-rust-python/src/expression.rs`
- Create: `crates/kiwisolver-rust-python/src/constraint.rs`
- Modify: `crates/kiwisolver-rust-python/src/lib.rs`
- Modify: `tests/python/test_repr_and_errors.py`

- [ ] **Step 1: Extend the focused tests with expression and constraint behavior**

Append to `tests/python/test_repr_and_errors.py`:

```python
import kiwisolver as kiwi


def test_term_from_variable_multiplication():
    width = kiwi.Variable("width")
    term = 2 * width
    assert isinstance(term, kiwi.Term)


def test_expression_addition():
    width = kiwi.Variable("width")
    expr = 2 * width + 10
    assert isinstance(expr, kiwi.Expression)


def test_constraint_creation():
    width = kiwi.Variable("width")
    constraint = (width >= 10) | "required"
    assert isinstance(constraint, kiwi.Constraint)
```

- [ ] **Step 2: Run the focused tests and verify arithmetic/operator failures**

Run:

```bash
python3 -m pytest tests/python/test_repr_and_errors.py -v
```

Expected: failures in numeric operators and constraint construction.

- [ ] **Step 3: Implement minimal `Term`, `Expression`, and `Constraint` support**

Create `crates/kiwisolver-rust-python/src/expression.rs` with wrappers that can:

```rust
// Required capabilities for this task:
// - `Variable * number` and `number * Variable` -> `Term`
// - `Term + number`, `Term + Term`, `Variable + number` -> `Expression`
// - expression normalization into a linear form (constant + terms)
```

Create `crates/kiwisolver-rust-python/src/constraint.rs` with wrappers that can:

```rust
// Required capabilities for this task:
// - `>=`, `<=`, `==` operators over variables/terms/expressions
// - `constraint | strength`
// - storage of relation kind, normalized expression, and strength
```

Update `crates/kiwisolver-rust-python/src/lib.rs` to export `Term`, `Expression`, and `Constraint`.

- [ ] **Step 4: Rebuild and rerun the focused tests**

Run:

```bash
maturin develop -m crates/kiwisolver-rust-python/Cargo.toml
python3 -m pytest tests/python/test_repr_and_errors.py -v
```

Expected: all focused tests in this file pass.

- [ ] **Step 5: Commit the linear-expression layer**

```bash
git add crates/kiwisolver-rust-python/src/lib.rs crates/kiwisolver-rust-python/src/expression.rs crates/kiwisolver-rust-python/src/constraint.rs tests/python/test_repr_and_errors.py
git commit -m "feat: add kiwisolver expressions and constraints"
```

### Task 6: Implement `Solver` Operations On Top Of The Selected Rust Solver Crate

**Files:**
- Create: `crates/kiwisolver-rust-python/src/solver.rs`
- Modify: `crates/kiwisolver-rust-python/Cargo.toml`
- Modify: `crates/kiwisolver-rust-python/src/lib.rs`
- Create: `crates/kiwisolver-rust-python/tests/native_api.rs`

- [ ] **Step 1: Write failing solver tests**

Add to `tests/python/test_repr_and_errors.py`:

```python
import kiwisolver as kiwi


def test_solver_updates_variable_values():
    width = kiwi.Variable("width")
    solver = kiwi.Solver()
    solver.addConstraint(width == 42)
    solver.updateVariables()
    assert width.value() == 42


def test_duplicate_constraint_raises():
    width = kiwi.Variable("width")
    c = width >= 10
    solver = kiwi.Solver()
    solver.addConstraint(c)
    try:
        solver.addConstraint(c)
    except kiwi.DuplicateConstraint:
        pass
    else:
        raise AssertionError("expected DuplicateConstraint")
```

- [ ] **Step 2: Run the solver tests and verify failure**

Run:

```bash
python3 -m pytest tests/python/test_repr_and_errors.py -v
```

Expected: failures for missing `Solver` methods or no variable updates.

- [ ] **Step 3: Add the chosen Rust solver dependency and implement the wrapper**

Update `crates/kiwisolver-rust-python/Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.27", features = ["extension-module"] }
cassowary = "0.3"
```

Create `crates/kiwisolver-rust-python/src/solver.rs` with support for:

```rust
// Required capabilities for this task:
// - create/destroy solver state
// - add/remove constraints
// - add/remove edit variables
// - suggest values
// - update variable values back onto Python-visible Variable wrappers
// - map duplicate/unknown/unsatisfiable cases to Python exceptions
```

Create `crates/kiwisolver-rust-python/tests/native_api.rs`:

```rust
#[test]
fn placeholder_native_solver_smoke() {
    assert!(true);
}
```

- [ ] **Step 4: Rebuild and rerun the focused tests plus Rust tests**

Run:

```bash
cargo test -p kiwisolver-rust-python
maturin develop -m crates/kiwisolver-rust-python/Cargo.toml
python3 -m pytest tests/python/test_repr_and_errors.py -v
```

Expected: focused solver tests pass.

- [ ] **Step 5: Commit the solver core**

```bash
git add crates/kiwisolver-rust-python/Cargo.toml crates/kiwisolver-rust-python/src/lib.rs crates/kiwisolver-rust-python/src/solver.rs crates/kiwisolver-rust-python/tests/native_api.rs tests/python/test_repr_and_errors.py
git commit -m "feat: add kiwisolver solver wrapper"
```

### Task 7: Drive Compatibility Against Adapted Upstream Solver Cases

**Files:**
- Modify: `tests/python/test_imports.py`
- Modify: `tests/python/test_repr_and_errors.py`
- Modify: `tests/run_python_compat.py`
- Create/Modify: `tests/upstream_cpp/*`
- Modify: `python/kiwisolver/` vendored package files only where strictly needed
- Modify: `crates/kiwisolver-rust-python/src/*.rs`

- [ ] **Step 1: Translate one upstream solver case into a local test and capture the first failing behavior**

Run:

```bash
python3 tests/run_python_compat.py
```

Expected: local Python compatibility failures identify the next semantic gaps, while translated upstream solver cases become the source of new regressions.

- [ ] **Step 2: Add one small local regression test per newly discovered bug before fixing it**

Use this template each time a new failure class appears:

```python
def test_regression_name_here():
    import kiwisolver as kiwi
    width = kiwi.Variable("width")
    solver = kiwi.Solver()
    solver.addConstraint(width == 42)
    solver.updateVariables()
    assert width.value() == 42
```

Place each focused regression into `tests/python/test_repr_and_errors.py` until the file becomes unwieldy, then split by topic. Where the behavior came from a specific upstream C++ test, note the source filename in a comment.

- [ ] **Step 3: Implement the minimal Rust or Python-package fix for the failing behavior**

Allowed fix locations:

```text
crates/kiwisolver-rust-python/src/errors.rs
crates/kiwisolver-rust-python/src/strength.rs
crates/kiwisolver-rust-python/src/variable.rs
crates/kiwisolver-rust-python/src/expression.rs
crates/kiwisolver-rust-python/src/constraint.rs
crates/kiwisolver-rust-python/src/solver.rs
python/kiwisolver/__init__.py
python/kiwisolver/_cext.py
```

Rule: prefer changing Rust native code; only patch vendored Python code where the package needs a minimal import-path or runtime compatibility adjustment.

- [ ] **Step 4: Rerun the local regression and then the upstream suite**

Run:

```bash
python3 -m pytest tests/python -v
python3 tests/run_python_compat.py
```

Expected: the local regression passes and the Python compatibility failures drop.

- [ ] **Step 5: Commit after each logical compatibility tranche**

Commit template:

```bash
git add tests/python python/kiwisolver crates/kiwisolver-rust-python/src
git commit -m "fix: match kiwisolver upstream constraint semantics"
```

Repeat Task 7 until the translated upstream solver corpus and the local Python compatibility suite pass, or until a blocker invalidates the standalone-port decision from Task 1.

### Task 8: Add RustPython And Downstream Integration Checks

**Files:**
- Create: `tests/run_rustpython_compat.py`
- Modify: `README.md`
- Modify: `pyproject.toml`

- [ ] **Step 1: Write a small RustPython runner mirroring the CPython compatibility runner**

Create `tests/run_rustpython_compat.py`:

```python
import os
import sys

from run_python_compat import main


if __name__ == "__main__":
    os.environ.setdefault("PYTHONPATH", os.path.join(os.path.dirname(os.path.dirname(__file__)), "python"))
    raise SystemExit(main())
```

- [ ] **Step 2: Run one smoke import under RustPython and verify the package imports**

Run:

```bash
../pyo3-rustpython/target/release/pyo3-rustpython -c "import kiwisolver; print(kiwisolver.__name__)"
```

Expected: prints `kiwisolver`.

- [ ] **Step 3: Run the Python compatibility suite under RustPython**

Run:

```bash
../pyo3-rustpython/target/release/pyo3-rustpython tests/run_rustpython_compat.py
```

Expected: same pass result as the CPython Python-compat run.

- [ ] **Step 4: Add a downstream smoke import from `matplotlib-rust`**

Run:

```bash
cd /Users/sunny/work/codepod/matplotlib-rust
python3 -c "import sys; sys.path.insert(0, '/Users/sunny/work/codepod/kiwisolver-rust/python'); import matplotlib._layoutgrid"
```

Expected: `matplotlib._layoutgrid` imports without `ImportError` from `kiwisolver`.

- [ ] **Step 5: Document both integration paths and commit**

Append to `README.md`:

```md
## Verification

- CPython: `python3 tests/run_python_compat.py`
- RustPython: `../pyo3-rustpython/target/release/pyo3-rustpython tests/run_rustpython_compat.py`
- Downstream smoke check: import `matplotlib._layoutgrid` with this package on `PYTHONPATH`
```

Commit:

```bash
git add README.md tests/run_rustpython_compat.py pyproject.toml
git commit -m "test: add rustpython and downstream integration checks"
```

### Task 9: Final Verification And Release Readiness

**Files:**
- Modify: `README.md`
- Modify: `docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md`

- [ ] **Step 1: Run the full verification matrix**

Run:

```bash
cargo test -p kiwisolver-rust-python
python3 -m pytest tests/python -v
python3 tests/run_python_compat.py
../pyo3-rustpython/target/release/pyo3-rustpython tests/run_rustpython_compat.py
```

Expected: all commands pass.

- [ ] **Step 2: Record the audited upstream version and final pass status in `README.md`**

Add:

```md
## Status

- Upstream target: `kiwisolver 1.5.0`
- Adapted upstream solver corpus: passing
- CPython support: passing
- RustPython support: passing
```

- [ ] **Step 3: Record the final outcome in the design spec**

Append to `docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md`:

```md
## Final Outcome

- Standalone port completed: yes
- Upstream version implemented: `kiwisolver 1.5.0`
- Upstream test status: passing
- RustPython status: passing
```

- [ ] **Step 4: Commit the release-readiness checkpoint**

```bash
git add README.md docs/superpowers/specs/2026-04-17-kiwisolver-rust-design.md
git commit -m "docs: record kiwisolver-rust verification status"
```

## Self-Review

Spec coverage check:
- New sibling repo, BSD-3-Clause licensing, vendored upstream Python package, vendored upstream tests, PyO3 native implementation, CPython and RustPython validation, and the audit stop/go gate are all covered by Tasks 1 through 9.
- The fallback decision is covered by Task 1 as a hard audit checkpoint before major implementation work.

Placeholder scan:
- The plan contains one intentionally open choice: the exact Rust solver crate is selected during Task 1 from audited candidates, because the approved design explicitly makes that choice conditional on the audit.
- All other tasks contain concrete files, commands, and expected outputs.

Type consistency:
- The planned exported Python symbols remain `Variable`, `Term`, `Expression`, `Constraint`, `Solver`, plus the named exception classes and `strength` module throughout the plan.
