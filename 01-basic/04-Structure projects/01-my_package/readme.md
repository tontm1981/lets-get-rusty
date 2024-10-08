# Basic Rust project template

## Components
Basically, a Rust project consists in:
- ***`Packages`***: allow us to build, test and share crates. Can  contain one or more crates that provides a set of functionality. It's compund by:
  - `Cargo.toml` file: Describes the packages and defines how to build crates

and must follow the following rules:
  - At least, 1 crate
  - At most, 1 **library crate**
  - Any number of binary crates

- ***`Crates`***: a tree of modules that produces a library or an executable
- ***`Modules`***: Let you control the organization, scope and privacy

When we run `cargo new` command, a package is created.

The `main.rs` file will be treated as binary root crate, and the file `lib.rs`, as the library root crate. In the case of many binary crates, there must to have a `bin` folder, containing the other bynary crates.