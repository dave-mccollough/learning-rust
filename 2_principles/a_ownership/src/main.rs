fn main() {
    // var is valid for the remainder of the main function
    // 1 is a fixed size (u32 or i32)
    let var = 1;

    // s is created on the heap
    let mut s = "Hello".to_string();
    s.push_str(" World!");

}

//var is dropped
// s is dropped
