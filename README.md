# kiwisolver-rust

`kiwisolver-rust` is a Rust/PyO3 port of
[kiwisolver](https://github.com/nucleic/kiwi).

The goal is to preserve the upstream Python package surface while replacing the
native solver layer with Rust so the package can participate in the PyO3 /
RustPython work in this workspace and remain usable from normal CPython as
well.

In practice, this repo keeps the upstream Python package files as close to
upstream as possible and reimplements the compiled extension in Rust.

## License

This repository is distributed under the BSD 3-Clause License. See
[LICENSE](/Users/sunny/work/codepod/kiwisolver-rust/LICENSE).

## Upstream Sync

The Python package files are vendored from upstream `kiwisolver`.
The audited `1.5.0` release does not ship a standalone Python test suite, so
compatibility work uses adapted upstream C++ solver cases plus local Python tests
derived from the audited API surface.

## Status

- Upstream target: `kiwisolver 1.5.0`
- Adapted upstream solver corpus: passing
- CPython support: passing
- Rust unit tests: passing
- RustPython runner: implemented in `tests/run_rustpython_compat.py`
- RustPython validation: blocked in the current environment because `/Users/sunny/work/codepod/pyo3-rustpython/target/release/pyo3-rustpython` is not built locally
- Downstream `matplotlib-rust` smoke check: blocked in the current environment by missing `pyparsing`

Feature-complete status:

- The audited upstream `kiwisolver 1.5.0` C++ solver corpus has been translated
  into local Python compatibility tests and is passing.
- The Rust native module implements the public solver surface needed by that
  corpus, including dump support.
- Remaining work is environment and release hygiene, not missing kiwisolver
  functionality in this repo.

## Verification

- CPython compatibility: `python3 tests/run_python_compat.py`
- Python test suite: `python3 -m pytest tests/python -v`
- Rust tests: `cargo test -p kiwisolver-rust-python`
- RustPython runner entrypoint: `python tests/run_rustpython_compat.py`
- Planned RustPython command: `../pyo3-rustpython/target/release/pyo3-rustpython tests/run_rustpython_compat.py`
- Planned downstream smoke check:
  `python3 -c "import sys; sys.path.insert(0, '/Users/sunny/work/codepod/kiwisolver-rust/python'); sys.path.insert(0, '/Users/sunny/work/codepod/matplotlib-rust/python'); import matplotlib._layoutgrid"`
