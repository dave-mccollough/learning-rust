fn main() {
    // Variable Creation
    // Use let keyword to create a variable
    // Rust will infer the type based on the value provided. In the example below, Rust inferred a 32 bit integer
    let a = 5;
    // We can explicitly set the type by adding colon (:) and the type
    let b: i16 = 6;

    // Mutating variables
    // In Rust variables are immutable by default
    // Reassigning the variable causes an error
    // to reassign a variable, add the mut keyword
    let mut c = 5;
    let c = 10;

    // Shadowing variables
    // Shadowing allows you to create two seperate variables but one of the variables is shadowing the other variable
    // Because the variables below have the same name and are in the same scope, the second variable will shadow the first variable
    let d = 5;
    let d = 10;
    println!("The value of d is: {d}");
    // The value of d is: 10

    // Scoping
    // Variables live within a scope
    // Scopes are defined by curly brackets
    // Variables defined in an inner scope cannot be accessed by the outer scope
    {
        let j = 10;
    }
    // Because j is defined in an inner scope, it can't be accessed by the outerscope
    println!("The value of j is: {j}");
}
