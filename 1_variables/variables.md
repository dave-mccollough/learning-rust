# Variables

- Variables are local to scope
- The scope of a variable begins where it is created and extends to the end of the block
    - Variables are dropped when they go out of scope
- Variables can shadowed
    `fn main() {`
        `let user_count = 1;`
        `{`
            `let user_count = 50`
            `println!("{}", user_count); //prints 50`
        `}`
        `println!("{}", user_count); //prints 1`
    `}`

- Variables must be initialized before they are used



- Variables can be defined using `let` or `const`

## `let`
- To declare a variable, use `let` 
    `fn main() {`
        `let user_count = 1;`
    `}`

- Rust infers variable type, but you can annotate the type.
    `fn main() {`
        `let user_count: i32 = 4;`
    `}`

- You can initialize multiple variables using the`let` statement
    `fn main() {`
        `let (user_count, admin_count) = (4,1);`
    `}`


- **Variables are immutable by default in Rust**

- To make a variable mutable, use `mut`
    `fn main() {`
        `let mut user_count = 4;`
        `user_count = 10;`
    `}`

## `const`

- `const` values can be scoped at the module level and can be used anywhere you want (global value)
- `const` variables can't be changed
- `const` variables should be defined using screaming snake case
- Type annotation is required for `const` variables
    `fn main() {`
        `const USER_COUNT: f32 = 4;`
    `}`




