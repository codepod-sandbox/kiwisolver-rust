# kiwisolver-rust Design

Date: 2026-04-17

## Goal

Create a new sibling repository, `kiwisolver-rust`, that ports upstream `kiwisolver`
to Rust using PyO3 while preserving as much of the upstream Python package and test
suite as possible. The port should target the latest upstream release at the time of
implementation, currently `kiwisolver 1.5.0` released on March 9, 2026.

## Audit Result

- Audited version: `kiwisolver 1.5.0`
- Standalone port decision: proceed
- Selected implementation baseline: vendored Python package plus PyO3 native module
- Audit provenance: reconstructed from the upstream `nucleic/kiwi` release snapshot at commit `5e76d91fd77dc443cb0db36e7398fc13844a0524` plus local source inspection; the exact PyPI sdist was not release-verified because sandboxed download failed.
- Compatibility corpus decision: the inspected release does not contain a standalone Python test suite. Task 2 should port/adapt the upstream C++ solver tests into the new harness and add Python compatibility smoke tests derived from the audited API surface.

The intended runtime model matches the other package ports in this workspace:

- Python package files remain primarily upstream-authored code.
- Native behavior moves into a Rust implementation exposed through PyO3.
- The resulting package should run on both CPython and RustPython through the PyO3
  backend model already used in this project.

## Non-Goals

- A WASM-specific backend in v1.
- A matplotlib-only subset implemented inside `matplotlib-rust`, unless the initial
  audit shows that full `kiwisolver` compatibility is unexpectedly large.
- A pure-Python fallback implementation intended to replace the native module.

## Acceptance Criteria

- A new sibling repo exists at `/Users/sunny/work/codepod/kiwisolver-rust`.
- The repo is BSD-3-Clause licensed and preserves upstream copyright and license
  notices for vendored files.
- Upstream Python package files are vendored with only the minimum edits needed to
  connect them to the Rust native module and to run in this workspace.
- The public import surface used by upstream tests is preserved.
- The audited upstream solver corpus is ported/adapted into the new harness and passes against the port.
- Python compatibility tests derived from the audited API surface exist and pass.
- The port passes the full upstream test suite for the targeted version, or else the
  project stops after the audit phase and falls back to implementing the required
  functionality directly in `matplotlib-rust`.

## Recommended Approach

Implement a thin Rust native module backed by an existing Rust constraint-solver
crate if, and only if, its semantics match `kiwisolver` closely enough. The Python
surface should remain as close to upstream as possible by vendoring upstream Python
files and tests.

This is the preferred approach because it:

- Matches the established pattern used by the other ports in this workspace.
- Minimizes Python API drift by reusing upstream package code directly.
- Gives a strong regression signal through the upstream tests.
- Keeps the Rust work focused on the native extension boundary rather than on
  re-creating package-level Python behavior.

## Architecture

The new repository should be structured in three layers:

1. Vendored upstream Python package

- A `python/kiwisolver/` tree containing the upstream Python files from the targeted
  release.
- Minimal edits only where packaging or native-module hookup requires them.
- Upstream license headers and notices preserved.

2. Rust native extension

- A PyO3 crate exporting the same Python-visible classes and functions expected by
  upstream `kiwisolver`.
- The native layer is responsible for solver state, variables, constraints,
  expressions, edit variables, strengths, and exception behavior.
- The implementation should prefer delegating core solving logic to an existing Rust
  Cassowary-compatible crate if its behavior is close enough under test.

3. Compatibility test harness

- Ported/adapted upstream C++ solver tests in a dedicated host test directory.
- Python smoke and compatibility tests derived from the audited `kiwisolver` API surface, since the release snapshot does not provide a standalone Python corpus.
- A small runner that can execute the adapted solver tests against the local build on CPython; RustPython support remains a later compatibility target once the API surface is mirrored in the Rust extension.
- Local xfail or skip lists should only be introduced for runtime-specific gaps that
  are proven not to be `kiwisolver` correctness bugs. The default target is zero
  compatibility failures.

## Public API Expectations

The implementation should preserve the upstream Python-visible API for the targeted
release, including:

- `Variable`
- `Term`
- `Expression`
- `Constraint`
- `Solver`
- Strength helpers and related module-level helpers/constants
- Native exceptions and error cases used by upstream tests

The exact inventory must be validated during the initial audit by inspecting the
upstream package and extension surface from version `1.5.0`.

## Early Audit Gate

Before heavy implementation begins, perform a short audit of upstream `kiwisolver`
`1.5.0` to answer these questions:

- What Python files are present and which can be copied unchanged?
- What compiled symbols, classes, methods, properties, and exception types are
  exposed by the native module?
- How large is the upstream test suite, and does it exercise mostly API semantics or
  deep solver internals?
- Does an existing Rust solver crate appear capable of matching the observed
  semantics for strengths, edit variables, constraint priorities, duplicate handling,
  and floating-point behavior?

Decision rule:

- If the audit indicates a compact native surface with realistic semantic mapping,
  proceed with the standalone `kiwisolver-rust` port.
- If the audit indicates a meaningfully larger or more idiosyncratic native surface
  than expected, stop and instead implement the required subset directly inside
  `matplotlib-rust`.

## Packaging and Repository Conventions

The new repo should mirror the conventions of the existing sibling ports where
practical:

- BSD-3-Clause root license.
- Clear README describing the split between upstream Python code and Rust native
  implementation.
- A Cargo workspace or small crate layout consistent with other PyO3-backed package
  ports in this workspace.
- Python package location and test layout chosen to make CPython and RustPython test
  execution straightforward.

The package should expose a normal `kiwisolver` import path so downstream consumers,
including `matplotlib-rust`, can use it without import shims.

## Error Handling

The Rust native layer must preserve the observable behavior expected by upstream
tests:

- Reject invalid operations with the correct Python exception type.
- Preserve failure cases for duplicate constraints, unknown edit variables, bad
  strengths, and invalid expressions if upstream tests assert them.
- Avoid silently accepting behavior that upstream treats as errors.

Where an exact exception message is asserted by tests, the message should match as
closely as practical.

## Testing Strategy

Testing should proceed in layers:

1. Rust unit tests for small native invariants and solver wrapper behavior.
2. Python integration tests for package import shape and basic object behavior.
3. Vendored upstream tests as the authoritative compatibility gate.
4. A downstream smoke check from `matplotlib-rust` once the package is consumable.

The upstream test suite is the primary success criterion. Local tests exist to make
debugging faster, not to replace upstream coverage.

## Risks

- A Rust Cassowary crate may differ from `kiwisolver` in subtle but test-visible
  semantics.
- PyO3 behavior may differ between CPython and RustPython in object identity,
  descriptor behavior, hashing, or exception formatting.
- Upstream tests may assume packaging details or native repr/str output that need
  careful matching.
- The latest upstream version may drift again during implementation. This is
  acceptable; the implementation should pin to the audited version rather than chase
  moving head.

## Mitigations

- Treat the audit as a hard gate before major implementation work.
- Prefer incremental bring-up with the upstream tests running as early as possible.
- Use local shim code only where vendored upstream code cannot run unchanged.
- Keep the Rust API organized around upstream-visible semantics, not around the
  internals of the chosen Rust solver crate, so the backend can be swapped if
  needed.

## Implementation Boundary

This spec covers the design and go/no-go criteria for a standalone `kiwisolver-rust`
port. It does not yet choose the exact Rust solver crate, define every file, or
sequence the work into implementation tasks. That belongs in the implementation
plan after the audit-backed design is approved.

## Final Outcome

- Standalone port completed: yes
- Upstream version implemented: `kiwisolver 1.5.0`
- Adapted upstream solver corpus: passing
- CPython status: passing
- Rust unit-test status: passing
- RustPython runner status: implemented
- RustPython validation status: blocked in the current environment because the expected `pyo3-rustpython` executable is not built locally
- Downstream `matplotlib-rust` smoke status: blocked in the current environment by missing `pyparsing`
