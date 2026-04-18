# kiwisolver-rust CI and Packaging Design

Date: 2026-04-18

## Goal

Add a GitHub Actions pipeline for `kiwisolver-rust` that gives fast quality
signal on every push and pull request, treats macOS as a first-class CPython
target, keeps Windows visible as a supported platform, and creates a clean
expansion path for future RustPython and WASM validation.

## Scope

This design covers:

- repository CI structure in `.github/workflows/ci.yml`
- CPython quality and test jobs
- CPython wheel and source distribution builds
- artifact upload for built distributions
- repository metadata or packaging adjustments required to make wheel builds run

This design does not cover:

- publishing to PyPI
- release tagging or GitHub Releases automation
- RustPython CI execution before `pyo3-rustpython` is stable and reproducible in CI
- WASM packaging or RustPython/WASM distribution artifacts

## Constraints

- The repository already has local verification commands that should remain the
  source of truth:
  - `cargo test -p kiwisolver-rust-python`
  - `python3 tests/run_python_compat.py`
  - `python3 -m pytest tests/python -v`
- The sibling projects use simple GitHub Actions workflows with a small number of
  focused jobs rather than heavily abstracted pipelines.
- The package is primarily being built to support RustPython-related work, but it
  must remain usable from normal CPython as well.
- macOS should be treated as a first-class platform now, not as a later add-on.
- Windows should be included in a way that keeps future support practical even if
  some platform-specific fixes are still needed.

## Approaches Considered

### 1. Linux-only quality and packaging

Pros:

- smallest initial workflow
- cheapest CI runtime

Cons:

- contradicts the goal of making macOS first-class now
- hides Windows breakage until much later
- makes future RustPython/WASM expansion more awkward because the pipeline starts
  too narrowly

### 2. Matrix-first CPython CI

Pros:

- keeps one main workflow file
- gives first-class macOS coverage immediately
- keeps Windows in view through the wheel matrix
- leaves room to add RustPython/WASM jobs beside the CPython path later

Cons:

- slightly more workflow complexity up front
- Windows-specific packaging issues may surface earlier

### 3. Fully split workflows by concern

Pros:

- clear separation between quality checks and packaging
- easy future growth into multiple target-specific workflows

Cons:

- more files and more duplication than the current repo needs
- heavier maintenance burden for an early-stage repository

## Recommended Approach

Use approach 2: a matrix-first CPython workflow in a single `ci.yml` file.

This keeps the repo aligned with the sibling projects' level of complexity while
still treating macOS as first-class and preserving a clean route to future
RustPython and WASM work. The design should be explicit about the separation
between current CPython jobs and future RustPython/WASM jobs so the later work
extends the pipeline rather than reshaping it.

## Workflow Structure

The workflow should trigger on:

- `push` to `main`
- all `pull_request` events targeting the repository

The workflow should define the following jobs.

### `lint`

Runs on `ubuntu-latest`.

Purpose:

- provide fast style and static-quality signal

Commands:

- `cargo fmt --all -- --check`
- `cargo clippy -p kiwisolver-rust-python --no-deps -- -D warnings`

Reasoning:

- formatting and clippy do not need a platform matrix for useful signal
- limiting clippy to the Python extension crate avoids unrelated workspace noise

### `test`

Runs on `ubuntu-latest`.

Purpose:

- validate the current local development contract in CI

Commands:

- `cargo test -p kiwisolver-rust-python`
- `python3 tests/run_python_compat.py`
- `python3 -m pytest tests/python -v`

Setup:

- install stable Rust
- install Python
- install `maturin`

Reasoning:

- this mirrors the commands already used locally
- one Linux test job gives fast feedback before the more expensive wheel matrix
- wheel builds are not a substitute for explicit test execution

### `wheels`

Runs as a matrix over:

- `ubuntu-latest`
- `macos-latest`
- `windows-latest`

Purpose:

- build distributable CPython artifacts for the platforms we intend to support
- make macOS first-class today
- keep Windows support practical now and later

Build outputs:

- platform wheels for CPython
- one source distribution

Implementation:

- use `PyO3/maturin-action`
- use `maturin build --release` for wheel builds
- build the sdist once on Linux
- upload all artifacts using `actions/upload-artifact`

Reasoning:

- `maturin-action` is the most direct path to multi-platform wheel builds and is a
  better base than ad hoc shell setup when macOS and Windows matter
- artifact upload gives immediate value without committing to PyPI
- keeping wheels in CI now reduces the friction of validating downstream CPython
  compatibility later

## Packaging Decisions

### Build backend

Keep `maturin` as the build backend.

### Distribution outputs

Build and retain:

- CPython wheels
- source distribution

Do not build or publish:

- PyPI releases
- RustPython-specific distributions
- WASM artifacts

### Versioning

Continue using the project version already declared in `pyproject.toml` until a
more formal release process exists.

## Future RustPython and WASM Track

The workflow should be structured so a later phase can add separate jobs such as:

- `rustpython-tests`
- `rustpython-build`
- `wasm-build`

Those future jobs should live beside the CPython jobs rather than being folded
into the wheel matrix, because they represent a different runtime target and
different success criteria.

The current workflow should therefore keep the CPython responsibilities grouped
under clear job names and avoid generic matrix logic that would make future
RustPython/WASM integration harder to understand.

## Error Handling and Failure Policy

- `lint` and `test` are required quality gates.
- `wheels` should fail normally on any platform error; Windows should not be
  hidden by default.
- If Windows exposes a real packaging incompatibility, fix the packaging or
  workflow explicitly rather than silently dropping the platform.
- Platform-specific exceptions should only be introduced when a concrete failure
  is understood and documented.

## Required Repository Changes

Expected code or metadata changes alongside the workflow:

- create `.github/workflows/ci.yml`
- add any missing `project` metadata in `pyproject.toml` required for sdist/wheel
  builds if CI reveals gaps
- add workflow comments only where future RustPython/WASM expansion would
  otherwise be unclear

## Testing Strategy

Validation for this work should be:

1. local dry-run of the exact commands used by the `lint` and `test` jobs where
   possible
2. local verification that `maturin build` can produce distributions on the
   current machine
3. GitHub Actions execution to validate Linux, macOS, and Windows wheel builds

## Success Criteria

- pushes and pull requests automatically run CI
- the repository has explicit `lint`, `test`, and `wheels` jobs
- macOS is covered in the wheel pipeline from the first version
- Windows is present in the wheel matrix
- CI produces downloadable CPython wheels and an sdist
- no PyPI publishing is configured
- the workflow structure leaves a clear place for future RustPython/WASM jobs
