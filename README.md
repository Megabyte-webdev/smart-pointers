# Smart Pointers

A small Rust project demonstrating common smart pointer types in the standard library: `Box`, `Rc`, and `RefCell`.

## Overview

This project is a hands-on example of how Rust manages ownership and shared data. It shows:

- `Box<T>` for heap allocation and recursive data structures
- `Rc<T>` for shared ownership of data with reference counting
- `RefCell<T>` for interior mutability when borrowing rules need to be enforced at runtime

## What the project demonstrates

The example in `src/main.rs` includes:

1. A linked-list-style recursive enum using `Box`
2. Reference counting with `Rc<String>`
3. Runtime-mutated counters using `RefCell<i32>`
4. Shared mutable vectors wrapped in `Rc<RefCell<Vec<String>>>`

## Project structure

```text
smart-pointers/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

## Running the project

From the project root, run:

```bash
cargo run
```

This will print examples of:

- recursive list traversal
- reference count tracking
- interior mutability updates
- shared, mutable data access

## Code sample

```rust
use std::{rc::Rc, cell::RefCell};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

let name = Rc::new(String::from("configmaster"));
let name2 = Rc::clone(&name);

let count = RefCell::new(0);
*count.borrow_mut() += 10;
```

## Learning goals

This project is useful for understanding:

- ownership and borrowing in Rust
- heap allocation with `Box`
- shared ownership with `Rc`
- mutability through `RefCell`
- how smart pointers enable patterns that are otherwise difficult with strict borrow rules

## Requirements

- Rust 2024 edition or a modern Rust toolchain
- Cargo

## License

This project is provided for educational purposes.
