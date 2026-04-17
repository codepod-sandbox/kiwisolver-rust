import importlib
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))


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
