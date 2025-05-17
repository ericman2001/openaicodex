# OpenAI Codex Sandbox

This repository contains a small Rust program used to experiment with the OpenAI Codex capabilities. The program is a simple command line utility that computes the arithmetic mean of numbers contained in a text file.

## Using the mean program

The Rust crate is located in the `mean/` directory. To compile the program, run:

```bash
cargo build --release
```

Once built, the resulting binary will read numbers from a file you specify and print their average. Lines that are empty or contain whitespace are ignored. For example:

```bash
./target/release/mean numbers.txt
```

If the input file does not exist or contains no numbers, the program prints an error message.

## Running tests

This repository includes unit tests for the mean library. To execute them, run

```bash
cargo test
```

from the repository root or from within the `mean/` directory. The tests will
build the project and verify the mean calculations work correctly.

## About this repository

Aside from the mean utility, this repository serves as a sandbox for exploring how OpenAI Codex interacts with a real codebase. Feel free to experiment and modify the code while learning what Codex can do.
