# Functions

- Functions are defined using the `fn` keyword and is pronounced *fun*.
    `fn do_stuff() {`
    `}`

- Use `snake_case` for Rust function names.
    - Lowercase words seperated by underscores

- Function parameters are defined by `name: type`
    `fn do_stuff(qty: f32, weight: f32) {`
    `}`

- Function return types are specified by adding an arrow pointing to the return type
    `fn do_stuff(qty: f32, weight: f32) -> f32 {`
    `}`

- You can return values by using the `return` keyword 
    `fn do_stuff(qty: f32, weight: f32) -> f32 {`
        `return qty * weight;`
    `}`
    - Or leaving the `;` off the return statement
        `fn do_stuff(qty: f32, weight: f32) -> f32 {`
            `qty * weight`
        `}`
    - **This is the prefered method**

- To call a function in Rust
`fn main() -> f32 {`
    `let result = do_stuff(12.5, 13.7);`
`}`