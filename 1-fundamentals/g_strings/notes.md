# Strings

- Strings are guaranteed to always be a valid UTF8 sequence
- Allocated on the heap
- Global
- Not null terminated

## Ways to create a string

- `let name = String::from("Dave");`
- `let language = "Rust".to_string();`

## String Replace
- `let new_name = name.replace("Dave", "David");`

## String slice
- `&str` known as string slice or 'stir'
- Reference and borrow the text from the variable
- Fat pointer containing the address, actual data and length
- Does not allocate memory on the heap
- You cannot modify a string slice
- To create a string slice
    - `let string_1 = "Hello";`

- String slice is more appropiate for function arguments


## Comparing strings

- Use `==` to compare if strings are equal
- use`!=` to compare if they are not equal




