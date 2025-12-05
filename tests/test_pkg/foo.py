"""
foo.py

Example module demonstrating a calculator.
"""

__all__ = ["add", "multiply"]


def add(a: float, b: float) -> float:
    """
    Return the sum of two numbers.

    Args:
        a (float): First number.
        b (float): Second number.

    Returns:
        float: Sum of a and b.

    Example:
        >>> add(2.5, 4.5)
        7.0

    See also:
        [[test_pkg.bar.greet|the greeting function]]
    """
    return a + b


def multiply(a: float, b: float) -> float:
    """
    Return the product of two numbers.

    Args:
        a (float): First number.
        b (float): Second number.

    Returns:
        float: Product of a and b.

    See Also:
        [[add]]
        [[_subtract]]

    """
    return a * b


def _subtract(a: float, b: float) -> float:
    """
    Subtract b from a (private helper).

    Args:
        a (float): Minuend.
        b (float): Subtrahend.

    Returns:
        float: Difference.
    """
    return a - b
