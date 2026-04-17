import os
import sys

import pytest


def main() -> int:
    root = os.path.dirname(os.path.dirname(__file__))
    sys.path.insert(0, os.path.join(root, "python"))
    return pytest.main(["-q", os.path.join(root, "tests", "python")])


if __name__ == "__main__":
    raise SystemExit(main())
