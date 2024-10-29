fn main() {
    // Boolean
    // True/False
    let b1: bool = true;

    // Unsigned Integers
    // Must be positive number
    // Unsigned integers start with the the letter u

    // 8 bit unsigned integer
    let i1: u8 = 1;

    // 16 bit unsigned integer
    let i2: u16 = 1;

    // 32 bit unsigned integer
    let i3: u32 = 1;

    // 64 bit unsigned integer
    let i4: u64 = 1;

    // 128 bit unsigned integer
    let i5: u128 = 1;

    // Signed Integers
    // Could be a positive or negative number
    // Signed integers start with i

    // 8 bit signed integer
    let i6: i8 = 1;

    // 16 bit signed integer
    let i7: i16 = 1;

    // 32 bit signed integer
    let i8: i32 = 1;

    // 64 bit signed integer
    let i9: i64 = 1;

    // 128 bit signed integer
    let i10: i128 = 1;

    // Floating Point Numbers
    // Number with decimal places
    // Start with the letter f
    let f1: f32 = 1.0;
    let f2: f64 = 2.0;

    // Platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // Char
    // Single character is represented with the char keyword
    let c1: char = 'c';

    // String types
    // String slices - all string literals are string slices
    // String slices start with ampersand str : &str
    let s1: &str = "Hello";
    // String type with uppercase S
    let s2: String = String::from("hello");

    //  Arrays
    // Fixed size array
    // Hold multiple values of the same type
    let a1: [i32; 5] = [1, 2, 3, 4, 5];

    // To get value from an array
    let aa1: i32 = a1[4];

    // Tuples
    // Hold multiple values of different types
    let t1 = (1, 2, "3", 4, "Five");
    // To get value from a tuple, use .notation
    let tt1 = t1.0;
    // To destructure a tuple use the following syntax
    let (d1, d2, d3, d4, d5) = t1;

    //  Empty tuples are called units
    // Functions that don't return a value, return the unit type
    let u1: () = ();

    //  Type Aliasing
    type Age = u8;
    let age1: Age = 57;
}
