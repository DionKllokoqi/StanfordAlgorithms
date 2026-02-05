pub fn sort_and_count_inversions(array: &[i64]) -> (Vec<i64>, u64) {
    if array.len() <= 1 {
        return (array.to_vec(), 0);
    }

    let mid = array.len() / 2;
    let (left_sorted, inv_left) = sort_and_count_inversions(&array[..mid]);
    let (right_sorted, inv_right) = sort_and_count_inversions(&array[mid..]);
    let (merged, inv_split) = merge_and_count_split_inversions(&left_sorted, &right_sorted);

    (merged, inv_left + inv_right + inv_split)
}

fn merge_and_count_split_inversions(left: &[i64], right: &[i64]) -> (Vec<i64>, u64) {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut count: u64 = 0;
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            count += (left.len() - i) as u64;
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        merged.extend_from_slice(&left[i..]);
    }
    if j < right.len() {
        merged.extend_from_slice(&right[j..]);
    }

    println!("Merged array: {:?} with {} split inversions", merged, count);
    (merged, count)
}
