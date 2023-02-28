# Rust Todo List

This is a simple command-line to-do list program written in Rust.

## Installation

1. Install Rust on your system by following the instructions [here](https://www.rust-lang.org/tools/install).

2. Clone the repository:

```git clone https://github.com/byChatGPT/todo-rust-cli```

3. Change into the project directory:

```cd todo-rust-cli```

4. Build the program using Cargo:

```cargo build --release```

The `--release` flag builds an optimized version of the program.

## Usage

The program accepts the following command-line arguments:

- `help`: Show a help message.
- `clear`: Clear all tasks from the to-do list.
- `task`: Add a new task to the to-do list.

To add a new task to the to-do list, run the program with the `task` argument followed by the task description:

```./target/release/todo-rust-cli "Buy milk"```

To clear all tasks from the to-do list, run the program with the `clear` argument:

```./target/release/todo-rust-cli clear```

To show all tasks in the to-do list, run the program without any arguments:

```./target/release/todo-rust-cli```

## License

This program is licensed under the [MIT license](LICENSE).
