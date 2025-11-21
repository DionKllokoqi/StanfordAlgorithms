"""Karatsuba recursive integer multiplication algorithm."""


def karatsuba(x, y):
    """Multiply two integers using the Karatsuba algorithm.

    Args:
        x (int): The first integer to multiply.
        y (int): The second integer to multiply.

    Returns:
        int: The product of x and y.
    """

    # Base case for recursion
    if x < 10 or y < 10:
        return x * y

    n = max(len(str(x)), len(str(y)))

    # Split x and y into two halves
    half_n = n // 2

    a, b = divmod(x, 10 ** half_n)
    c, d = divmod(y, 10 ** half_n)

    # Recursively compute ac
    a_c = karatsuba(a, c)

    # Recursively compute bd
    b_d = karatsuba(b, d)

    # Subtract to get ad + bc
    ad_plus_bc = karatsuba(a + b, c + d) - a_c - b_d

    return ((10 ** (half_n * 2)) * a_c) + ((10 ** half_n) * ad_plus_bc) + b_d
