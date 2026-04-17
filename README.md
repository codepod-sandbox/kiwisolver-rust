# kiwisolver-rust

Rust/PyO3 port of `kiwisolver` that preserves the upstream Python package
surface while reimplementing the native solver layer in Rust.

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

## Verification

- CPython compatibility: `python3 tests/run_python_compat.py`
- Python test suite: `python3 -m pytest tests/python -v`
- Rust tests: `cargo test -p kiwisolver-rust-python`
- RustPython runner entrypoint: `python tests/run_rustpython_compat.py`
- Planned RustPython command: `../pyo3-rustpython/target/release/pyo3-rustpython tests/run_rustpython_compat.py`
- Planned downstream smoke check:
  `python3 -c "import sys; sys.path.insert(0, '/Users/sunny/work/codepod/kiwisolver-rust/python'); sys.path.insert(0, '/Users/sunny/work/codepod/matplotlib-rust/python'); import matplotlib._layoutgrid"`
