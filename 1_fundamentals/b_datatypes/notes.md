# Datatypes

- Rust has 4 scalar datatypes
- Scalar types represent a single value

## Integers
- Integers can be:
    - 8 bit:  `i8`
    - 16 bit:  `i16`
    - 32 bit:  `i32`
    - 64 bit:  `i64`
    - 128 bit:  `i128`

- Integers can be signed or unsigned
    - Signed integers can be positive or negative
    - Unsigned integers won't have a plus or minus
        - Use unsigned integers if the value you want is always positive
            - 8 bit:  `u8`
            - 16 bit:  `u16`
            - 32 bit:  `u32`
            - 64 bit:  `u64`
            - 128 bit:  `u128`


## Floating point
- Floating point f64 is Default - better precision
- `let x = 2.0;`
- `let y: f32 = 1.0;`

## Boolean
- True or false values
- `let t = true`
    - Rust's compiler can infer the type
- `let f: bool = false`

## Characters
- `let c = "c";`
- `println!("{}", c);`

## Arthmatic Operators
- +
- -
- *
- /
- %

