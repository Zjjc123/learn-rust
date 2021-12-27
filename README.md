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

-   statically typed, must know type at compile time
-   compiler can usually infer type from usage

## Variables

-   Default immutable
-   Make mutable with `mut`
-   holds primitive data or references to data
-   can shadow variables by declaring variables with the same name (can change type)

## Modules

-   rust treats files and subfiles as modules and sub-modules

-   cannot reference files directly with simple imports
-   need to create a tree of modules (similar to barrel export in javascript)
-   use the `mod` keyword to include a module and `pub` keyword to make it visibless

```
-- human.rs
    (mod head; mod upper_body; mod lower_body;)
    -- head.rs
        (mod eyes; mod nose; mod mouth;)
    -- head
        -- eyes.rs
        -- nose.rs
        mouth.rs
    -- upperbody.rs
        (mod chest; mod stomach;)
    -- upperbody
        -- chest.rs
        -- stomach.rs
    -- lowerbody.rs
```

-   or modules can be declared by a file `mod.rs` in a folder and the compiler will look in the `folder/mod.rs` file to use as contents for the module declaration of `folder`
