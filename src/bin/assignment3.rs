fn main() {
    let input_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("quicksort_input.txt");
    let contents = std::fs::read_to_string(&input_path)
        .unwrap_or_else(|err| panic!("Failed to read {}: {err}", input_path.display()));

    let values: Vec<i64> = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim().parse::<i64>().unwrap_or_else(|err| {
                panic!(
                    "Invalid integer in {}: {line} ({err})",
                    input_path.display()
                )
            })
        })
        .collect();

    let mut values_first = values.clone();
    let mut values_median = values.clone();
    let mut values_last = values;

    let comparisons_first = stanford_algorithms::assignment3::quicksort(
        &mut values_first,
        stanford_algorithms::assignment3::PivotMode::First,
    );
    print_edges("first", &values_first);

    let comparisons_median = stanford_algorithms::assignment3::quicksort(
        &mut values_median,
        stanford_algorithms::assignment3::PivotMode::Median,
    );
    print_edges("median", &values_median);

    let comparisons_last = stanford_algorithms::assignment3::quicksort(
        &mut values_last,
        stanford_algorithms::assignment3::PivotMode::Last,
    );
    print_edges("last", &values_last);

    println!("comparisons (first): {comparisons_first}");
    println!("comparisons (median): {comparisons_median}");
    println!("comparisons (last): {comparisons_last}");
}

fn print_edges(label: &str, sorted: &[i64]) {
    let n = sorted.len();
    let head_len = n.min(5);
    let tail_len = n.min(5);
    let head = &sorted[..head_len];
    let tail = &sorted[n - tail_len..];
    println!("sorted ({label}): {head:?}, ..., {tail:?}");
}
