"""
mid.py

Example module inside sub1.
"""


def square(x: int) -> int:
    """
    Return the square of a number.

    Args:
        x (int): Number to square.

    Returns:
        int: Square of x.

    Example:
        >>> square(4)
        16
    """
    return x * x


class Squarer:
    """
    Class to square numbers.

    Methods:
        square_number(x): Return square of x.
    """

    def square_number(self, x: int) -> int:
        """
        Square the given number.

        this is a member of the [[Squarer]] class, and is not the same
        as the [[square]] function

        Args:
            x (int): Number to square.

        Returns:
            int: Square of x.
        """
        return x * x
