# kiwisolver-rust

Scaffolded Rust/PyO3 port of `kiwisolver` that preserves the upstream Python package surface as closely as possible while the native solver work is still in progress.

Development setup:

```bash
python3 -m pytest tests/python/test_imports.py -v
python3 tests/run_python_compat.py
```

## Upstream Sync

The Python package files are vendored from upstream `kiwisolver`.
The audited `1.5.0` release does not ship a standalone Python test suite, so
compatibility work uses adapted upstream C++ solver cases plus local Python tests
derived from the audited API surface. The current state is a scaffolded package
with a temporary `_cext` shim over the renamed native module, not a claim that the
full upstream solver runtime has already been reimplemented in Rust.
