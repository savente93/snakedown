"""
foo.py

Example module demonstrating a calculator.

See Also:
    [[test_pkg.bar]]
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
