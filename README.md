# smli

A mini-LISP interpreter written in Rust. This is a term project for the course "Compiler" in National Central University. Also, it is my first Rust project. I'm still learning Rust, so the code may not be idiomatic. Currently, it only support a subset of LISP, and it also contains some bugs. I will fix them in the future.

## Features

- [x] Syntax Validation
- [x] Print
- [x] Numerical Operations
- [x] Logical Operations
- [x] if Expression
- [x] Variable Definition
- [x] Function
- [x] Named Function
- [ ] Recursion
- [x] Type Checking
- [ ] Nested Function
- [ ] First-class Function

## Installation

Download Rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and install it.

```sh
git clone https://github.com/ppodds/smli.git
cd smli

# run directly
cargo run <filename>

# compile and run (you only need smli(.exe))
cargo build -r
./target/release/smli <filename>
```

## Run Tests

```sh
cargo test
```
