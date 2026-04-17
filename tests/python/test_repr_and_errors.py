import importlib
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "python"))

import kiwisolver as kiwi


native = importlib.import_module("kiwisolver._kiwisolver_native")


def test_variable_name_round_trip():
    var = kiwi.Variable("width")
    assert var.name() == "width"


def test_strength_required_is_numeric():
    assert isinstance(kiwi.strength.required, (int, float))
    assert isinstance(native.strength.required, (int, float))


def test_duplicate_constraint_error_exists():
    assert hasattr(kiwi, "DuplicateConstraint")
    assert hasattr(native, "DuplicateConstraint")
