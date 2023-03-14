fn main() {
    let name = String::from("Dave");
    let language = "Rust".to_string();

    let new_name = name.replace("Dave", "David");

    println!("{}", name);
    println!("{}", language);
    println!("{}", new_name);

    // Create string slice
    let string_1 = "Hello";
    // Convert string slice to string
    let string_2 = string_1.to_string();
    // Convert string to string slice
    let string_3 = &string_2;

    // String comparison
    println!("{}", "ONE".to_lowercase() == "one");
}
