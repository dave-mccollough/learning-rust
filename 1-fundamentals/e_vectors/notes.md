# Vectors

- Vectors are a resizable array of items
- Used almost everywhere you need a list of dynamic size
- Using the Vector macro `vec!` is the same as calling a new Vector constructor `Vec::new();`
- If you already know the number of items that will be in your Vector, you can declare it
    - `let mut vect = Vect::i32::with_capacity(4);`

- https://doc.rust-lang.org/rust-by-example/std/vec.html
