"""
one.py

Module with simple utility functions.

See also:
    [[test_pkg.sub1.sub2.two]]
"""


def is_even(num: int) -> bool:
    """
    Check if a number is even.

    Args:
        num (int): Number to check.

    Returns:
        bool: True if even, else False.

    Example:
        >>> is_even(4)
        True

    See Also:
        [[test_pkg.sub1.sub2.one.is_odd]]
    """
    return num % 2 == 0


def is_odd(num: int) -> bool:
    """
    Check if a number is odd. It is a descendant function from the
    [[test_pkg.sub1.sub2]] module

    Args:
        num (int): Number to check.

    Returns:
        bool: True if odd, else False.

    See Also:
        [[test_pkg.sub1.sub2.one.is_even]]
    """
    return num % 2 != 0
