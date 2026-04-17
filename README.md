# kiwisolver-rust

Rust/PyO3 port of `kiwisolver` that preserves the upstream Python package surface as closely as possible.

Development setup:

```bash
maturin develop -m crates/kiwisolver-rust-python/Cargo.toml
python -m pytest tests/python/test_imports.py -v
```
