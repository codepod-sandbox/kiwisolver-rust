# --------------------------------------------------------------------------------------
# Copyright (c) 2013-2026, Nucleic Development Team.
#
# Distributed under the terms of the Modified BSD License.
#
# The full license is in the file LICENSE, distributed with this software.
# --------------------------------------------------------------------------------------
"""Compatibility shim for the renamed native module."""

from __future__ import annotations

from . import _kiwisolver_native as _native

Constraint = _native.Constraint
Expression = _native.Expression
Solver = _native.Solver
Term = _native.Term
Variable = _native.Variable

__version__ = getattr(_native, "__version__", "1.5.0")
__kiwi_version__ = getattr(_native, "__kiwi_version__", "1.5.0")


class _Strength:
    weak = 1.0
    medium = 1000.0
    strong = 1000000.0
    required = 1001001000.0

    @staticmethod
    def create(a, b, c, weight=1.0):
        return ((a * 1000000.0) + (b * 1000.0) + c) * weight


strength = getattr(_native, "strength", _Strength())
