fn main() {
    let tup = (100, "char", 23_34, true);
    println!("{}", tup.0);

    // you can assign each value in a tuple to a different value
    let (w, x, y, z) = tup;

    println!("{}", w);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
