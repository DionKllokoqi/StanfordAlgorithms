use std::io::{self, Write};
use std::str::FromStr;

use num_bigint::BigInt;

use stanford_algorithms::assignment1::karatsuba;

fn read_bigint(prompt: &str) -> BigInt {
    print!("{prompt}");
    io::stdout().flush().expect("stdout flush failed");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    BigInt::from_str(input.trim()).expect("invalid integer")
}

fn main() {
    let x = read_bigint("Enter the first integer (x): ");
    let y = read_bigint("Enter the second integer (y): ");

    let result = karatsuba(&x, &y);
    println!("The product of {x} and {y} is {result}");
}
