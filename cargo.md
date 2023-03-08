# Cargo
- Cargo is Rust's build and package manager
- Use `--help` or `-h` if you need any help with Cargo's command-line utility:  `cargo -help`
- To create a new project with Cargo, use cargo new: `cargo new hello_world`
    - Use `lib` if you're creating a library.
    - This creates a new git repository by default. Pass `--vcs none` if you don't want one.
- `Cargo.toml` folder contains package metadata
- `main.rs` is the entry point of a Rust application
- Use `cargo build` to build the package
    - This generates a new folder called `target` with an executable
- Use `cargo run` to run the executable

## Resources
- [The Cargo Book](https://doc.rust-lang.org/cargo/)