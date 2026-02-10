//! Assignment 3 utilities for running QuickSort with different pivot rules.
//!
//! This module exposes:
//! - [`quicksort`]: sorts a slice in place and returns the assignment
//!   comparison count.
//! - [`PivotMode`]: pivot-selection strategy (`First`, `Median`, or `Last`).
//! - [`choose_pivot`] and [`partition`]: helper functions used by `quicksort`.
//!   They are public so they can be exercised directly by tests or experiments.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PivotMode {
    /// Always select the first element in the current subarray.
    First,
    /// Select the median-of-three among first, middle, and last elements.
    Median,
    /// Always select the last element in the current subarray.
    Last,
}

/// Sorts `input` in place using QuickSort and returns the comparison count.
///
/// The `comparison_count` follows the assignment convention:
/// each recursive call on a subarray of length `n` contributes `n - 1`.
///
/// `pivot` controls which pivot-selection strategy is applied at each
/// recursive step.
pub fn quicksort(input: &mut [i64], pivot: PivotMode) -> u64 {
    if input.len() <= 1 {
        return 0;
    }

    let pivot_index = choose_pivot(input, pivot);
    let pivot_final_index = partition(input, pivot_index);
    let mut comparisons = (input.len() - 1) as u64;

    let (left, pivot_and_right) = input.split_at_mut(pivot_final_index);
    let (_, right) = pivot_and_right
        .split_first_mut()
        .expect("pivot element must exist after partition");

    // Recurse on smaller side first to reduce worst-case stack depth.
    if left.len() < right.len() {
        comparisons += quicksort(left, pivot);
        comparisons += quicksort(right, pivot);
    } else {
        comparisons += quicksort(right, pivot);
        comparisons += quicksort(left, pivot);
    }

    comparisons
}

/// Returns the pivot index for `input` according to `pivot`.
///
/// For [`PivotMode::Median`], this uses the median-of-three rule over
/// `input[0]`, the middle element (left-middle for even lengths), and
/// `input[input.len() - 1]`.
pub fn choose_pivot(input: &[i64], pivot: PivotMode) -> usize {
    debug_assert!(!input.is_empty());

    match pivot {
        PivotMode::First => 0,
        PivotMode::Median => median_of_three_index(input),
        PivotMode::Last => input.len() - 1,
    }
}

/// Partitions `input` around the element at `pivot_index`.
///
/// Returns the final index of the pivot after partitioning.
pub fn partition(input: &mut [i64], pivot_index: usize) -> usize {
    debug_assert!(!input.is_empty());
    debug_assert!(pivot_index < input.len());

    input.swap(0, pivot_index);
    let pivot_value = input[0];
    let mut store_index = 1;

    for scan_index in 1..input.len() {
        if input[scan_index] < pivot_value {
            input.swap(scan_index, store_index);
            store_index += 1;
        }
    }

    let pivot_final_index = store_index - 1;
    input.swap(0, pivot_final_index);
    pivot_final_index
}

fn median_of_three_index(input: &[i64]) -> usize {
    let first_idx = 0;
    let last_idx = input.len() - 1;
    let mid_idx = (input.len() - 1) / 2;

    let first = input[first_idx];
    let mid = input[mid_idx];
    let last = input[last_idx];

    if (first <= mid && mid <= last) || (last <= mid && mid <= first) {
        mid_idx
    } else if (mid <= first && first <= last) || (last <= first && first <= mid) {
        first_idx
    } else {
        last_idx
    }
}
