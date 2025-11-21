"""Main module for the Karatsuba integer multiplication algorithm."""

import karatsuba as karatsuba_module

if __name__ == "__main__":
    # Get two integers from the user
    x = int(input("Enter the first integer (x): "))
    y = int(input("Enter the second integer (y): "))

    # Multiply x and y using the Karatsuba algorithm
    result = karatsuba_module.karatsuba(x, y)

    # Print the result
    print(f"The product of {x} and {y} is {result}")
