# --------------------------------------------------------------------------------------
# Copyright (c) 2013-2026, Nucleic Development Team.
#
# Distributed under the terms of the Modified BSD License.
#
# The full license is in the file LICENSE, distributed with this software.
# --------------------------------------------------------------------------------------
"""Temporary compatibility shim for the renamed native module.

This keeps the vendored Python package importable while the Rust native module
still exposes only the minimal scaffolded surface.
"""

from __future__ import annotations

from . import _kiwisolver_native as _native

__kiwisolver_native_shim__ = True

Constraint = _native.Constraint
Expression = _native.Expression
Solver = _native.Solver
Term = _native.Term
Variable = _native.Variable

__version__ = "1.5.0+shim"
__kiwi_version__ = "1.5.0+shim"


class _Strength:
    """Explicit placeholder for upstream strength exports.

    The upstream API publishes a singleton `strength` object. The scaffolded
    Rust module does not expose it yet, so this local shim keeps the package
    importable without pretending the native implementation is complete.
    """

    weak = 1.0
    medium = 1000.0
    strong = 1000000.0
    required = 1001001000.0

    @staticmethod
    def create(a, b, c, weight=1.0):
        return ((a * 1000000.0) + (b * 1000.0) + c) * weight


strength = getattr(_native, "strength", _Strength())
