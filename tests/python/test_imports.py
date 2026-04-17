import importlib


def test_import_kiwisolver_module():
    mod = importlib.import_module("kiwisolver")
    assert mod.__name__ == "kiwisolver"


def test_import_core_symbols():
    mod = importlib.import_module("kiwisolver")
    for name in ["Variable", "Term", "Expression", "Constraint", "Solver"]:
        assert hasattr(mod, name), name
