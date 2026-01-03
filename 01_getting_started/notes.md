# Getting Started Notes

## Basic Commands

- Create a new project
  - `cargo new <project_name>`
- Compile Rust file
  - `rustc main.rs`
- Cargo Build
  - `cargo build` in project directory
  - Add `--release` flag for release build
    - `cargo build --release`
- Cargo Run
  - Run project
  - `cargo run` in project directory
- Cargo Check
  - `cargo check` checks source for compiler issues
  - Does not create an executable
- Cargo Clean
  - `cargo clean` removes build artifacts (like compiled binaries, intermediate files) from your project's target directory to free up space or start a fresh build.
- Cargo FMT
  - `cargo fmt` formats your code
