use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use stanford_algorithms::assignment2::sort_and_count_inversions;

fn load_integer_array() -> io::Result<Vec<i64>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("IntegerArray.txt");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut values = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        values.push(trimmed.parse::<i64>().expect("invalid integer"));
    }

    Ok(values)
}

fn main() -> io::Result<()> {
    let array = load_integer_array()?;
    let (_merged, inversion_count) = sort_and_count_inversions(&array);
    println!("The number of split inversions is: {inversion_count}");
    Ok(())
}
