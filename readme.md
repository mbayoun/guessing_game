# Guess the Number Game

This is a simple command-line number guessing game written in Rust. The program generates a random secret number between 1 and 100, and the player tries to guess it. The game provides feedback on whether the guess is too small, too big, or correct.

## Features

- Randomly generates a secret number between 1 and 100.
- Prompts the player to input their guess.
- Provides feedback if the guess is too small, too big, or correct.
- Colored output for better readability.

## Prerequisites

- Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/learn/get-started).

## Usage

1. Clone the repository or copy the code into a new Rust project.
2. Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
colored = "2.0"
rand = "0.8"
```

3. Build and run the project:

```sh
cargo build
cargo run
```