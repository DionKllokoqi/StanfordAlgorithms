
#[derive(Copy, Clone)]
pub enum PivotMode {
    First,
    Median,
    Last
}

/// Returns the sorted array and numbers of comparisons done. Accepts the input array, and the mode
/// to use for comparisons, i.e., `first pivot`, `last pivot`, and `median pivot`.
pub fn quicksort(input: &mut [i64], pivot: PivotMode) -> (Vec<i64>, u64) {
    // Termination condition, if n == 1, return
    if input.len() <= 1 {
        return (input.to_vec(), 0);
    }

    // Choose Pivot based on pivot mode
    let pivot_index = choose_pivot(input, pivot);

    // Partition array around pivot and add number of comparisons
    let (partitioned, comparisons, p) = partition(input, pivot_index);
    let pivot_val = partitioned[p];

    let (left, rest) = partitioned.split_at_mut(p);
    let (_pivot, right) = rest.split_first_mut().unwrap();

    // Recursively sort 1st part of the array
    let (left, left_comparisons) = quicksort(left, pivot);

    // Recursively sort 2nd part of the array
    let (right, right_comparisons) = quicksort(right, pivot);

    // Return sorted array and No. of comparisons
    let mut out = Vec::with_capacity(input.len());
    out.extend_from_slice(&left);
    out.push(pivot_val);
    out.extend_from_slice(&right);
    (out, comparisons + left_comparisons + right_comparisons)

}

pub fn choose_pivot(input: &mut [i64], pivot: PivotMode) -> usize {
    match pivot {
        PivotMode::First => 0,
        PivotMode::Median => {
            if input.len() % 2 != 0 {
                return input.len() / 2 + 1
            }

            input.len() / 2
        },
        PivotMode::Last => { input.len() - 1 }
    }
}

pub fn partition(input: &mut [i64], pivot_index: usize) -> (&mut [i64], u64, usize) {
    let temp = input[pivot_index];
    input[pivot_index] = input[0];
    input[0] = temp;

    let pivot = input[0];
    let mut i = 1;

    for j in i..input.len() {
        if input[j] < pivot {
            let temp = input[j];
            input[j] = input[i];
            input[i] = temp;
            i += 1;
        }
    }

    let temp = input[0];
    input[0] = input[i - 1];
    input[i - 1] = temp;

    let n = input.len();
    (input, (n - 1) as u64, i - 1)
}
