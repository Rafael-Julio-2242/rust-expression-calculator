# Rust Expression Calculator

This is a simple yet powerful command-line calculator written in Rust. It can evaluate complex mathematical expressions, respecting operator precedence and parentheses.

## Features

-   Evaluates mathematical expressions with `+`, `-`, `*`, `/` operators.
-   Correctly handles operator precedence (e.g., `*` and `/` are evaluated before `+` and `-`).
-   Supports parentheses to override precedence.
-   Handles complex nested expressions.
-   Interactive command-line interface.
-   Input "q" to quit the application.

## How to Use

1.  Run the calculator from your terminal.
2.  Enter a mathematical expression and press `Enter`.
3.  The calculator will display the result.
4.  To exit, type `q` and press `Enter`.

### Example

```
----------- CALCULATOR -----------
q - to quit
Expression: (1+1)*(2+2)*8
Result: 64
Expression: q
shutting down...
```

## How it Works

The calculator is implemented using two main algorithms:

1.  **Shunting-yard algorithm:** This algorithm is used to parse the infix mathematical expressions (the way humans write them) and convert them into a postfix notation (Reverse Polish Notation - RPN).
2.  **Expression Tree:** The postfix expression is then used to build an expression tree. Each node in the tree is either an operator or a number.
3.  **Evaluation:** Finally, the expression tree is evaluated recursively to compute the final result.

## How to Build and Run

### Prerequisites

-   [Rust programming language](https://www.rust-lang.org/tools/install) installed.

### Build

To build the project, navigate to the project directory and run:

```bash
cargo build
```

### Run

To run the calculator, you can use:

```bash
cargo run
```

This will start the interactive calculator in your terminal.
