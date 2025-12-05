"""
one.py

Module with simple utility functions.
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
    See also:
        [[test_pkg.sub1.mid.Squarer.square]]
        [[numpy.routines.fft]]
    """
    return num % 2 == 0


def is_odd(num: int) -> bool:
    """
    Check if a number is odd.

    Args:
        num (int): Number to check.

    Returns:
        bool: True if odd, else False.
    """
    return num % 2 != 0
