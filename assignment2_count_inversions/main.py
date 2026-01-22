"""Main module for the count splint inversion algorithm"""

import os

from sort_count_split_inversions import sort_and_count_inversions

if __name__ == "__main__":
    # Read the integer array file into an array
    file_path = os.path.join(os.path.dirname(__file__), "IntegerArray.txt")
    with open(file_path, "r", encoding="utf-8") as f:
        array = [int(line.strip()) for line in f.readlines()]

    # Call count split inversions algorithm on array
    merged, inversion_count = sort_and_count_inversions(array, len(array))

    # Print the result
    print(f"The number of split inversions is: {inversion_count}")
