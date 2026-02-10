# Stanford Algorithms Solutions

This repository contains my solutions to the assignments from the Stanford Algorithms course by Tim Roughgarden.

## Assignments

- **Assignment 1: Integer Multiplication** - Implementation of Karatsuba algorithm
- **Assignment 2: Count Inversions** - Sorting and counting inversions in an array

## Rust Build and Run

This repository now includes a single Rust crate with one binary per assignment.

Prerequisites:
- Rust toolchain with `cargo`
- Optional: Python and `pre-commit` for git hooks

Build all assignments:
```sh
cargo build
```

Run Assignment 1 (Karatsuba integer multiplication):
```sh
cargo run --bin assignment1
```

Run Assignment 2 (Count inversions):
```sh
cargo run --bin assignment2
```

Run Assignment 3 (QuickSort):
```sh
cargo run --bin assignment3
```

## Pre-commit Hooks (Rust fmt/clippy)

If you want consistent formatting and linting before commits, install `pre-commit`
and enable the provided hooks.

Install:
```sh
pip install pre-commit
```

Enable hooks:
```sh
pre-commit install
```

Run on all files:
```sh
pre-commit run --all-files
```

## Course Information

- Instructor: Tim Roughgarden
- Institution: Stanford University
- Course: Algorithms
