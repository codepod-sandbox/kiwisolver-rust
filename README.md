# kiwisolver-rust

Rust/PyO3 port of `kiwisolver` that preserves the upstream Python package surface as closely as possible.

Development setup:

```bash
maturin develop -m crates/kiwisolver-rust-python/Cargo.toml
python -m pytest tests/python/test_imports.py -v
```

## Upstream Sync

The Python package files are vendored from upstream `kiwisolver`.
The audited `1.5.0` release does not ship a standalone Python test suite, so
compatibility work uses adapted upstream C++ solver cases plus local Python tests
derived from the audited API surface. The compatibility goal is to keep the Python
package as close to upstream as possible and reimplement only the native extension
layer in Rust.
