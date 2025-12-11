# test_pkg._private.internals

internals.py

Internal helper functions and classes.

## test_pkg._private.internals.calculate_secret_value

calculate_secret_value(x: int, y: int) -> int

Calculate a secret value by multiplying inputs and adding a constant.

Args:
    x (int): First number.
    y (int): Second number.

Returns:
    int: Secret value.

Example:
    >>> calculate_secret_value(2, 3)
    13

## test_pkg._private.internals._double_value

_double_value(value: int) -> int

Double the input value (private helper).

Args:
    value (int): Value to double.

Returns:
    int: Doubled value.

## test_pkg._private.internals.InternalHelper

Helper class for internal computations.

Attributes:
    factor (int): Multiplier factor.

### test_pkg._private.internals.InternalHelper.__init__

__init__(self, factor: int = DEFAULT_FACTOR)

Initialize with a multiplication factor.

Args:
    factor (int): Factor to multiply values by.

### test_pkg._private.internals.InternalHelper.amplify

amplify(self, value: int) -> int

Multiply value by factor.

Args:
    value (int): Value to amplify.

Returns:
    int: Amplified value.

### test_pkg._private.internals.InternalHelper.reset_factor

reset_factor(self)

Reset factor to default value.
