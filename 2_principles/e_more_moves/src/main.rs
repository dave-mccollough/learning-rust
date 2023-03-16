fn main() {
    // Create a new variable
    let s = String::from("Takes");
    // Give ownership to the function
    takes_ownership(s);

    // Make a copy because i32 implements the copy trait
    let value = 1;
    make_copy(value);

    // Give ownership
    let str1: String = give_ownership();
    println!("{}", str1);

    // Take and give ownership
    let str3: String = take_and_give(str1);
}

fn takes_ownership(s: String) {
    let ownd_string = s;
    println!("{}", ownd_string);
}

fn make_copy(value: i32) {
    let copy_value = value;
    println!("{}", copy_value)
}

fn give_ownership() -> String {
    "Given".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}
