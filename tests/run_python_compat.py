import os
import sys

import pytest


def main() -> int:
    root = os.path.dirname(os.path.dirname(__file__))
    python_dir = os.path.join(root, "python")
    os.environ.setdefault("PYTHONPATH", python_dir)
    sys.path.insert(0, python_dir)
    return pytest.main(["-q", os.path.join(root, "tests", "python")])


if __name__ == "__main__":
    raise SystemExit(main())
