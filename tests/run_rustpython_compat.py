import os

from run_python_compat import main


if __name__ == "__main__":
    os.environ.setdefault(
        "PYTHONPATH",
        os.path.join(os.path.dirname(os.path.dirname(__file__)), "python"),
    )
    raise SystemExit(main())
