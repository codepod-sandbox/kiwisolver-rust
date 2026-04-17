import importlib
from pathlib import Path


def test_import_kiwisolver_module():
    mod = importlib.import_module("kiwisolver")
    assert mod.__name__ == "kiwisolver"
    assert mod.__package__ == "kiwisolver"
    assert Path(mod.__file__).name == "__init__.py"


def test_import_core_symbols():
    mod = importlib.import_module("kiwisolver")
    native = importlib.import_module("kiwisolver._kiwisolver_native")
    assert native.__name__ == "kiwisolver._kiwisolver_native"
    assert mod.__all__ == ["Variable", "Term", "Expression", "Constraint", "Solver"]
    for name in ["Variable", "Term", "Expression", "Constraint", "Solver"]:
        assert hasattr(mod, name), name
        assert getattr(mod, name).__module__ == "kiwisolver._kiwisolver_native"
