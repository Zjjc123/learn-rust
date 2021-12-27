# Learn Rust

Repository for learning the programming language [Rust](https://www.rust-lang.org/)

## Ecosystem

### rustup

Toolchain Manager for managing Rust installation and build (installs rustc, cargo, rustup)

### cargo

Package manager for downloading and maging dependencies

### rustc

Rust compiler

## Installation

rustup (macOS)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Initialization

```
cargo init
```

### Subfolder

```
cargo new project_name
```

## Run

```
cargo run
```

### Build

```
cargo build
```

### Release

```
cargo build --release
```

# Highlights

## Types

-   Statically typed, must know type at compile time
-   Compiler can usually infer type from usage

## Variables

-   Default immutable
-   Make mutable with `mut`
-   Holds primitive data or references to data
-   Can shadow variables by declaring variables with the same name (can change type)

## Ownership

-   Each value in Rust has a variable that’s called its owner.
-   There can only be one owner at a time.
-   When the owner goes out of scope, the value will be dropped (`drop` is called).
-   Assigning a value to another variable moves ownership (no shallow copies).
    -   Only one pointer to a piece of data

## Modules

-   Rust treats files and subfiles as modules and sub-modules

-   Cannot reference files directly with simple imports
-   Need to create a tree of modules (similar to barrel export in javascript)
-   Use the `mod` keyword to include a module and `pub` keyword to make it visible

```
-- human.rs
    -- head.rs
    -- head
        -- eyes.rs
        -- nose.rs
        mouth.rs
    -- upperbody.rs
    -- upperbody
        -- chest.rs
        -- stomach.rs
    -- lowerbody.rs
    -- lowerbody
        -- legs.rs
        -- mod.rs
```

-   Modules can also be declared by a file `mod.rs` in a folder and the compiler will look in the `folder/mod.rs` file to use as contents for the module declaration of `folder`

```
-- human.rs
    -- head.rs
    -- head
        -- eyes.rs
        -- nose.rs
        -- mouth.rs
        -- mod.rs
    -- upperbody
        -- chest.rs
        -- stomach.rs
        -- mod.rs
    -- lowerbody
        -- legs.rs
        -- mod.rs
```
