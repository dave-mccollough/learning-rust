fn main() {
    let mut s = String::from("Hello");
    // add the & sign to change the reference
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(string1: &mut String) {
    string1.push_str(" World!");

}