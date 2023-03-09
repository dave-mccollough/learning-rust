fn main() {
    let array = [1,2,3,4];
    println!("{}", array[0]);

    let array2: [i32; 3] = [1,2,3];
    println!("{}", array[2]);

    // To reassign a value in an array, the array must be made mutable
    let mut array3: [i32; 3] = [1,2,3];
    array3[2] = 45;
    println!("{}", array3[2]);
}
