"""Module for counting split inversions during merge sort"""


def sort_and_count_inversions(array, length):
    """Count split inversions between two sorted arrays."""

    # Base case
    if length <= 1:
        return array, 0

    mid = length // 2

    # Get left inversions
    left_sort, inv_left = sort_and_count_inversions(array[:mid], mid)

    # Get right inversions
    right_sort, inv_right = sort_and_count_inversions(array[mid:], length - mid)

    # Get split inversions
    merged, inv_split = merge_and_count_split_inversions(
        left_sort + right_sort, mid, length
    )

    return merged, inv_left + inv_right + inv_split


def merge_and_count_split_inversions(array, mid, length):
    """Count split inversions between two sorted arrays."""
    left = array[:mid]
    right = array[mid:]

    count = 0
    merged = [None] * length
    left_index = 0
    right_index = 0

    for k in range(length):
        if left[left_index] < right[right_index]:
            merged[k] = left[left_index]
            left_index += 1
        else:
            count += mid - left_index
            merged[k] = right[right_index]
            right_index += 1

        if left_index == mid:
            merged[k + 1 :] = right[right_index:]
            break
        if right_index == length - mid:
            merged[k + 1 :] = left[left_index:]
            break

    print(f"Merged array: {merged} with {count} split inversions")
    return merged, count
