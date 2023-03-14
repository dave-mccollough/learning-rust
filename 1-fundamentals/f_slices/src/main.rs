fn main () {
    let mut v: Vec<i32> = (0..5).collect();

    // Point to specific items in a vector

    // sv variable points to data in v variable
    // This is a non-owning reference (borrowing)
    let sv: &[i32] = &v[2..4];
    println!("{:?}", sv);
}